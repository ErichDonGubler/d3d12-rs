//! Graphics command list

use std::mem::{self, ManuallyDrop};

use windows::{
    core::ComInterface,
    Win32::Graphics::Direct3D12::{
        ID3D12CommandList, ID3D12CommandSignature, ID3D12GraphicsCommandList, D3D12_CLEAR_FLAGS,
        D3D12_COMMAND_LIST_TYPE, D3D12_COMMAND_LIST_TYPE_BUNDLE, D3D12_COMMAND_LIST_TYPE_COMPUTE,
        D3D12_COMMAND_LIST_TYPE_COPY, D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_INDEX_BUFFER_VIEW,
        D3D12_INDIRECT_ARGUMENT_DESC, D3D12_INDIRECT_ARGUMENT_DESC_0,
        D3D12_INDIRECT_ARGUMENT_DESC_0_0, D3D12_INDIRECT_ARGUMENT_DESC_0_1,
        D3D12_INDIRECT_ARGUMENT_DESC_0_2, D3D12_INDIRECT_ARGUMENT_DESC_0_3,
        D3D12_INDIRECT_ARGUMENT_DESC_0_4, D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT,
        D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT_BUFFER_VIEW, D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH,
        D3D12_INDIRECT_ARGUMENT_TYPE_DRAW, D3D12_INDIRECT_ARGUMENT_TYPE_DRAW_INDEXED,
        D3D12_INDIRECT_ARGUMENT_TYPE_SHADER_RESOURCE_VIEW,
        D3D12_INDIRECT_ARGUMENT_TYPE_UNORDERED_ACCESS_VIEW,
        D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW, D3D12_RESOURCE_BARRIER,
        D3D12_RESOURCE_BARRIER_0, D3D12_RESOURCE_BARRIER_FLAGS,
        D3D12_RESOURCE_BARRIER_TYPE_TRANSITION, D3D12_RESOURCE_STATES,
        D3D12_RESOURCE_TRANSITION_BARRIER,
    },
};

use crate::{
    resource::DiscardRegion, CommandAllocator, CpuDescriptor, DescriptorHeap, Format, GpuAddress,
    GpuDescriptor, IndexCount, InstanceCount, PipelineState, Rect, Resource, RootIndex,
    RootSignature, Subresource, VertexCount, VertexOffset, WorkGroupCount, HRESULT,
};

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum CmdListType {
    Direct,
    Bundle,
    Compute,
    Copy,
    // VideoDecode,
    // VideoProcess,
}

impl From<CmdListType> for D3D12_COMMAND_LIST_TYPE {
    fn from(value: CmdListType) -> Self {
        match value {
            CmdListType::Direct => D3D12_COMMAND_LIST_TYPE_DIRECT,
            CmdListType::Bundle => D3D12_COMMAND_LIST_TYPE_BUNDLE,
            CmdListType::Compute => D3D12_COMMAND_LIST_TYPE_COMPUTE,
            CmdListType::Copy => D3D12_COMMAND_LIST_TYPE_COPY,
            // CmdListType::VideoDecode => D3D12_COMMAND_LIST_TYPE_VIDEO_DECODE,
            // CmdListType::VideoProcess => D3D12_COMMAND_LIST_TYPE_VIDEO_PROCESS,
        }
    }
}

// NOTE: regressed `PartialOrd`, but y wud u do dis neway
pub type ClearFlags = D3D12_CLEAR_FLAGS;

#[repr(transparent)]
pub struct IndirectArgument(D3D12_INDIRECT_ARGUMENT_DESC);

impl IndirectArgument {
    pub fn draw() -> Self {
        IndirectArgument(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_DRAW,
            ..Default::default()
        })
    }

    pub fn draw_indexed() -> Self {
        IndirectArgument(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_DRAW_INDEXED,
            ..Default::default()
        })
    }

    pub fn dispatch() -> Self {
        IndirectArgument(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH,
            ..Default::default()
        })
    }

    pub fn vertex_buffer(slot: u32) -> Self {
        IndirectArgument(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW,
            Anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0 {
                VertexBuffer: D3D12_INDIRECT_ARGUMENT_DESC_0_4 { Slot: slot },
            },
        })
    }

    pub fn constant(root_index: RootIndex, dest_offset_words: u32, count: u32) -> Self {
        IndirectArgument(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT,
            Anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0 {
                Constant: D3D12_INDIRECT_ARGUMENT_DESC_0_1 {
                    RootParameterIndex: root_index,
                    DestOffsetIn32BitValues: dest_offset_words,
                    Num32BitValuesToSet: count,
                },
            },
        })
    }

    pub fn constant_buffer_view(root_index: RootIndex) -> Self {
        IndirectArgument(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT_BUFFER_VIEW,
            Anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0 {
                ConstantBufferView: D3D12_INDIRECT_ARGUMENT_DESC_0_0 {
                    RootParameterIndex: root_index,
                },
            },
        })
    }

    pub fn shader_resource_view(root_index: RootIndex) -> Self {
        IndirectArgument(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_SHADER_RESOURCE_VIEW,
            Anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0 {
                ShaderResourceView: D3D12_INDIRECT_ARGUMENT_DESC_0_2 {
                    RootParameterIndex: root_index,
                },
            },
        })
    }

    pub fn unordered_access_view(root_index: RootIndex) -> Self {
        IndirectArgument(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_UNORDERED_ACCESS_VIEW,
            Anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0 {
                UnorderedAccessView: D3D12_INDIRECT_ARGUMENT_DESC_0_3 {
                    RootParameterIndex: root_index,
                },
            },
        })
    }
}

