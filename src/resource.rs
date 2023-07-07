//! GPU Resource

use crate::{D3DResult, Rect};
use std::{ops::Range, ptr};
use windows::Win32::Graphics::Direct3D12::{ID3D12Resource, D3D12_DISCARD_REGION, D3D12_RANGE};

pub type Subresource = u32;

pub struct DiscardRegion<'a> {
    pub rects: &'a [Rect],
    pub subregions: Range<Subresource>,
}

impl DiscardRegion<'_> {
    pub(crate) fn to_ffi(&self) -> D3D12_DISCARD_REGION {
        D3D12_DISCARD_REGION {}
    }
}

pub struct Resource {
    pub(crate) inner: ID3D12Resource,
}

impl Resource {
    pub fn to_inner(&self) -> ID3D12Resource {
        self.inner.clone()
    }

    // TODO: WHY WHY WHY this return type aaagh
    pub fn map(
        &self,
        subresource: Subresource,
        read_range: Option<Range<usize>>,
    ) -> D3DResult<*mut ()> {
        let mut ptr = ptr::null_mut();
        let read_range = read_range.map(|r| D3D12_RANGE {
            Begin: r.start,
            End: r.end,
        });
        let hr = unsafe {
            self.inner.Map(
                subresource,
                read_range
                    .as_ref()
                    .map(|r| -> *const _ { &*r })
                    .map(|r| r.cast()),
                Some(&mut ptr),
            )
        };

        (ptr as _, hr)
    }

    pub fn unmap(&self, subresource: Subresource, write_range: Option<Range<usize>>) {
        let write_range = write_range.map(|r| D3D12_RANGE {
            Begin: r.start,
            End: r.end,
        });

        unsafe {
            self.inner.Unmap(
                subresource,
                write_range
                    .as_ref()
                    .map(|r| -> *const _ { &*r })
                    .map(|r| r.cast()),
            )
        };
    }

    pub fn gpu_virtual_address(&self) -> u64 {
        unsafe { self.inner.GetGPUVirtualAddress() }
    }
}
