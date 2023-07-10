#[macro_use]
extern crate bitflags;

use std::convert::TryFrom;

mod command_allocator;
mod command_list;
mod debug;
mod descriptor;
mod device;
mod dxgi;
mod heap;
mod pso;
mod query;
mod queue;
mod resource;
mod sync;

pub use crate::command_allocator::*;
pub use crate::command_list::*;
pub use crate::debug::*;
pub use crate::descriptor::*;
pub use crate::device::*;
pub use crate::dxgi::*;
pub use crate::heap::*;
pub use crate::pso::*;
pub use crate::query::*;
pub use crate::queue::*;
pub use crate::resource::*;
pub use crate::sync::*;

pub use windows::core::HRESULT;
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL;
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_10_0;
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_10_1;
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_11_0;
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_11_1;
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_12_0;
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_12_1;
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_9_1;
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_9_2;
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_9_3;

// TODO: overhaul allathese
pub type D3DResult<T> = windows::core::Result<T>;
pub type GpuAddress = u64;
pub type Format = windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT;
pub type Rect = windows::Win32::Foundation::RECT;
pub type NodeMask = u32;

/// Index into the root signature.
pub type RootIndex = u32;
/// Draw vertex count.
pub type VertexCount = u32;
/// Draw vertex base offset.
pub type VertexOffset = i32;
/// Draw number of indices.
pub type IndexCount = u32;
/// Draw number of instances.
pub type InstanceCount = u32;
/// Number of work groups.
pub type WorkGroupCount = [u32; 3];

pub type TextureAddressMode = [windows::Win32::Graphics::Direct3D12::D3D12_TEXTURE_ADDRESS_MODE; 3];

pub struct SampleDesc {
    pub count: u32,
    pub quality: u32,
}

#[repr(u32)]
#[non_exhaustive]
pub enum FeatureLevel {
    L9_1,
    L9_2,
    L9_3,
    L10_0,
    L10_1,
    L11_0,
    L11_1,
    L12_0,
    L12_1,
}

impl From<FeatureLevel> for D3D_FEATURE_LEVEL {
    fn from(value: FeatureLevel) -> Self {
        match value {
            FeatureLevel::L9_1 => D3D_FEATURE_LEVEL_9_1,
            FeatureLevel::L9_2 => D3D_FEATURE_LEVEL_9_2,
            FeatureLevel::L9_3 => D3D_FEATURE_LEVEL_9_3,
            FeatureLevel::L10_0 => D3D_FEATURE_LEVEL_10_0,
            FeatureLevel::L10_1 => D3D_FEATURE_LEVEL_10_1,
            FeatureLevel::L11_0 => D3D_FEATURE_LEVEL_11_0,
            FeatureLevel::L11_1 => D3D_FEATURE_LEVEL_11_1,
            FeatureLevel::L12_0 => D3D_FEATURE_LEVEL_12_0,
            FeatureLevel::L12_1 => D3D_FEATURE_LEVEL_12_1,
        }
    }
}

impl TryFrom<D3D_FEATURE_LEVEL> for FeatureLevel {
    type Error = ();

    fn try_from(value: D3D_FEATURE_LEVEL) -> Result<Self, Self::Error> {
        Ok(match value {
            D3D_FEATURE_LEVEL_9_1 => Self::L9_1,
            D3D_FEATURE_LEVEL_9_2 => Self::L9_2,
            D3D_FEATURE_LEVEL_9_3 => Self::L9_3,
            D3D_FEATURE_LEVEL_10_0 => Self::L10_0,
            D3D_FEATURE_LEVEL_10_1 => Self::L10_1,
            D3D_FEATURE_LEVEL_11_0 => Self::L11_0,
            D3D_FEATURE_LEVEL_11_1 => Self::L11_1,
            D3D_FEATURE_LEVEL_12_0 => Self::L12_0,
            D3D_FEATURE_LEVEL_12_1 => Self::L12_1,
            _ => return Err(()),
        })
    }
}

// TODO: unused?
pub type Blob = windows::Win32::Graphics::Direct3D::ID3DBlob;

pub type Error = windows::Win32::Graphics::Direct3D::ID3DBlob;
// impl Error {
//     pub unsafe fn as_c_str(&self) -> &CStr {
//         debug_assert!(!self.is_null());
//         let data = self.GetBufferPointer();
//         CStr::from_ptr(data as *const _ as *const _)
//     }
// }

// TODO: remove this?
#[cfg(feature = "libloading")]
#[derive(Debug)]
pub struct D3D12Lib {
    _disable_ctor: (),
}

#[cfg(feature = "libloading")]
impl D3D12Lib {
    pub fn new() -> Result<Self, libloading::Error> {
        Self { _disable_ctor: () }
    }
}
