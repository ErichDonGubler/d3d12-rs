//! Command Allocator

use windows::Win32::Graphics::Direct3D12::ID3D12CommandAllocator;

pub struct CommandAllocator {
    pub(crate) inner: ID3D12CommandAllocator,
}

impl CommandAllocator {
    pub fn reset(&self) {
        unsafe {
            self.inner.Reset();
        }
    }
}
