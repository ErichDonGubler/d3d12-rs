use crate::{sync::Fence, CommandList, D3DResult};
use windows::Win32::Graphics::Direct3D12::{
    ID3D12CommandQueue, D3D12_COMMAND_QUEUE_FLAGS, D3D12_COMMAND_QUEUE_PRIORITY,
    D3D12_COMMAND_QUEUE_PRIORITY_GLOBAL_REALTIME, D3D12_COMMAND_QUEUE_PRIORITY_HIGH,
    D3D12_COMMAND_QUEUE_PRIORITY_NORMAL,
};

pub enum Priority {
    Normal,
    High,
    GlobalRealtime,
}

impl From<Priority> for D3D12_COMMAND_QUEUE_PRIORITY {
    fn from(value: Priority) -> Self {
        match value {
            Priority::Normal => D3D12_COMMAND_QUEUE_PRIORITY_NORMAL,
            Priority::High => D3D12_COMMAND_QUEUE_PRIORITY_HIGH,
            Priority::GlobalRealtime => D3D12_COMMAND_QUEUE_PRIORITY_GLOBAL_REALTIME,
        }
    }
}

// NOTE: regressed `PartialOrd`, but y wud u do dis neway
pub type CommandQueueFlags = D3D12_COMMAND_QUEUE_FLAGS;

pub struct CommandQueue {
    pub(crate) inner: ID3D12CommandQueue,
}

impl CommandQueue {
    pub fn execute_command_lists(&self, command_lists: &[CommandList]) {
        let command_lists = command_lists
            .iter()
            .cloned()
            .map(|outer| Some(outer.inner))
            .collect::<Box<[_]>>();
        unsafe { self.inner.ExecuteCommandLists(&*command_lists) }
    }

    pub fn signal(&self, fence: Fence, value: u64) -> D3DResult<()> {
        unsafe { self.inner.Signal(&fence.inner, value) }
    }
}
