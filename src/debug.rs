use windows::Win32::Graphics::Direct3D12::ID3D12Debug;

pub struct Debug {
    inner: ID3D12Debug,
}

#[cfg(feature = "libloading")]
impl crate::D3D12Lib {
    pub fn get_debug_interface(&self) -> Result<crate::D3DResult<Debug>, libloading::Error> {
        Ok(Debug::new())
    }
}

impl Debug {
    #[cfg(any(feature = "libloading", feature = "implicit-link"))]
    fn new() -> crate::D3DResult<Self> {
        use windows::Win32::Graphics::Direct3D12::D3D12GetDebugInterface;

        let mut inner = None;
        unsafe { D3D12GetDebugInterface(&mut inner) }?;
        Ok(Debug {
            inner: inner.unwrap(),
        })
    }

    #[cfg(feature = "implicit-link")]
    pub fn get_interface() -> crate::D3DResult<Self> {
        Self::new()
    }

    pub fn enable_layer(&self) {
        unsafe { self.inner.EnableDebugLayer() }
    }
}
