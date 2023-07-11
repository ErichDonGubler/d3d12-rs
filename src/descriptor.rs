use windows::Win32::Graphics::{
    Direct3D12::{
        ID3D12DescriptorHeap, ID3D12RootSignature, D3D12_COMPARISON_FUNC,
        D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_DESCRIPTOR_HEAP_FLAGS, D3D12_DESCRIPTOR_HEAP_TYPE,
        D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV, D3D12_DESCRIPTOR_HEAP_TYPE_DSV,
        D3D12_DESCRIPTOR_HEAP_TYPE_RTV, D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER, D3D12_DESCRIPTOR_RANGE,
        D3D12_DESCRIPTOR_RANGE_TYPE, D3D12_DESCRIPTOR_RANGE_TYPE_CBV,
        D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER, D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
        D3D12_DESCRIPTOR_RANGE_TYPE_UAV, D3D12_FILTER, D3D12_GPU_DESCRIPTOR_HANDLE,
        D3D12_RENDER_TARGET_VIEW_DESC, D3D12_ROOT_CONSTANTS, D3D12_ROOT_DESCRIPTOR,
        D3D12_ROOT_DESCRIPTOR_TABLE, D3D12_ROOT_PARAMETER, D3D12_ROOT_PARAMETER_0,
        D3D12_ROOT_PARAMETER_TYPE, D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
        D3D12_ROOT_PARAMETER_TYPE_CBV, D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
        D3D12_ROOT_PARAMETER_TYPE_SRV, D3D12_ROOT_PARAMETER_TYPE_UAV, D3D12_ROOT_SIGNATURE_FLAGS,
        D3D12_RTV_DIMENSION_TEXTURE2D, D3D12_SHADER_VISIBILITY, D3D12_SHADER_VISIBILITY_ALL,
        D3D12_SHADER_VISIBILITY_DOMAIN, D3D12_SHADER_VISIBILITY_GEOMETRY,
        D3D12_SHADER_VISIBILITY_HULL, D3D12_SHADER_VISIBILITY_PIXEL,
        D3D12_SHADER_VISIBILITY_VERTEX, D3D12_STATIC_BORDER_COLOR,
        D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK, D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE,
        D3D12_STATIC_BORDER_COLOR_TRANSPARENT_BLACK, D3D12_STATIC_SAMPLER_DESC, D3D12_TEX2D_RTV,
        D3D_ROOT_SIGNATURE_VERSION, D3D_ROOT_SIGNATURE_VERSION_1_0, D3D_ROOT_SIGNATURE_VERSION_1_1,
    },
    Dxgi::Common::DXGI_FORMAT,
};

use crate::{Blob, D3DResult, Error, TextureAddressMode};
use std::{fmt, ops::Range};

pub type CpuDescriptor = D3D12_CPU_DESCRIPTOR_HANDLE;
pub type GpuDescriptor = D3D12_GPU_DESCRIPTOR_HANDLE;

#[derive(Clone, Copy, Debug)]
pub struct Binding {
    pub space: u32,
    pub register: u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum DescriptorHeapType {
    CbvSrvUav,
    Sampler,
    Rtv,
    Dsv,
}

impl From<DescriptorHeapType> for D3D12_DESCRIPTOR_HEAP_TYPE {
    fn from(value: DescriptorHeapType) -> Self {
        match value {
            DescriptorHeapType::CbvSrvUav => D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            DescriptorHeapType::Sampler => D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER,
            DescriptorHeapType::Rtv => D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
            DescriptorHeapType::Dsv => D3D12_DESCRIPTOR_HEAP_TYPE_DSV,
        }
    }
}

// NOTE: regressed `PartialOrd`, but y wud u do dis neway
pub type DescriptorHeapFlags = D3D12_DESCRIPTOR_HEAP_FLAGS;

#[derive(Clone)]
pub struct DescriptorHeap {
    pub(crate) inner: ID3D12DescriptorHeap,
}

impl DescriptorHeap {
    pub fn start_cpu_descriptor(&self) -> CpuDescriptor {
        unsafe { self.inner.GetCPUDescriptorHandleForHeapStart() }
    }

