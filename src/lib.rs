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

#[allow(unused_imports)]
use qsort::*;

#[test]
fn test_swap() {
    let mut tab = vec![0, 1, 5, 3, 4, 2, 6, 7];
    swap(&mut tab, 2, 5);
    assert_eq!(tab, vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_swap_big() {
    let mut tab = vec![
        0 << 60,
        1 << 60,
        5 << 60,
        3 << 60,
        4 << 60,
        2 << 60,
        6 << 60,
        7 << 60,
    ];
    swap(&mut tab, 2, 5);
    for i in 0..tab.len() {
        assert_eq!(tab[i], (i as i64) << 60);
    }
}

#[test]
fn test_multi_swap() {
    let mut tab = vec![6, 5, 4, 3, 2, 1, 0];
    swap(&mut tab, 0, 6);
    swap(&mut tab, 1, 5);
    swap(&mut tab, 2, 4);
    swap(&mut tab, 3, 3);
    assert_eq!(tab, vec![0, 1, 2, 3, 4, 5, 6]);
}

}
