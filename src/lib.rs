#[allow(dead_code)]
mod qsort {
    mod internal {
        extern "C" {
            pub fn swap(tab: *mut i64, i: usize, j: usize);
        }
    }
    pub fn swap(tab: &mut Vec<i64>, i: usize, j: usize) {
        unsafe {
            internal::swap(tab.as_mut_ptr() as *mut i64, i, j);
        }
    }
}
