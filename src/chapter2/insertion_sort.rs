/// # Section 2.1: Insertion Sort
///
/// Insertion sort is an efficient sort for sorting small amounts of elements. Behaviorally it works in a similar way to how most humans sort. Insertion sort does the sorting in place and is quite simple, not requiring any helper functions or more complex operations or accompanying arrays.
///
/// A few notes:
/// - Our loop starts at 1 as we compare the value at the index with the prior value not the following value.
/// - This sort has a time complexity of O(n<sup>2</sup>).
///
/// Insertion sort loops through each value of the array starting from the front and checks if the value in front of it is of a lower value. If the value is lower, the elements are swapped, this happens until the value is at the front of the array or the value in front of it lower. This process repeats until the entire array has been traversed through the first loop.

// The generic T has the Copy trait so that we can limit the types to more straight forward copyable types.
// PartialEq and PartialOrd are included for the needed comparisons of the generic types.
pub fn sort<T: Copy + PartialEq + PartialOrd>(input: &mut Vec<T>) -> &Vec<T> {
    for j in 1..input.len() {
        let key = input[j];
        let mut i = j - 1;
        while i > 0 && input[i] > key {
            input[i + 1] = input[i];
            i -= 1;
        }
        input[i + 1] = key;
    }
    return input;
}
