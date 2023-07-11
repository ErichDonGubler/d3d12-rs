use windows::{
    core::PCSTR,
    imp::WaitForSingleObject,
    Win32::{
        Foundation::HANDLE, Graphics::Direct3D12::ID3D12Fence, System::Threading::CreateEventA,
    },
};

use crate::HRESULT;

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Event(pub HANDLE);
impl Event {
    pub fn create(manual_reset: bool, initial_state: bool) -> Self {
        Event(unsafe { CreateEventA(None, manual_reset, initial_state, PCSTR::null()) }.unwrap())
    }

    // TODO: return value
    pub fn wait(&self, timeout_ms: u32) -> u32 {
        unsafe { WaitForSingleObject(self.0 .0, timeout_ms) }
    }
}

pub struct Fence {
    pub(crate) inner: ID3D12Fence,
}

impl Fence {
    pub fn set_event_on_completion(&self, event: Event, value: u64) -> HRESULT {
        unsafe { self.inner.SetEventOnCompletion(value, event.0) }
            .err()
            .map(|e| e.code())
            .unwrap_or(HRESULT(0))
    }

    pub fn get_value(&self) -> u64 {
        unsafe { self.inner.GetCompletedValue() }
    }

    pub fn signal(&self, value: u64) -> HRESULT {
        unsafe { self.inner.Signal(value) }
            .err()
            .map(|e| e.code())
            .unwrap_or(HRESULT(0))
    }
}
