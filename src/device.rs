//! Device

use windows::Win32::Graphics::Direct3D12::{
    ID3D12Device, D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_SIGNATURE_DESC, D3D12_COMPARISON_FUNC,
    D3D12_COMPUTE_PIPELINE_STATE_DESC, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_FENCE_FLAG_NONE,
    D3D12_FILTER, D3D12_HEAP_DESC, D3D12_QUERY_HEAP_DESC, D3D12_SAMPLER_DESC,
};

use crate::{
    command_list::{CmdListType, CommandSignature, IndirectArgument},
    descriptor::{CpuDescriptor, DescriptorHeapFlags, DescriptorHeapType, RenderTargetViewDesc},
    heap::{Heap, HeapFlags, HeapProperties},
    pso, query, queue, CachedPSO, CommandAllocator, CommandQueue, D3DResult, DescriptorHeap, Fence,
    GraphicsCommandList, NodeMask, PipelineState, QueryHeap, Resource, RootSignature, Shader,
    TextureAddressMode,
};
use std::{ops::Range, ptr};

pub struct Device {
    inner: ID3D12Device,
}

#[cfg(feature = "libloading")]
impl crate::D3D12Lib {
    // TODO: redundant with `Device::create`?
    pub fn create_device<I: Interface>(
        &self,
        adapter: &ComPtr<I>,
        feature_level: crate::FeatureLevel,
    ) -> Result<D3DResult<Device>, libloading::Error> {
        use windows::Win32::Graphics::Direct3D12::D3D12CreateDevice;

        let mut device = None;
        let hr = unsafe { D3D12CreateDevice(adapter, feature_level, &mut device) };

        Ok((device, hr))
    }
}

impl Device {
    // TODO: redundant with `D3D12Lib::create_device`?
    #[cfg(feature = "implicit-link")]
    pub fn create<I: Interface>(
        adapter: ComPtr<I>,
        feature_level: crate::FeatureLevel,
    ) -> D3DResult<Self> {
        use windows::Win32::Graphics::Direct3D12::D3D12CreateDevice;

        let mut device = None;
        let hr = unsafe { D3D12CreateDevice(adapter, feature_level, &mut device) };

        (device, hr)
    }

    pub fn create_heap(
        &self,
        size_in_bytes: u64,
        properties: HeapProperties,
        alignment: u64,
        flags: HeapFlags,
    ) -> D3DResult<Heap> {
        let mut heap = None;

        let desc = D3D12_HEAP_DESC {
            SizeInBytes: size_in_bytes,
            Properties: properties.0,
            Alignment: alignment,
            Flags: flags,
        };

        unsafe { self.inner.CreateHeap(&desc, &mut heap) }.map(|()| heap.unwrap())
    }

    pub fn create_command_allocator(&self, list_type: CmdListType) -> D3DResult<CommandAllocator> {
        unsafe { self.inner.CreateCommandAllocator(list_type.into()) }
            .map(|inner| CommandAllocator { inner })
    }

    pub fn create_command_queue(
        &self,
        list_type: CmdListType,
        priority: queue::Priority,
        flags: queue::CommandQueueFlags,
        node_mask: NodeMask,
    ) -> D3DResult<CommandQueue> {
        let desc = D3D12_COMMAND_QUEUE_DESC {
            Type: list_type as _,
            Priority: priority as _,
            Flags: flags.bits(),
            NodeMask: node_mask,
        };

        let queue_res = unsafe { self.inner.CreateCommandQueue(&desc) };

        queue_res
    }

    pub fn create_descriptor_heap(
        &self,
        num_descriptors: u32,
        heap_type: DescriptorHeapType,
        flags: DescriptorHeapFlags,
        node_mask: NodeMask,
    ) -> D3DResult<DescriptorHeap> {
        let desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type: heap_type as _,
            NumDescriptors: num_descriptors,
            Flags: flags.bits(),
            NodeMask: node_mask,
        };

        let heap_res = unsafe { self.inner.CreateDescriptorHeap(&desc) };

