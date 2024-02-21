//! 主要功能
//!
//! quickselect 实现选择算法的库。
//!
#[cfg(test)]
mod tests {
    use crate::quick_select;
    #[test]
    fn quick_select_test() {
        let mut arr_f64 = vec![
            65.0, 28.0, 59.0, 33.0, 21.0, 56.0, 22.0, 95.0, 50.0, 12.0, 90.0, 53.0, 28.0, 77.0,
            39.0,
        ];
        let length = arr_f64.len();
        quick_select(&mut arr_f64, 8, 0, length - 1, true);
        assert_eq!(
            arr_f64,
            vec![
                39.0, 28.0, 28.0, 33.0, 21.0, 12.0, 22.0, 50.0, 53.0, 56.0, 59.0, 65.0, 90.0, 77.0,
                95.0
            ]
        );

        let mut arr_i64 = vec![65, 28, 59, 33, 21, 56, 22, 95, 50, 12, 90, 53, 28, 77, 39];
        let length = arr_i64.len();
        quick_select(&mut arr_i64, 8, 0, length - 1, true);
        assert_eq!(
            arr_i64,
            vec![39, 28, 28, 33, 21, 12, 22, 50, 53, 56, 59, 65, 90, 77, 95]
        );
    }
}
use std::cmp::PartialOrd;
/// 选择算法
///
/// # Examples
///
/// ```
/// use quickselect::quick_select;
/// let mut test_arr = vec![65.0, 28.0, 59.0, 33.0, 21.0, 56.0, 22.0, 95.0, 50.0, 12.0, 90.0, 53.0, 28.0, 77.0, 39.0];
/// let length = test_arr.len();
/// quick_select(&mut test_arr,8,0,length-1,true);
/// assert_eq!(test_arr,vec![39.0, 28.0, 28.0, 33.0, 21.0, 12.0, 22.0, 50.0, 53.0, 56.0, 59.0, 65.0, 90.0, 77.0, 95.0]);
/// ```
pub fn quick_select<N: Copy + PartialOrd>(
    arr: &mut Vec<N>,
    k: usize,
    mut left: usize,
    mut right: usize,
    is_left_smallest: bool,
) {
    // 比较函数 根据条件动态绑定回调函数
    let compare: fn(N, N) -> i32;
    if is_left_smallest {
        compare = left_smallest_compare;
    } else {
        compare = left_bigest_compare;
    }
    while right > left {
        if right - left > 600 {
            let n = (right - left + 1) as f32;
            let m = (k - left + 1) as f32;
            let z = n.ln();
            let s = (2.0 * z / 3.0).exp() * 0.5;
            let a: f32 = if (m - n / 2.0) < 0.0 { -1.0 } else { 1.0 };
            let sd = (z * s * (n - s) / n).sqrt() * a * 0.5;
            let mut new_left = ((k as f32) - m * s / n + sd).floor() as usize;
            new_left = if left > new_left { left } else { new_left };
            let mut new_right = ((k as f32)+ (n - m) * s / n + sd).floor() as usize;
            new_right = if right < new_right { right } else { new_right };
            quick_select(arr, k, new_left, new_right, is_left_smallest);
        }
        let t: N = arr[k];
        let mut i = left;
        let mut j = right;
        arr.swap(left, k);

        if compare(arr[right], t) > 0 {
            arr.swap(left, right);
        }
        while i < j {
            arr.swap(i, j);
            i = i + 1;
            j = j - 1;
            while compare(arr[i], t) < 0 {
                i = i + 1;
            }
            while compare(arr[j], t) > 0 {
                j = j - 1;
            }
        }
        if arr[left] == t {
            arr.swap(left, j);
        } else {
            j = j + 1;
            arr.swap(j, right);
        }

        if j <= k {
            left = j + 1;
        }
        if k <= j {
            right = j - 1;
        }
    }
}

fn left_smallest_compare<N: PartialOrd>(a: N, b: N) -> i32 {
    if a < b {
        -1
    } else {
        if a > b {
            1
        } else {
            0
        }
    }
}

fn left_bigest_compare<N: PartialOrd>(a: N, b: N) -> i32 {
    if a < b {
        1
    } else {
        if a > b {
            -1
        } else {
            0
        }
    }
}
