use generic_array::{ArrayLength, GenericArray};
//use core::marker::PhantomPinned;
use core::ops::{Index, IndexMut};

/// Trait for implementing WideSlice
pub trait IntoWideSlice<'a> {
    fn into_wideslice(&'a mut self) -> WideSlice<'a>;
}

/// The platform independent data storage
#[derive(Debug)]
pub struct WideSlice<'a> {
    pub slices: &'a mut [*mut f64],
    slice_len: usize,
}

impl<'a> WideSlice<'a> {
    /// Gets the `idx`-th slice from the WideSlice
    pub fn as_slice(&self, idx: usize) -> &'a [f64] {
        // Safe through the construction and layout of the WideSlice
        assert!(idx < self.slices.len());
        unsafe { std::slice::from_raw_parts(self.slices[idx], self.slice_len) }
    }

    /// Gets the `idx`-th slice from the WideSlice, mutable slice version
    pub fn as_mut_slice(&mut self, idx: usize) -> &'a mut [f64] {
        // Safe through the construction and layout of the WideSlice
        assert!(idx < self.slices.len());
        unsafe { std::slice::from_raw_parts_mut(self.slices[idx], self.slice_len) }
    }
}

impl<'a> Index<usize> for WideSlice<'a> {
    type Output = [f64];
    fn index(&self, idx: usize) -> &Self::Output {
        self.as_slice(idx)
    }
}

impl<'a> IndexMut<usize> for WideSlice<'a> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.as_mut_slice(idx)
    }
}

/// Memory that is head allocated (std)
#[derive(Debug)]
pub struct HeapMem {
    buf: Box<[f64]>,
    ptr: Box<[*mut f64]>,
}

impl HeapMem {
    pub fn new(slice_size: usize, n_slices: usize) -> HeapMem {
        let mut buf = vec![0.0; slice_size * n_slices].into_boxed_slice();
        let ptr = {
            let mut v = Vec::with_capacity(n_slices);

            for chunk in buf.chunks_exact_mut(slice_size) {
                v.push(chunk.as_mut_ptr());
            }

            v.into_boxed_slice()
        };

        HeapMem { buf: buf, ptr: ptr }
    }
}

impl<'a> IntoWideSlice<'a> for HeapMem {
    fn into_wideslice(&mut self) -> WideSlice {
        let slice_len = self.buf.len() / self.ptr.len();

        WideSlice {
            slices: &mut self.ptr,
            slice_len: slice_len,
        }
    }
}

/// Memory that is statically allocated (no-std)
#[derive(Debug)]
pub struct StaticMem {
    buf: &'static mut [f64],
    ptr: &'static mut [*mut f64],
}

impl StaticMem {
    pub fn new(buf: &'static mut [f64], ptr_buf: &'static mut [*mut f64]) -> StaticMem {
        // assert!(buf.len() == slice_size * n_slices);
        // assert!(ptr_buf.len() == n_slices);
        assert!(buf.len() % ptr_buf.len() == 0);
        assert!(ptr_buf.len() > 0);
        assert!(buf.len() > 1);

        let slice_len = buf.len() / ptr_buf.len();
        let base_addr = buf.as_mut_ptr();

        // Fill the pointer buffer
        for (idx, p) in ptr_buf.iter_mut().enumerate() {
            *p = unsafe { base_addr.add(idx * slice_len) };
        }

        StaticMem {
            buf: buf,
            ptr: ptr_buf,
        }
    }
}

impl<'a> IntoWideSlice<'a> for StaticMem {
    fn into_wideslice(&'a mut self) -> WideSlice<'a> {
        let slice_len = self.buf.len() / self.ptr.len();

        WideSlice {
            slices: &mut self.ptr,
            slice_len: slice_len,
        }
    }
}

/// Memory that is compile-time allocated (no-std)
#[derive(Debug)]
pub struct ConstMem<SliceSize, NSlices>
where
    SliceSize: ArrayLength<f64>,
    NSlices: ArrayLength<f64> + ArrayLength<GenericArray<f64, SliceSize>> + ArrayLength<*mut f64>,
{
    buf: GenericArray<GenericArray<f64, SliceSize>, NSlices>,
    ptr: GenericArray<*mut f64, NSlices>,
    //pinned: PhantomPinned,
}

impl<SliceSize, NSlices> ConstMem<SliceSize, NSlices>
where
    SliceSize: ArrayLength<f64>,
    NSlices: ArrayLength<f64> + ArrayLength<GenericArray<f64, SliceSize>> + ArrayLength<*mut f64>,
{
}

// /// Memory that is stack allocated (no-std)
// #[derive(Debug)]
// pub struct StackMem {
//     // pub struct<const SLICE_SIZE: usize, const N_SLICES: usize> StackMem {
//     buf: [f64; 8], // No const generics yet :(
//     ptr: [*mut f64; 4],
// }
//
// impl StackMem {
//     pub fn new() -> StackMem {
//         StackMem {
//             buf: [0.0; 8],
//             ptr: [0 as *mut f64; 4],
//         }
//     }
// }
//
// impl<'a> IntoWideSlice<'a> for StackMem {
//     fn into_wideslice(&mut self) -> WideSlice {
//         let len = self.buf.len() / self.ptr.len();
//
//         for (chunk, ptr) in self.buf.chunks_exact_mut(2).zip(self.ptr.iter_mut()) {
//             *ptr = chunk.as_mut_ptr()
//         }
//
//         WideSlice {
//             slices: &mut self.ptr,
//             len: len,
//         }
//     }
// }
