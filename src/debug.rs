#[cfg(any(feature = "libloading", feature = "implicit-link"))]
use windows::core::Interface as _;
use windows::Win32::Graphics::Direct3D12::{D3D12GetDebugInterface, ID3D12Debug};

pub struct Debug {
    inner: ID3D12Debug,
}

#[cfg(feature = "libloading")]
impl crate::D3D12Lib {
    pub fn get_debug_interface(&self) -> Result<crate::D3DResult<Debug>, libloading::Error> {
        let mut debug = Debug::null();
        let hr = unsafe { D3D12GetDebugInterface(Some(&mut debug)) };
        Ok((debug, hr))
    }
}

impl Debug {
    #[cfg(feature = "implicit-link")]
    pub fn get_interface() -> crate::D3DResult<Self> {
        let mut debug = Debug::null();
        let hr = unsafe { D3D12GetDebugInterface(Some(&mut debug)) };
        Ok((debug, hr))
    }

    pub fn enable_layer(&self) {
        unsafe { self.inner.EnableDebugLayer() }
    }
}
