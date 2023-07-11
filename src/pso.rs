//! Pipeline state

use windows::{
    core::PCSTR,
    Win32::Graphics::{
        Direct3D::Fxc::{
            D3DCompile, D3DCOMPILE_DEBUG, D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR,
            D3DCOMPILE_PACK_MATRIX_ROW_MAJOR, D3DCOMPILE_PARTIAL_PRECISION,
            D3DCOMPILE_SKIP_OPTIMIZATION, D3DCOMPILE_SKIP_VALIDATION,
        },
        Direct3D12::{
            ID3D12PipelineState, D3D12_CACHED_PIPELINE_STATE, D3D12_PIPELINE_STATE_FLAGS,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_BLEND,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CACHED_PSO, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CS,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL1,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL_FORMAT,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DS, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_FLAGS,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_GS, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_HS,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_IB_STRIP_CUT_VALUE,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_INPUT_LAYOUT,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_NODE_MASK,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PRIMITIVE_TOPOLOGY,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PS, D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RENDER_TARGET_FORMATS,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_ROOT_SIGNATURE,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_DESC,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_MASK,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_STREAM_OUTPUT,
            D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VS, D3D12_SHADER_BYTECODE,
        },
    },
};

use crate::{Blob, D3DResult, Error};
use std::{ffi, ops::Deref, ptr};

// NOTE: regressed `PartialOrd`, but y wud u do dis neway
pub type PipelineStateFlags = D3D12_PIPELINE_STATE_FLAGS;

// // NOTE: regressed `PartialOrd`, but y wud u do dis neway
bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct ShaderCompileFlags: u32 {
        const DEBUG = D3DCOMPILE_DEBUG;
        const SKIP_VALIDATION = D3DCOMPILE_SKIP_VALIDATION;
        const SKIP_OPTIMIZATION = D3DCOMPILE_SKIP_OPTIMIZATION;
        const PACK_MATRIX_ROW_MAJOR = D3DCOMPILE_PACK_MATRIX_ROW_MAJOR;
        const PACK_MATRIX_COLUMN_MAJOR = D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR;
        const PARTIAL_PRECISION = D3DCOMPILE_PARTIAL_PRECISION;
        // TODO: add missing flags
    }
}

#[derive(Copy, Clone)]
pub struct Shader(D3D12_SHADER_BYTECODE);
impl Shader {
    pub fn null() -> Self {
        Shader(D3D12_SHADER_BYTECODE {
            BytecodeLength: 0,
            pShaderBytecode: ptr::null(),
        }, PhantomData)
    }

    pub fn from_raw(data: &'a [u8]) -> Self {
        Shader(D3D12_SHADER_BYTECODE {
            BytecodeLength: data.len() as _,
            pShaderBytecode: data.as_ptr() as _,
        }, PhantomData)
    }

    // `blob` may not be null.
    pub fn from_blob(blob: &'a Blob) -> Self {
        Shader(D3D12_SHADER_BYTECODE {
            BytecodeLength: unsafe { blob.GetBufferSize() },
            pShaderBytecode: unsafe { blob.GetBufferPointer() },
        }, PhantomData)
    }

    /// Compile a shader from raw HLSL.
    ///
    /// * `target`: example format: `ps_5_1`.
    pub fn compile(
        code: &[u8],
        target: &ffi::CStr,
        entry: &ffi::CStr,
        flags: ShaderCompileFlags,
    ) -> D3DResult<(Blob, Error)> {
        let mut shader = None;
        let mut error = None;

        unsafe {
            D3DCompile(
                // FIXME: warning, `usize` to `u32`!
                code.as_ptr() as *const _,
                code.len(),
                None,
                None, // defines
                None, // include
                PCSTR::from_raw(entry.as_ptr().cast()),
                PCSTR::from_raw(target.as_ptr().cast()),
                flags.bits(),
                0,
                &mut shader,
                Some(&mut error),
            )
        }
        .map(|()| (shader.unwrap(), error.unwrap()))
    }
}

impl<'a> Deref for Shader<'a> {
    type Target = D3D12_SHADER_BYTECODE;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone)]
pub struct CachedPSO(D3D12_CACHED_PIPELINE_STATE);
impl CachedPSO {
    pub fn null() -> Self {
        CachedPSO(D3D12_CACHED_PIPELINE_STATE {
            CachedBlobSizeInBytes: 0,
            pCachedBlob: ptr::null(),
        }, PhantomData)
    }

    // `blob` may not be null.
    pub fn from_blob(blob: &'a Blob) -> Self {
        CachedPSO(D3D12_CACHED_PIPELINE_STATE {
            CachedBlobSizeInBytes: unsafe { blob.GetBufferSize() },
            pCachedBlob: unsafe { blob.GetBufferPointer() },
        }, PhantomData)
    }
}

impl<'a> Deref for CachedPSO<'a> {
    type Target = D3D12_CACHED_PIPELINE_STATE;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct PipelineState {
    pub(crate) inner: ID3D12PipelineState,
}

#[repr(u32)]
pub enum Subobject {
    RootSignature,
    VS,
    PS,
    DS,
    HS,
    GS,
    CS,
    StreamOutput,
    Blend,
    SampleMask,
    Rasterizer,
    DepthStencil,
    InputLayout,
    IBStripCut,
    PrimitiveTopology,
    RTFormats,
    DSFormat,
    SampleDesc,
    NodeMask,
    CachedPSO,
    Flags,
    DepthStencil1,
    // ViewInstancing,
}

impl From<Subobject> for D3D12_PIPELINE_STATE_SUBOBJECT_TYPE {
    fn from(value: Subobject) -> Self {
        match value {
            Subobject::RootSignature => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_ROOT_SIGNATURE,
            Subobject::VS => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VS,
            Subobject::PS => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PS,
            Subobject::DS => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DS,
            Subobject::HS => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_HS,
            Subobject::GS => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_GS,
            Subobject::CS => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CS,
            Subobject::StreamOutput => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_STREAM_OUTPUT,
            Subobject::Blend => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_BLEND,
            Subobject::SampleMask => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_MASK,
            Subobject::Rasterizer => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER,
            Subobject::DepthStencil => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL,
            Subobject::InputLayout => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_INPUT_LAYOUT,
            Subobject::IBStripCut => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_IB_STRIP_CUT_VALUE,
            Subobject::PrimitiveTopology => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PRIMITIVE_TOPOLOGY,
            Subobject::RTFormats => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RENDER_TARGET_FORMATS,
            Subobject::DSFormat => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL_FORMAT,
            Subobject::SampleDesc => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_DESC,
            Subobject::NodeMask => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_NODE_MASK,
            Subobject::CachedPSO => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CACHED_PSO,
            Subobject::Flags => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_FLAGS,
            Subobject::DepthStencil1 => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL1,
            // Subobject::ViewInstancing => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VIEW_INSTANCING,
        }
    }
}

/// Subobject of a pipeline stream description
#[repr(C)]
pub struct PipelineStateSubobject<T> {
    subobject_align: [usize; 0], // Subobjects must have the same alignment as pointers.
    subobject_type: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE,
    subobject: T,
}

impl<T> PipelineStateSubobject<T> {
    pub fn new(subobject_type: Subobject, subobject: T) -> Self {
        PipelineStateSubobject {
            subobject_align: [],
            subobject_type: subobject_type.into(),
            subobject,
        }
    }
}