        heap_res
    }

    pub fn get_descriptor_increment_size(&self, heap_type: DescriptorHeapType) -> u32 {
        unsafe { self.GetDescriptorHandleIncrementSize(heap_type as _) }
    }

    pub fn create_graphics_command_list(
        &self,
        list_type: CmdListType,
        allocator: &CommandAllocator,
        initial: PipelineState,
        node_mask: NodeMask,
    ) -> D3DResult<GraphicsCommandList> {
        let mut command_list = GraphicsCommandList::null();
        let cmd_list_res = unsafe {
            self.inner
                .CreateCommandList(node_mask, list_type as _, &allocator, &initial)
        };

        cmd_list_res
    }

    pub fn create_query_heap(
        &self,
        heap_ty: query::QueryHeapType,
        count: u32,
        node_mask: NodeMask,
    ) -> D3DResult<QueryHeap> {
        let desc = D3D12_QUERY_HEAP_DESC {
            Type: heap_ty as _,
            Count: count,
            NodeMask: node_mask,
        };

        let mut query_heap = None;
        let hr = unsafe { self.inner.CreateQueryHeap(&desc, &mut query_heap) };

        (query_heap, hr)
    }

    pub fn create_graphics_pipeline_state(
        &self,
        _root_signature: RootSignature,
        _vs: Shader,
        _ps: Shader,
        _gs: Shader,
        _hs: Shader,
        _ds: Shader,
        _node_mask: NodeMask,
        _cached_pso: CachedPSO,
        _flags: pso::PipelineStateFlags,
    ) -> D3DResult<PipelineState> {
        unimplemented!()
    }

    pub fn create_compute_pipeline_state(
        &self,
        root_signature: &RootSignature,
        cs: Shader,
        node_mask: NodeMask,
        cached_pso: CachedPSO,
        flags: pso::PipelineStateFlags,
    ) -> D3DResult<PipelineState> {
        let mut pipeline = PipelineState::null();
        let desc = D3D12_COMPUTE_PIPELINE_STATE_DESC {
            pRootSignature: root_signature.as_mut_ptr(),
            CS: *cs,
            NodeMask: node_mask,
            CachedPSO: *cached_pso,
            Flags: flags.bits(),
        };

        let pipeline_res = unsafe { self.inner.CreateComputePipelineState(&desc) };

        pipeline_res
    }

    pub fn create_sampler(
        &self,
        sampler: CpuDescriptor,
        filter: D3D12_FILTER,
        address_mode: TextureAddressMode,
        mip_lod_bias: f32,
        max_anisotropy: u32,
        comparison_op: D3D12_COMPARISON_FUNC,
        border_color: [f32; 4],
        lod: Range<f32>,
    ) {
        let desc = D3D12_SAMPLER_DESC {
            Filter: filter,
            AddressU: address_mode[0],
            AddressV: address_mode[1],
            AddressW: address_mode[2],
            MipLODBias: mip_lod_bias,
            MaxAnisotropy: max_anisotropy,
            ComparisonFunc: comparison_op,
            BorderColor: border_color,
            MinLOD: lod.start,
            MaxLOD: lod.end,
        };

        unsafe {
            self.CreateSampler(&desc, sampler);
        }
    }

    pub fn create_root_signature(
        &self,
        blob: &[u8],
        node_mask: NodeMask,
    ) -> D3DResult<RootSignature> {
        let mut signature = RootSignature::null();
        let hr = unsafe { self.inner.CreateRootSignature(node_mask, blob) };

        (signature, hr)
    }

    pub fn create_command_signature(
        &self,
        root_signature: RootSignature,
        arguments: &[IndirectArgument],
        stride: u32,
        node_mask: NodeMask,
    ) -> D3DResult<CommandSignature> {
        let mut signature = None;
        let desc = D3D12_COMMAND_SIGNATURE_DESC {
            ByteStride: stride,
            NumArgumentDescs: arguments.len() as _,
            pArgumentDescs: arguments.as_ptr() as *const _,
            NodeMask: node_mask,
        };

        let hr = unsafe {
            self.inner
                .CreateCommandSignature(&desc, &root_signature, &mut signature)
        };

        (signature, hr)
    }

    pub fn create_render_target_view(
        &self,
        resource: Resource,
        desc: &RenderTargetViewDesc,
        descriptor: CpuDescriptor,
    ) {
        unsafe {
            self.CreateRenderTargetView(resource.as_mut_ptr(), &desc.0 as *const _, descriptor);
        }
    }

    // TODO: interface not complete
    pub fn create_fence(&self, initial: u64) -> D3DResult<Fence> {
        let fence_res = unsafe { self.inner.CreateFence(initial, D3D12_FENCE_FLAG_NONE) };

        fence_res
    }
}
