/// Section 2.1: Insertion Sort
///
/// Insertion sort is an efficient sort for sorting small amounts of elements. Behaviorally it works in a similar way to how most humans sort. Insertion sort does the sorting in place and is quite simple, not requiring any helper functions or more complex operations.

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
