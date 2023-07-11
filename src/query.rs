use windows::Win32::Graphics::Direct3D12::{
    ID3D12QueryHeap, D3D12_QUERY_HEAP_TYPE, D3D12_QUERY_HEAP_TYPE_OCCLUSION,
    D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS, D3D12_QUERY_HEAP_TYPE_SO_STATISTICS,
    D3D12_QUERY_HEAP_TYPE_TIMESTAMP,
};

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum QueryHeapType {
    Occlusion,
    Timestamp,
    PipelineStatistics,
    SOStatistics,
    // VideoDecodeStatistcs,
    // CopyQueueTimestamp,
}

impl From<QueryHeapType> for D3D12_QUERY_HEAP_TYPE {
    fn from(value: QueryHeapType) -> Self {
        match value {
            QueryHeapType::Occlusion => D3D12_QUERY_HEAP_TYPE_OCCLUSION,
            QueryHeapType::Timestamp => D3D12_QUERY_HEAP_TYPE_TIMESTAMP,
            QueryHeapType::PipelineStatistics => D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS,
            QueryHeapType::SOStatistics => D3D12_QUERY_HEAP_TYPE_SO_STATISTICS,
            // QueryHeapType::VideoDecodeStatistcs => D3D12_QUERY_HEAP_TYPE_VIDEO_DECODE_STATISTICS,
            // QueryHeapType::CopyQueueTimestamp => D3D12_QUERY_HEAP_TYPE_COPY_QUEUE_TIMESTAMP,
        }
    }
}

pub struct QueryHeap {
    #[allow(dead_code)] // TODO: remove this
    pub(crate) inner: ID3D12QueryHeap,
}
