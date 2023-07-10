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
            Occlusion => D3D12_QUERY_HEAP_TYPE_OCCLUSION,
            Timestamp => D3D12_QUERY_HEAP_TYPE_TIMESTAMP,
            PipelineStatistics => D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS,
            SOStatistics => D3D12_QUERY_HEAP_TYPE_SO_STATISTICS,
            // VideoDecodeStatistcs => D3D12_QUERY_HEAP_TYPE_VIDEO_DECODE_STATISTICS,
            // CopyQueueTimestamp => D3D12_QUERY_HEAP_TYPE_COPY_QUEUE_TIMESTAMP,
        }
    }
}

pub type QueryHeap = ID3D12QueryHeap;