#[repr(transparent)]
pub struct ResourceBarrier(D3D12_RESOURCE_BARRIER);

impl ResourceBarrier {
    pub fn transition(
        resource: Resource,
        subresource: Subresource,
        state_before: D3D12_RESOURCE_STATES,
        state_after: D3D12_RESOURCE_STATES,
        flags: D3D12_RESOURCE_BARRIER_FLAGS,
    ) -> Self {
        ResourceBarrier(D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            Flags: flags,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                    // TODO: I feel spooked. Why is this the public API? Is there client-side
                    // dropping that needs to be done, if that were even possible?
                    pResource: ManuallyDrop::new(Some(resource.to_inner())),
                    Subresource: subresource,
                    StateBefore: state_before,
                    StateAfter: state_after,
                }),
            },
        })
    }
}

pub type CommandSignature = ID3D12CommandSignature;
pub type CommandList = ID3D12CommandList;

pub struct GraphicsCommandList {
    inner: ID3D12GraphicsCommandList,
}

impl GraphicsCommandList {
    pub fn as_list(&self) -> CommandList {
        self.inner.cast::<CommandList>().unwrap()
    }

    pub fn close(&self) -> HRESULT {
        let err_to_hres = |res: Result<_, windows::core::Error>| match res.map_err(|e| e.code()) {
            Ok(()) => HRESULT(0), // the value of `S_OK`
            Err(code) => code,
        };
        unsafe { err_to_hres(self.inner.Close()) }
    }

    pub fn reset(&self, allocator: &CommandAllocator, initial_pso: PipelineState) -> HRESULT {
        let err_to_hres = |res: Result<_, windows::core::Error>| match res.map_err(|e| e.code()) {
            Ok(()) => HRESULT(0), // the value of `S_OK`
            Err(code) => code,
        };
        unsafe { err_to_hres(self.inner.Reset(&allocator.inner, &initial_pso)) }
    }

    pub fn discard_resource(&self, resource: Resource, region: DiscardRegion) {
        debug_assert!(region.subregions.start < region.subregions.end);
        let region = region.to_ffi();
        let region = Some(&region).map(|r| -> *const _ { r });
        unsafe {
            self.inner.DiscardResource(&resource.inner, region);
        }
    }

    pub fn clear_depth_stencil_view(
        &self,
        dsv: CpuDescriptor,
        flags: ClearFlags,
        depth: f32,
        stencil: u8,
        rects: &[Rect],
    ) {
        unsafe {
            self.inner
                .ClearDepthStencilView(dsv, flags, depth, stencil, rects);
        }
    }

    pub fn clear_render_target_view(&self, rtv: CpuDescriptor, color: [f32; 4], rects: &[Rect]) {
        // TODO: What does using `None` here actually do? Not sure from documentation:
        // <https://learn.microsoft.com/en-us/windows/win32/api/d3d11/nf-d3d11-id3d11devicecontext-clearrendertargetview>
        unsafe {
            self.inner
                .ClearRenderTargetView(rtv, color.as_ptr(), Some(rects));
        }
    }

    pub fn dispatch(&self, count: WorkGroupCount) {
        unsafe {
            self.inner.Dispatch(count[0], count[1], count[2]);
        }
    }

    pub fn draw(
        &self,
        num_vertices: VertexCount,
        num_instances: InstanceCount,
        start_vertex: VertexCount,
        start_instance: InstanceCount,
    ) {
        unsafe {
            self.inner
                .DrawInstanced(num_vertices, num_instances, start_vertex, start_instance);
        }
    }

    pub fn draw_indexed(
        &self,
        num_indices: IndexCount,
        num_instances: InstanceCount,
        start_index: IndexCount,
        base_vertex: VertexOffset,
        start_instance: InstanceCount,
    ) {
        unsafe {
            self.inner.DrawIndexedInstanced(
                num_indices,
                num_instances,
                start_index,
                base_vertex,
                start_instance,
            );
        }
    }

