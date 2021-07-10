//! # Chapter 4: Divide-and-Conquer
//! This chapter introduces the concept of dividing a problem up into smaller sections and working on them separately, sometimes followed by then putting the results of the smaller sections back together.

/// The first example given for divide and conquer algorithms concerning finding the subarray that contains the greatest summation.
///
/// To find the maximum subarray there are exactly three scenarios that need to be considered:
/// - The maximum subarray is within the first half of the array and does not cross the middle.
/// - The maximum subarray is within the latter half of the array and does not cross the middle.
/// - The maximum subarray crossed the midpoint of the array at some point.
pub mod maximum_subarray {

    /// This function finds the largest summation within a subarray of the input array.
    ///
    /// This is done by recursively dividing the array in half until a subarray is found from the [find_max_crossing_subarray()] function or there is just a single element in the array.
    ///
    /// # Arguments:
    /// - 'array'   - The array that is analyzed.
    /// - 'low'     - The lowest index of the array that should be analyzed.
    /// - 'mid'     - The midpoint of the section of the array being searched.
    /// - 'high'    - The highest index of the array that should be analyzed.
    ///
    /// # Returns:
    /// This method returns a tuple as such (lowest index of subarray, highest index of subarray, sum of subarray).
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
    
    /// This function finds the largest subarray that includes the value at the midpoint of the array.
    ///
    /// This is done by finding the max subarray from the midpoint towards either the start or end of the array.
    ///
    /// The left_sum and right_sum being tuples with a boolean values is to replicate the ability to compare the number to negative infinity. As this comparison is only done a single time for each there is no need to create a new struct for this behavior.
    ///
    /// # Arguments:
    /// - 'array'   - The array that is analyzed.
    /// - 'low'     - The lowest index of the array that should be analyzed.
    /// - 'mid'     - The midpoint of the section of the array being searched.
    /// - 'high'    - The highest index of the array that should be analyzed.
    ///
    /// # Returns:
    /// This method returns a tuple as such (lowest index of subarray, highest index of subarray, sum of subarray).
    pub fn find_max_crossing_subarray(
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
}

/// A more efficient matrix multiplication algorithm, standard matrix multiplication included as well.
///
/// More documentation needed.
pub mod strassens_algorithm;