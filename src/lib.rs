#[allow(dead_code)]
mod qsort {
    mod internal {
        extern "C" {
            pub fn swap(tab: *mut i64, i: usize, j: usize);
            pub fn partition(
                tab: *mut i64,
                lo: usize,
                sm: *mut usize,
                eq: *mut usize,
                hi: usize,
                pv: i64,
            );
        }
    }
    pub fn swap(tab: &mut Vec<i64>, i: usize, j: usize) {
        unsafe {
            internal::swap(tab.as_mut_ptr() as *mut i64, i, j);
        }
    }
    pub fn partition(
        tab: &mut Vec<i64>,
        lo: usize,
        sm: &mut usize,
        eq: &mut usize,
        hi: usize,
        pv: i64,
    ) {
        unsafe {
            internal::partition(
                tab.as_mut_ptr() as *mut i64,
                lo,
                sm as *mut usize,
                eq as *mut usize,
                hi,
                pv,
            );
        }
    }
}

}
