use windows::Win32::Graphics::Direct3D12::{
    ID3D12Heap, D3D12_CPU_PAGE_PROPERTY, D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE,
    D3D12_CPU_PAGE_PROPERTY_UNKNOWN, D3D12_CPU_PAGE_PROPERTY_WRITE_BACK,
    D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE, D3D12_HEAP_DESC, D3D12_HEAP_FLAGS,
    D3D12_HEAP_PROPERTIES, D3D12_HEAP_TYPE, D3D12_HEAP_TYPE_CUSTOM, D3D12_HEAP_TYPE_DEFAULT,
    D3D12_HEAP_TYPE_READBACK, D3D12_HEAP_TYPE_UPLOAD, D3D12_MEMORY_POOL, D3D12_MEMORY_POOL_L0,
    D3D12_MEMORY_POOL_L1,
};

pub type Heap = ID3D12Heap;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum HeapType {
    Default,
    Upload,
    Readback,
    Custom,
}

impl From<HeapType> for D3D12_HEAP_TYPE {
    fn from(value: HeapType) -> Self {
        match value {
            HeapType::Default => D3D12_HEAP_TYPE_DEFAULT,
            HeapType::Upload => D3D12_HEAP_TYPE_UPLOAD,
            HeapType::Readback => D3D12_HEAP_TYPE_READBACK,
            HeapType::Custom => D3D12_HEAP_TYPE_CUSTOM,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum CpuPageProperty {
    Unknown,
    NotAvailable,
    WriteCombine,
    WriteBack,
}

impl From<CpuPageProperty> for D3D12_CPU_PAGE_PROPERTY {
    fn from(value: CpuPageProperty) -> Self {
        match value {
            CpuPageProperty::Unknown => D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
            CpuPageProperty::NotAvailable => D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE,
            CpuPageProperty::WriteCombine => D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE,
            CpuPageProperty::WriteBack => D3D12_CPU_PAGE_PROPERTY_WRITE_BACK,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum MemoryPool {
    Unknown,
    L0,
    L1,
}

impl From<MemoryPool> for D3D12_MEMORY_POOL {
    fn from(value: MemoryPool) -> Self {
        match value {
            MemoryPool::Unknown => D3D12_MEMORY_POOL(D3D12_CPU_PAGE_PROPERTY_UNKNOWN.0),
            MemoryPool::L0 => D3D12_MEMORY_POOL_L0,
            MemoryPool::L1 => D3D12_MEMORY_POOL_L1,
        }
    }
}

impl Default for MemoryPool {
    fn default() -> Self {
        Self::Unknown
    }
}

// NOTE: regressed `PartialOrd`, but y wud u do dis neway
pub type HeapFlags = D3D12_HEAP_FLAGS;

// TODO: Don't make the inner thing `pub`!
#[repr(transparent)]
pub struct HeapProperties(pub D3D12_HEAP_PROPERTIES);
impl HeapProperties {
    pub fn new(
        heap_type: HeapType,
        cpu_page_property: CpuPageProperty,
        memory_pool_preference: MemoryPool,
        creation_node_mask: u32,
        visible_node_mask: u32,
    ) -> Self {
        HeapProperties(D3D12_HEAP_PROPERTIES {
            Type: heap_type.into(),
            CPUPageProperty: cpu_page_property.into(),
            MemoryPoolPreference: memory_pool_preference.into(),
            CreationNodeMask: creation_node_mask,
            VisibleNodeMask: visible_node_mask,
        })
    }
}

#[repr(transparent)]
pub struct HeapDesc(D3D12_HEAP_DESC);
impl HeapDesc {
    pub fn new(
        size_in_bytes: u64,
        properties: HeapProperties,
        alignment: u64,
        flags: HeapFlags,
    ) -> Self {
        HeapDesc(D3D12_HEAP_DESC {
            SizeInBytes: size_in_bytes,
            Properties: properties.0,
            Alignment: alignment,
            Flags: flags,
        })
    }
}
