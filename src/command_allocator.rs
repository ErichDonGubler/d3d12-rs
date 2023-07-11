//! Command Allocator

use windows::Win32::Graphics::Direct3D12::ID3D12CommandAllocator;

use crate::D3DResult;

pub struct CommandAllocator {
    pub(crate) inner: ID3D12CommandAllocator,
}

impl CommandAllocator {
    pub fn reset(&self) -> D3DResult<()> {
        unsafe { self.inner.Reset() }
    }
}