    pub fn start_gpu_descriptor(&self) -> GpuDescriptor {
        unsafe { self.inner.GetGPUDescriptorHandleForHeapStart() }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum ShaderVisibility {
    All,
    VS,
    DS,
    HS,
    GS,
    PS,
}

impl From<ShaderVisibility> for D3D12_SHADER_VISIBILITY {
    fn from(value: ShaderVisibility) -> Self {
        match value {
            ShaderVisibility::All => D3D12_SHADER_VISIBILITY_ALL,
            ShaderVisibility::VS => D3D12_SHADER_VISIBILITY_VERTEX,
            ShaderVisibility::DS => D3D12_SHADER_VISIBILITY_DOMAIN,
            ShaderVisibility::HS => D3D12_SHADER_VISIBILITY_HULL,
            ShaderVisibility::GS => D3D12_SHADER_VISIBILITY_GEOMETRY,
            ShaderVisibility::PS => D3D12_SHADER_VISIBILITY_PIXEL,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum DescriptorRangeType {
    SRV,
    UAV,
    CBV,
    Sampler,
}

impl From<DescriptorRangeType> for D3D12_DESCRIPTOR_RANGE_TYPE {
    fn from(value: DescriptorRangeType) -> Self {
        match value {
            DescriptorRangeType::SRV => D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
            DescriptorRangeType::UAV => D3D12_DESCRIPTOR_RANGE_TYPE_UAV,
            DescriptorRangeType::CBV => D3D12_DESCRIPTOR_RANGE_TYPE_CBV,
            DescriptorRangeType::Sampler => D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER,
        }
    }
}

#[repr(transparent)]
pub struct DescriptorRange(D3D12_DESCRIPTOR_RANGE);
impl DescriptorRange {
    pub fn new(ty: DescriptorRangeType, count: u32, base_binding: Binding, offset: u32) -> Self {
        DescriptorRange(D3D12_DESCRIPTOR_RANGE {
            RangeType: ty.into(),
            NumDescriptors: count,
            BaseShaderRegister: base_binding.register,
            RegisterSpace: base_binding.space,
            OffsetInDescriptorsFromTableStart: offset,
        })
    }
}

impl fmt::Debug for DescriptorRange {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_struct("DescriptorRange")
            .field("range_type", &self.0.RangeType)
            .field("num", &self.0.NumDescriptors)
            .field("register_space", &self.0.RegisterSpace)
            .field("base_register", &self.0.BaseShaderRegister)
            .field("table_offset", &self.0.OffsetInDescriptorsFromTableStart)
            .finish()
    }
}

// TODO: just use a strongly typed `enum` for the different variants, yo!
#[repr(transparent)]
pub struct RootParameter(D3D12_ROOT_PARAMETER);
impl RootParameter {
    // TODO: DescriptorRange must outlive Self
    pub fn descriptor_table(visibility: ShaderVisibility, ranges: &[DescriptorRange]) -> Self {
        Self(D3D12_ROOT_PARAMETER {
            ParameterType: D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
            ShaderVisibility: visibility.into(),
            Anonymous: D3D12_ROOT_PARAMETER_0 {
                DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE {
                    // FIXME: `u32` size, warning!
                    NumDescriptorRanges: ranges.len() as _,
                    pDescriptorRanges: ranges.as_ptr() as *const _,
                },
            },
        })
    }

    pub fn constants(visibility: ShaderVisibility, binding: Binding, num: u32) -> Self {
        Self(D3D12_ROOT_PARAMETER {
            ParameterType: D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
            ShaderVisibility: visibility.into(),
            Anonymous: D3D12_ROOT_PARAMETER_0 {
                Constants: D3D12_ROOT_CONSTANTS {
                    ShaderRegister: binding.register,
                    RegisterSpace: binding.space,
                    Num32BitValues: num,
                },
            },
        })
    }

    //TODO: should this be unsafe?
    pub fn descriptor(
        ty: D3D12_ROOT_PARAMETER_TYPE,
        visibility: ShaderVisibility,
        binding: Binding,
    ) -> Self {
        Self(D3D12_ROOT_PARAMETER {
            ParameterType: ty,
            ShaderVisibility: visibility.into(),
            Anonymous: D3D12_ROOT_PARAMETER_0 {
                Descriptor: D3D12_ROOT_DESCRIPTOR {
                    ShaderRegister: binding.register,
                    RegisterSpace: binding.space,
                },
            },
        })
    }

    pub fn cbv_descriptor(visibility: ShaderVisibility, binding: Binding) -> Self {
        Self::descriptor(D3D12_ROOT_PARAMETER_TYPE_CBV, visibility, binding)
    }

    pub fn srv_descriptor(visibility: ShaderVisibility, binding: Binding) -> Self {
        Self::descriptor(D3D12_ROOT_PARAMETER_TYPE_SRV, visibility, binding)
    }

    pub fn uav_descriptor(visibility: ShaderVisibility, binding: Binding) -> Self {
        Self::descriptor(D3D12_ROOT_PARAMETER_TYPE_UAV, visibility, binding)
    }
}

impl fmt::Debug for RootParameter {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        #[derive(Debug)]
        #[allow(dead_code)] // False-positive
        enum Inner<'a> {
            Table(&'a [DescriptorRange]),
            Constants { binding: Binding, num: u32 },
            SingleCbv(Binding),
            SingleSrv(Binding),
            SingleUav(Binding),
        }
        let kind = match self.0.ParameterType {
            D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE => unsafe {
                let raw = self.0.Anonymous.DescriptorTable;
                Inner::Table(std::slice::from_raw_parts(
                    raw.pDescriptorRanges as *const _,
                    raw.NumDescriptorRanges as usize,
                ))
            },
            D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS => unsafe {
                let raw = self.0.Anonymous.Constants;
                Inner::Constants {
                    binding: Binding {
                        space: raw.RegisterSpace,
                        register: raw.ShaderRegister,
                    },
                    num: raw.Num32BitValues,
                }
            },
            _ => unsafe {
                let raw = self.0.Anonymous.Descriptor;
                let binding = Binding {
                    space: raw.RegisterSpace,
                    register: raw.ShaderRegister,
                };
                match self.0.ParameterType {
                    D3D12_ROOT_PARAMETER_TYPE_CBV => Inner::SingleCbv(binding),
                    D3D12_ROOT_PARAMETER_TYPE_SRV => Inner::SingleSrv(binding),
                    D3D12_ROOT_PARAMETER_TYPE_UAV => Inner::SingleUav(binding),
                    other => panic!("Unexpected type {:?}", other),
                }
            },
        };

        formatter
            .debug_struct("RootParameter")
            .field("visibility", &self.0.ShaderVisibility)
            .field("kind", &kind)
            .finish()
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum StaticBorderColor {
    TransparentBlack,
    OpaqueBlack,
    OpaqueWhite,
}

impl From<StaticBorderColor> for D3D12_STATIC_BORDER_COLOR {
    fn from(value: StaticBorderColor) -> Self {
        match value {
            StaticBorderColor::TransparentBlack => D3D12_STATIC_BORDER_COLOR_TRANSPARENT_BLACK,
            StaticBorderColor::OpaqueBlack => D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK,
            StaticBorderColor::OpaqueWhite => D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE,
        }
    }
}

#[repr(transparent)]
pub struct StaticSampler(D3D12_STATIC_SAMPLER_DESC);
impl StaticSampler {
    pub fn new(
        visibility: ShaderVisibility,
        binding: Binding,
        filter: D3D12_FILTER,
        address_mode: TextureAddressMode,
        mip_lod_bias: f32,
        max_anisotropy: u32,
        comparison_op: D3D12_COMPARISON_FUNC,
        border_color: StaticBorderColor,
        lod: Range<f32>,
    ) -> Self {
        StaticSampler(D3D12_STATIC_SAMPLER_DESC {
            Filter: filter,
            AddressU: address_mode[0],
            AddressV: address_mode[1],
            AddressW: address_mode[2],
            MipLODBias: mip_lod_bias,
            MaxAnisotropy: max_anisotropy,
            ComparisonFunc: comparison_op,
            BorderColor: border_color.into(),
            MinLOD: lod.start,
            MaxLOD: lod.end,
            ShaderRegister: binding.register,
            RegisterSpace: binding.space,
            ShaderVisibility: visibility.into(),
        })
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum RootSignatureVersion {
    V1_0,
    V1_1,
}

impl From<RootSignatureVersion> for D3D_ROOT_SIGNATURE_VERSION {
    fn from(value: RootSignatureVersion) -> Self {
        match value {
            RootSignatureVersion::V1_0 => D3D_ROOT_SIGNATURE_VERSION_1_0,
            RootSignatureVersion::V1_1 => D3D_ROOT_SIGNATURE_VERSION_1_1,
        }
    }
}

// NOTE: regressed `PartialOrd`, but y wud u do dis neway
pub type RootSignatureFlags = D3D12_ROOT_SIGNATURE_FLAGS;

pub struct RootSignature {
    pub(crate) inner: ID3D12RootSignature,
}
pub type BlobResult = D3DResult<(Blob, Error)>;

#[cfg(feature = "libloading")]
impl crate::D3D12Lib {
    pub fn serialize_root_signature(
        &self,
        version: RootSignatureVersion,
        parameters: &[RootParameter],
        static_samplers: &[StaticSampler],
        flags: RootSignatureFlags,
    ) -> Result<BlobResult, libloading::Error> {
        let desc = D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: parameters.len() as _,
            pParameters: parameters.as_ptr() as *const _,
            NumStaticSamplers: static_samplers.len() as _,
            pStaticSamplers: static_samplers.as_ptr() as _,
            Flags: flags.bits(),
        };

        let mut blob = Blob::null();
        let mut error = Error::null();
        let hr = unsafe {
            let func: libloading::Symbol<Fun> = self.lib.get(b"D3D12SerializeRootSignature")?;
            func(
                &desc,
                version as _,
                blob.mut_void() as *mut *mut _,
                error.mut_void() as *mut *mut _,
            )
        };

        Ok(((blob, error), hr))
    }
}

impl RootSignature {
    #[cfg(feature = "implicit-link")]
    pub fn serialize(
        version: RootSignatureVersion,
        parameters: &[RootParameter],
        static_samplers: &[StaticSampler],
        flags: RootSignatureFlags,
    ) -> BlobResult {
        let mut blob = None;
        let mut error = None;

        // FIXME: warning, slice length to `u32`!
        let desc = D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: parameters.len() as _,
            pParameters: parameters.as_ptr() as *const _,
            NumStaticSamplers: static_samplers.len() as _,
            pStaticSamplers: static_samplers.as_ptr() as _,
            Flags: flags.bits(),
        };

        let hr = unsafe {
            windows::Win32::Graphics::Direct3D12::D3D12SerializeRootSignature(
                &desc,
                version as _,
                blob.mut_void() as *mut *mut _,
                error.mut_void() as *mut *mut _,
            )
        };

        ((blob, error), hr)
    }
}

#[repr(transparent)]
pub struct RenderTargetViewDesc(pub(crate) D3D12_RENDER_TARGET_VIEW_DESC);

impl RenderTargetViewDesc {
    // TODO: Don't we already have a `Format` symbol covering this, FFI-wise? :think:
    pub fn texture_2d(format: DXGI_FORMAT, mip_slice: u32, plane_slice: u32) -> Self {
        RenderTargetViewDesc(D3D12_RENDER_TARGET_VIEW_DESC {
            Format: format,
            ViewDimension: D3D12_RTV_DIMENSION_TEXTURE2D,
            Anonymous: windows::Win32::Graphics::Direct3D12::D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture2D: D3D12_TEX2D_RTV {
                    MipSlice: mip_slice,
                    PlaneSlice: plane_slice,
                },
            },
        })
    }
}
