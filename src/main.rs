fn main() {
    println!(".\n.\nThis is a test-only project, run `cargo test` to see the test results\n.\n.");
}
const MIN_MERGE: usize = 32;

fn calc_min_run(mut n: usize) -> usize {
    let mut r = 0;
    while n >= MIN_MERGE {
        r |= n & 1;
        n >>= 1;
    }
    n + r
}

fn inser_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn insertion_sort<T: Ord>(arr: &mut [T], left: usize, right: usize) {
    for i in (left + 1)..=right {
        let mut j = i;
        while j > left && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn merge<T: Ord + Copy>(arr: &mut [T], left: usize, mid: usize, right: usize) {
    let len1 = mid - left + 1;
    let len2 = right - mid;
    let mut left_arr = vec![arr[left]; len1];
    let mut right_arr = vec![arr[mid + 1]; len2];
    for i in 0..len1 {
        left_arr[i] = arr[left + i];
    }
    for i in 0..len2 {
        right_arr[i] = arr[mid + 1 + i];
    }
    let mut i = 0;
    let mut j = 0;
    let mut k = left;
    while i < len1 && j < len2 {
        if left_arr[i] <= right_arr[j] {
            arr[k] = left_arr[i];
            i += 1;
        } else {
            arr[k] = right_arr[j];
            j += 1;
        }
        k += 1;
    }
    while i < len1 {
        arr[k] = left_arr[i];
        i += 1;
        k += 1;
    }
    while j < len2 {
        arr[k] = right_arr[j];
        j += 1;
        k += 1;
    }
}

fn tim_sort<T: Ord + Copy>(arr: &mut [T]) {
    let n = arr.len();
    let min_run = calc_min_run(n);

    for start in (0..n).step_by(min_run) {
        let end = std::cmp::min(start + min_run - 1, n - 1);
        insertion_sort(arr, start, end);
    }

    let mut size = min_run;
    while size < n {
        for left in (0..n).step_by(2 * size) {
            let mid = std::cmp::min(left + size - 1, n - 1);
            let right = std::cmp::min(left + 2 * size - 1, n - 1);
            if mid < right {
                merge(arr, left, mid, right);
            }
        }
        size *= 2;
    }
}

#[allow(dead_code)]
fn sort<T: Ord + Clone + Copy>(slice: &mut [T]) {
    if slice.len() <= 20 {
        inser_sort(slice);
    } else {
        tim_sort(slice);
    }
}

#[cfg(test)]
mod sort_test {
    use super::*;
    #[test]
    fn test_inser() {
        let mut a = [2, 6, 1, 8, 3, 23, 8, 1];
        sort(&mut a);
        println!("{:?}", a);
        assert_eq!(a, [1, 1, 2, 3, 6, 8, 8, 23]);
    }

    #[test]
    fn test_insertion_sort_empty() {
        let mut values: [i32; 0] = [];
        sort(&mut values);
        assert_eq!(values, [])
    }

    #[test]
    fn test_insertion_sort_one() {
        let mut values = [1];
        sort(&mut values);
        assert_eq!(values, [1]);
    }

    #[test]
    fn test_tim_sort_empty() {
        let mut arr: [i32; 0] = [];
        sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_tim_sort_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_tim_sort_reverse_sorted() {
        let mut arr = [5, 4, 3, 2, 1];
        sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_duplicate_values() {
        let mut arr = [5, 2, 8, 2, 1, 5];
        sort(&mut arr);
        assert_eq!(arr, [1, 2, 2, 5, 5, 8]);
    }

    #[test]
    fn test_sort_large_array() {
        let mut arr = [
            21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, -1, 6, 5, 4, 3, 2, 1, -2,
        ];
        sort(&mut arr);
        assert_eq!(
            arr,
            [-2, -1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]
        );
    }

    #[test]
    fn test_sort_large_array_2() {
        let mut arr = [
            -123, 12, 5, 10, 0, -13, 42, 19, 12, 2, -23, -2, 38, 32, -23, 234, 144, -657, 32, 1, 31,
        ];
        sort(&mut arr);
        assert_eq!(
            arr,
            [
                -657, -123, -23, -23, -13, -2, 0, 1, 2, 5, 10, 12, 12, 19, 31, 32, 32, 38, 42, 144,
                234
            ]
        );
    }

    #[test]
    fn test_sort_large_array_3() {
        let mut arr = [
            12, 75, 13, 897, 123, 625, 23, 4, -12, 3, -12, -54, -67, -112, -123, 1235, 123, 346,
            989, 5, 4, 355, 342, 67, 23, 56, 12,
        ];
        sort(&mut arr);
        assert_eq!(
            arr,
            [
                -123, -112, -67, -54, -12, -12, 3, 4, 4, 5, 12, 12, 13, 23, 23, 56, 67, 75, 123,
                123, 342, 346, 355, 625, 897, 989, 1235
            ]
        );
    }
    #[test]
    fn test_sort_random() {
        let mut arr = [33, 65, 23, 54, 12, 76, 44, 89];
        sort(&mut arr);
        assert_eq!(arr, [12, 23, 33, 44, 54, 65, 76, 89]);
    }

    #[test]
    fn test_sort_negative_numbers() {
        let mut arr = [-2, 5, -1, 0, -7, 3];
        sort(&mut arr);
        assert_eq!(arr, [-7, -2, -1, 0, 3, 5]);
    }
}
