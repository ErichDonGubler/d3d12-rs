use windows::{
    core::IUnknown,
    Win32::{
        Foundation::{HANDLE, HWND, S_OK, TRUE},
        Graphics::Dxgi::{
            Common::{
                DXGI_ALPHA_MODE, DXGI_ALPHA_MODE_FORCE_DWORD, DXGI_ALPHA_MODE_IGNORE,
                DXGI_ALPHA_MODE_PREMULTIPLIED, DXGI_ALPHA_MODE_STRAIGHT,
                DXGI_ALPHA_MODE_UNSPECIFIED, DXGI_FORMAT, DXGI_MODE_DESC,
                DXGI_MODE_SCALING_UNSPECIFIED, DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED, DXGI_RATIONAL,
                DXGI_SAMPLE_DESC,
            },
            IDXGIAdapter1, IDXGIAdapter2, IDXGIAdapter3, IDXGIAdapter4, IDXGIFactory1,
            IDXGIFactory2, IDXGIFactory3, IDXGIFactory4, IDXGIFactory5, IDXGIFactory6,
            IDXGIFactoryMedia, IDXGIInfoQueue, IDXGISwapChain, IDXGISwapChain1, IDXGISwapChain2,
            IDXGISwapChain3, DXGI_CREATE_FACTORY_DEBUG, DXGI_PRESENT_ALLOW_TEARING,
            DXGI_PRESENT_DO_NOT_SEQUENCE, DXGI_PRESENT_DO_NOT_WAIT, DXGI_PRESENT_RESTART,
            DXGI_PRESENT_RESTRICT_TO_OUTPUT, DXGI_PRESENT_STEREO_PREFER_RIGHT,
            DXGI_PRESENT_STEREO_TEMPORARY_MONO, DXGI_PRESENT_TEST, DXGI_PRESENT_USE_DURATION,
            DXGI_SCALING, DXGI_SCALING_ASPECT_RATIO_STRETCH, DXGI_SCALING_NONE,
            DXGI_SCALING_STRETCH, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_EFFECT,
            DXGI_SWAP_EFFECT_DISCARD, DXGI_SWAP_EFFECT_FLIP_DISCARD,
            DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL, DXGI_SWAP_EFFECT_SEQUENTIAL, DXGI_USAGE,
        },
    },
};

