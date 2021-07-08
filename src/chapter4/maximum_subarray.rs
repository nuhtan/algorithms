pub fn find_maximum_subarray(array: &Vec<i32>, low: usize, high: usize) -> (usize, usize, i32) {
    if high == low {
        (low, high, array[low])
    } else {
        let mid = (low + high) / 2;
        let (left_low, left_high, left_sum) = find_maximum_subarray(&array, low, mid);
        let (right_low, right_high, right_sum) = find_maximum_subarray(&array, mid + 1, high);
        let (cross_low, cross_high, cross_sum) = find_max_crossing_subarray(&array, low, mid, high);
        if left_sum >= right_sum && left_sum >= cross_sum {
            (left_low, left_high, left_sum)
        } else if right_sum >= left_sum && right_sum >= cross_sum {
            (right_low, right_high, right_sum)
        } else {
            (cross_low, cross_high, cross_sum)
        }
    }
}

fn find_max_crossing_subarray(
    array: &Vec<i32>,
    low: usize,
    mid: usize,
    high: usize,
) -> (usize, usize, i32) {
    let mut left_sum = (0, true);
    let mut sum = 0;
    let mut max_left = 0;
    for i in mid..low {
        sum += array[i];
        if sum > left_sum.0 || left_sum.1 {
            left_sum.1 = false;
            left_sum.0 = sum;
            max_left = i;
        }
    }
    let mut right_sum = (0, true);
    let mut max_right = 0;
    sum = 0;
    for j in mid + 1..high {
        sum += array[j];
        if sum > right_sum.0 || right_sum.1 {
            right_sum.1 = false;
            right_sum.0 = sum;
            max_right = j;
        }
    }
    (max_left, max_right, left_sum.0 + right_sum.0)
}
