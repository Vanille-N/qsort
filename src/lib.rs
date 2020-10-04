#[macro_use]
extern crate more_asserts;

#[allow(dead_code)]
mod qsort {
    mod internal {
        extern "C" {
            pub fn swap(tab: *mut i64, i: usize, j: usize);
            pub fn choose_pivot(tab: *const i64, lo: usize, hi: usize) -> i64;
            pub fn partition(
                tab: *mut i64,
                lo: usize,
                sm: *mut usize,
                eq: *mut usize,
                hi: usize,
                pv: i64,
            );
            pub fn qsort_aux(tab: *mut i64, lo: usize, hi: usize);
        }
    }
    pub fn swap(tab: &mut Vec<i64>, i: usize, j: usize) {
        unsafe {
            internal::swap(tab.as_mut_ptr() as *mut i64, i, j);
        }
    }
    pub fn choose_pivot(tab: &Vec<i64>, lo: usize, hi: usize) -> i64 {
        unsafe {
            internal::choose_pivot(tab.as_ptr() as *const i64, lo, hi)
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
    pub fn qsort_aux(tab: &mut Vec<i64>, lo: usize, hi: usize) {
        unsafe {
            internal::qsort_aux(tab.as_mut_ptr() as *mut i64, lo, hi);
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

#[test]
fn test_choose_pivot() {
    let tab = vec![0, 1, 2, 3, 4, 5];
    let pv = choose_pivot(&tab, 2, 4);
    assert_eq!(pv, 2);
}

#[cfg(test)]
fn assert_partitioned(tab: &Vec<i64>, lo: usize, sm: usize, eq: usize, hi: usize, pv: i64) {
    for i in lo..sm {
        assert_lt!(tab[i], pv);
    }
    for i in sm..eq {
        assert_eq!(tab[i], pv);
    }
    for i in eq..hi {
        assert_gt!(tab[i], pv);
    }
}

#[test]
fn test_partition() {
    let mut tab = vec![0, 4, 2, 8, 4, 6, 3, 9, 1, 7];
    let lo = 0;
    let mut sm = lo;
    let mut eq = lo;
    let hi = tab.len();
    partition(&mut tab, lo, &mut sm, &mut eq, hi, 5);
    assert_partitioned(&tab, lo, sm, eq, hi, 5);
}

#[test]
fn test_qsort_aux() {
    let mut tab = vec![101, 105, 104, 5, 3, 9, 1, 7, 2, 5, 6, 7, 1, 2, -207, -203, -206];
    qsort_aux(&mut tab, 3, 14);
    println!("{:?}", tab);
    panic!();
}