use crate::{D3DResult, Resource, SampleDesc, HRESULT};

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct FactoryCreationFlags: u32 {
        const DEBUG = DXGI_CREATE_FACTORY_DEBUG;
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Scaling {
    Stretch,
    Identity,
    Aspect,
}

impl From<Scaling> for DXGI_SCALING {
    fn from(value: Scaling) -> Self {
        match value {
            Scaling::Stretch => DXGI_SCALING_STRETCH,
            Scaling::Identity => DXGI_SCALING_NONE,
            Scaling::Aspect => DXGI_SCALING_ASPECT_RATIO_STRETCH,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum SwapEffect {
    Discard,
    Sequential,
    FlipDiscard,
    FlipSequential,
}

impl From<SwapEffect> for DXGI_SWAP_EFFECT {
    fn from(value: SwapEffect) -> DXGI_SWAP_EFFECT {
        match value {
            SwapEffect::Discard => DXGI_SWAP_EFFECT_DISCARD,
            SwapEffect::Sequential => DXGI_SWAP_EFFECT_SEQUENTIAL,
            SwapEffect::FlipDiscard => DXGI_SWAP_EFFECT_FLIP_DISCARD,
            SwapEffect::FlipSequential => DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AlphaMode {
    Unspecified,
    Premultiplied,
    Straight,
    Ignore,
    ForceDword,
}

impl From<AlphaMode> for DXGI_ALPHA_MODE {
    fn from(value: AlphaMode) -> DXGI_ALPHA_MODE {
        match value {
            AlphaMode::Unspecified => DXGI_ALPHA_MODE_UNSPECIFIED,
            AlphaMode::Premultiplied => DXGI_ALPHA_MODE_PREMULTIPLIED,
            AlphaMode::Straight => DXGI_ALPHA_MODE_STRAIGHT,
            AlphaMode::Ignore => DXGI_ALPHA_MODE_IGNORE,
            AlphaMode::ForceDword => DXGI_ALPHA_MODE_FORCE_DWORD,
        }
    }
}

pub struct InfoQueue {
    #[allow(dead_code)] // TODO: remove this
    inner: IDXGIInfoQueue,
}

pub struct Adapter1 {
    #[allow(dead_code)] // TODO: remove this
    inner: IDXGIAdapter1,
}
pub struct Adapter2 {
    #[allow(dead_code)] // TODO: remove this
    inner: IDXGIAdapter2,
}
pub struct Adapter3 {
    #[allow(dead_code)] // TODO: remove this
    inner: IDXGIAdapter3,
}
pub struct Adapter4 {
    #[allow(dead_code)] // TODO: remove this
    inner: IDXGIAdapter4,
}
// crate::weak_com_inheritance_chain! {
//     #[derive(Debug, Copy, Clone, PartialEq, Hash)]
//     pub enum DxgiAdapter {
//         Adapter1(IDXGIAdapter1), from_adapter1, as_adapter1, adapter1;
//         Adapter2(IDXGIAdapter2), from_adapter2, as_adapter2, unwrap_adapter2;
//         Adapter3(IDXGIAdapter3), from_adapter3, as_adapter3, unwrap_adapter3;
//         Adapter4(IDXGIAdapter4), from_adapter4, as_adapter4, unwrap_adapter4;
//     }
// }

pub struct Factory1 {
    #[allow(dead_code)] // TODO: remove this
    inner: IDXGIFactory1,
}
pub struct Factory2 {
    #[allow(dead_code)] // TODO: remove this
    inner: IDXGIFactory2,
}
pub struct Factory3 {
    #[allow(dead_code)] // TODO: remove this
    inner: IDXGIFactory3,
}
pub struct Factory4 {
    #[allow(dead_code)] // TODO: remove this
    inner: IDXGIFactory4,
}
pub struct Factory5 {
    #[allow(dead_code)] // TODO: remove this
    inner: IDXGIFactory5,
}
pub struct Factory6 {
    #[allow(dead_code)] // TODO: remove this
    inner: IDXGIFactory6,
}
// crate::weak_com_inheritance_chain! {
//     #[derive(Debug, Copy, Clone, PartialEq, Hash)]
//     pub enum DxgiFactory {
//         Factory1(IDXGIFactory1), from_factory1, as_factory1, factory1;
//         Factory2(IDXGIFactory2), from_factory2, as_factory2, unwrap_factory2;
//         Factory3(IDXGIFactory3), from_factory3, as_factory3, unwrap_factory3;
//         Factory4(IDXGIFactory4), from_factory4, as_factory4, unwrap_factory4;
//         Factory5(IDXGIFactory5), from_factory5, as_factory5, unwrap_factory5;
//         Factory6(IDXGIFactory6), from_factory6, as_factory6, unwrap_factory6;
//     }
// }

pub struct FactoryMedia {
    #[allow(dead_code)] // TODO: remove this
    inner: IDXGIFactoryMedia,
}

pub struct SwapChain {
    #[allow(dead_code)] // TODO: remove this
    inner: IDXGISwapChain,
}
pub struct SwapChain1 {
    #[allow(dead_code)] // TODO: remove this
    inner: IDXGISwapChain1,
}
pub struct SwapChain2 {
    #[allow(dead_code)] // TODO: remove this
    inner: IDXGISwapChain2,
}
pub struct SwapChain3 {
    #[allow(dead_code)] // TODO: remove this
    inner: IDXGISwapChain3,
}
// crate::weak_com_inheritance_chain! {
//     #[derive(Debug, Copy, Clone, PartialEq, Hash)]
//     pub enum DxgiSwapchain {
//         SwapChain(IDXGISwapChain), from_swap_chain, as_swap_chain, swap_chain;
//         SwapChain1(IDXGISwapChain1), from_swap_chain1, as_swap_chain1, unwrap_swap_chain1;
//         SwapChain2(IDXGISwapChain2), from_swap_chain2, as_swap_chain2, unwrap_swap_chain2;
//         SwapChain3(IDXGISwapChain3), from_swap_chain3, as_swap_chain3, unwrap_swap_chain3;
//     }
// }

#[cfg(feature = "libloading")]
#[derive(Debug)]
pub struct DxgiLib {
    _disable_ctor: (),
}

#[cfg(feature = "libloading")]
impl DxgiLib {
    pub fn new() -> Result<Self, libloading::Error> {
        Ok(Self { _disable_ctor: () })
    }

    pub fn create_factory2(
        &self,
        flags: FactoryCreationFlags,
    ) -> Result<D3DResult<Factory4>, libloading::Error> {
        Ok(unsafe { CreateDXGIFactory2(flags.bits()) })
    }

    pub fn create_factory1(&self) -> Result<D3DResult<Factory1>, libloading::Error> {
        Ok(unsafe { CreateDXGIFactory1() })
    }

    pub fn create_factory_media(&self) -> Result<D3DResult<FactoryMedia>, libloading::Error> {
        Ok(unsafe { CreateDXGIFactory1() })
    }

    pub fn get_debug_interface1(&self) -> Result<D3DResult<InfoQueue>, libloading::Error> {
        Ok(unsafe { DXGIGetDebugInterface1(0) })
    }
}

// TODO: strong types
pub struct SwapchainDesc {
    pub width: u32,
    pub height: u32,
    pub format: DXGI_FORMAT,
    pub stereo: bool,
    pub sample: SampleDesc,
    pub buffer_usage: DXGI_USAGE,
    pub buffer_count: u32,
    pub scaling: Scaling,
    pub swap_effect: SwapEffect,
    pub alpha_mode: AlphaMode,
    pub flags: u32,
}
impl SwapchainDesc {
    pub fn to_desc1(&self) -> DXGI_SWAP_CHAIN_DESC1 {
        DXGI_SWAP_CHAIN_DESC1 {
            AlphaMode: self.alpha_mode.into(),
            BufferCount: self.buffer_count,
            Width: self.width,
            Height: self.height,
            Format: self.format,
            Flags: self.flags,
            BufferUsage: self.buffer_usage,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: self.sample.count,
                Quality: self.sample.quality,
            },
            Scaling: self.scaling.into(),
            Stereo: self.stereo.into(),
            SwapEffect: self.swap_effect.into(),
        }
    }
}

impl Factory1 {
    pub fn create_swapchain(
        &self,
        queue: &IUnknown,
        hwnd: HWND,
        desc: &SwapchainDesc,
    ) -> D3DResult<SwapChain> {
        let mut desc = DXGI_SWAP_CHAIN_DESC {
            BufferDesc: DXGI_MODE_DESC {
                Width: desc.width,
                Height: desc.width,
                RefreshRate: DXGI_RATIONAL {
                    Numerator: 1,
                    Denominator: 60,
                },
                Format: desc.format,
                ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
            },
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: desc.sample.count,
                Quality: desc.sample.quality,
            },
            BufferUsage: desc.buffer_usage,
            BufferCount: desc.buffer_count,
            OutputWindow: hwnd,
            Windowed: TRUE,
            SwapEffect: desc.swap_effect.into(),
            Flags: desc.flags,
        };

        let mut swapchain = None;
        let hr = unsafe { self.inner.CreateSwapChain(queue, &mut desc, &mut swapchain) };
        if hr == S_OK {
            Ok(SwapChain {
                inner: swapchain.unwrap(),
            })
        } else {
            Err(windows::core::Error::from(hr))
        }
    }
}

impl Factory2 {
    // TODO: interface not complete
    pub fn create_swapchain_for_hwnd(
        &self,
        queue: &IUnknown,
        hwnd: HWND,
        desc: &SwapchainDesc,
    ) -> D3DResult<SwapChain1> {
        Ok(SwapChain1 {
            inner: unsafe {
                self.inner
                    .CreateSwapChainForHwnd(queue, hwnd, &desc.to_desc1(), None, None)
            }?,
        })
    }

    pub fn create_swapchain_for_composition(
        &self,
        queue: &IUnknown,
        desc: &SwapchainDesc,
    ) -> D3DResult<SwapChain1> {
        let inner = unsafe {
            self.inner
                .CreateSwapChainForComposition(queue, &desc.to_desc1(), None)
        }?;
        Ok(SwapChain1 { inner })
    }
}

impl Factory4 {
    #[cfg(feature = "implicit-link")]
    pub fn create(flags: FactoryCreationFlags) -> D3DResult<Self> {
        use windows::Win32::Graphics::Dxgi::CreateDXGIFactory2;

        let inner = unsafe { CreateDXGIFactory2(flags.into()) };

        Ok(Self { inner })
    }

    pub fn enumerate_adapters(&self, id: u32) -> D3DResult<Adapter1> {
        Ok(Adapter1 {
            inner: unsafe { self.inner.EnumAdapters1(id) }?,
        })
    }
}

impl FactoryMedia {
    pub fn create_swapchain_for_composition_surface_handle(
        &self,
        queue: &IUnknown,
        surface_handle: HANDLE,
        desc: &SwapchainDesc,
    ) -> D3DResult<SwapChain1> {
        Ok(SwapChain1 {
            inner: unsafe {
                self.inner.CreateSwapChainForCompositionSurfaceHandle(
                    queue,
                    surface_handle,
                    &desc.to_desc1(),
                    None,
                )
            }?,
        })
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct SwapChainPresentFlags: u32 {
        const DXGI_PRESENT_DO_NOT_SEQUENCE = DXGI_PRESENT_DO_NOT_SEQUENCE;
        const DXGI_PRESENT_TEST = DXGI_PRESENT_TEST;
        const DXGI_PRESENT_RESTART = DXGI_PRESENT_RESTART;
        const DXGI_PRESENT_DO_NOT_WAIT = DXGI_PRESENT_DO_NOT_WAIT;
        const DXGI_PRESENT_RESTRICT_TO_OUTPUT = DXGI_PRESENT_RESTRICT_TO_OUTPUT;
        const DXGI_PRESENT_STEREO_PREFER_RIGHT = DXGI_PRESENT_STEREO_PREFER_RIGHT;
        const DXGI_PRESENT_STEREO_TEMPORARY_MONO = DXGI_PRESENT_STEREO_TEMPORARY_MONO;
        const DXGI_PRESENT_USE_DURATION = DXGI_PRESENT_USE_DURATION;
        const DXGI_PRESENT_ALLOW_TEARING = DXGI_PRESENT_ALLOW_TEARING;
    }
}

impl SwapChain {
    pub fn get_buffer(&self, id: u32) -> D3DResult<Resource> {
        Ok(Resource {
            inner: unsafe { self.inner.GetBuffer(id) }?,
        })
    }

    //TODO: replace by present_flags
    pub fn present(&self, interval: u32, flags: u32) -> HRESULT {
        unsafe { self.inner.Present(interval, flags) }
    }

    pub fn present_flags(&self, interval: u32, flags: SwapChainPresentFlags) -> HRESULT {
        unsafe { self.inner.Present(interval, flags.bits()) }
    }
}

impl SwapChain3 {
    pub fn get_current_back_buffer_index(&self) -> u32 {
        unsafe { self.inner.GetCurrentBackBufferIndex() }
    }
}