    pub fn set_index_buffer(&self, gpu_address: GpuAddress, size: u32, format: Format) {
        let ibv = D3D12_INDEX_BUFFER_VIEW {
            BufferLocation: gpu_address,
            SizeInBytes: size,
            Format: format,
        };
        unsafe {
            self.inner.IASetIndexBuffer(Some(&ibv));
        }
    }

    pub fn set_blend_factor(&self, factor: [f32; 4]) {
        unsafe {
            self.inner.OMSetBlendFactor(Some(&factor));
        }
    }

    pub fn set_stencil_reference(&self, reference: u32) {
        unsafe {
            self.inner.OMSetStencilRef(reference);
        }
    }

    pub fn set_pipeline_state(&self, pso:&PipelineState) {
        unsafe {
            self.inner.SetPipelineState(&pso);
        }
    }

    pub fn execute_bundle(&self, bundle: GraphicsCommandList) {
        unsafe {
            self.inner.ExecuteBundle(&bundle.inner);
        }
    }

    pub fn set_descriptor_heaps(&self, heaps: &[DescriptorHeap]) {
        unsafe {
            self.inner.SetDescriptorHeaps(heaps);
        }
    }

    pub fn set_compute_root_signature(&self, signature: &RootSignature) {
        unsafe {
            self.inner.SetComputeRootSignature(&signature.inner);
        }
    }

    pub fn set_graphics_root_signature(&self, signature: &RootSignature) {
        unsafe {
            self.inner.SetGraphicsRootSignature(&signature.inner);
        }
    }

    pub fn set_compute_root_descriptor_table(
        &self,
        root_index: RootIndex,
        base_descriptor: GpuDescriptor,
    ) {
        unsafe {
            self.inner
                .SetComputeRootDescriptorTable(root_index, base_descriptor);
        }
    }

    pub fn set_compute_root_constant_buffer_view(
        &self,
        root_index: RootIndex,
        buffer_location: GpuAddress,
    ) {
        unsafe {
            self.inner
                .SetComputeRootConstantBufferView(root_index, buffer_location);
        }
    }

    pub fn set_compute_root_shader_resource_view(
        &self,
        root_index: RootIndex,
        buffer_location: GpuAddress,
    ) {
        unsafe {
            self.inner
                .SetComputeRootShaderResourceView(root_index, buffer_location);
        }
    }

    pub fn set_compute_root_unordered_access_view(
        &self,
        root_index: RootIndex,
        buffer_location: GpuAddress,
    ) {
        unsafe {
            self.inner
                .SetComputeRootUnorderedAccessView(root_index, buffer_location);
        }
    }

    pub fn set_compute_root_constant(
        &self,
        root_index: RootIndex,
        value: u32,
        dest_offset_words: u32,
    ) {
        unsafe {
            self.inner
                .SetComputeRoot32BitConstant(root_index, value, dest_offset_words);
        }
    }

    pub fn set_graphics_root_descriptor_table(
        &self,
        root_index: RootIndex,
        base_descriptor: GpuDescriptor,
    ) {
        unsafe {
            self.inner
                .SetGraphicsRootDescriptorTable(root_index, base_descriptor);
        }
    }

    pub fn set_graphics_root_constant_buffer_view(
        &self,
        root_index: RootIndex,
        buffer_location: GpuAddress,
    ) {
        unsafe {
            self.inner
                .SetGraphicsRootConstantBufferView(root_index, buffer_location);
        }
    }

    pub fn set_graphics_root_shader_resource_view(
        &self,
        root_index: RootIndex,
        buffer_location: GpuAddress,
    ) {
        unsafe {
            self.inner
                .SetGraphicsRootShaderResourceView(root_index, buffer_location);
        }
    }

    pub fn set_graphics_root_unordered_access_view(
        &self,
        root_index: RootIndex,
        buffer_location: GpuAddress,
    ) {
        unsafe {
            self.inner
                .SetGraphicsRootUnorderedAccessView(root_index, buffer_location);
        }
    }

    pub fn set_graphics_root_constant(
        &self,
        root_index: RootIndex,
        value: u32,
        dest_offset_words: u32,
    ) {
        unsafe {
            self.inner
                .SetGraphicsRoot32BitConstant(root_index, value, dest_offset_words);
        }
    }

    pub fn resource_barrier(&self, barriers: &[ResourceBarrier]) {
        // SAFETY: Safe because barriers is `repr(transparent)` containing our target type.
        // TODO: Find source for transmuting slices.
        let barriers = unsafe { mem::transmute(barriers) };
        unsafe { self.inner.ResourceBarrier(barriers) }
    }
}
