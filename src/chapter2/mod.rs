//! # Chapter 2: Getting Started
//! This chapter is mostly explanation on how the book will explain things and how to analyze and understand algorithms. This chapter only features a single algorithm, insertion sort.

/// The sample algorithm used to explain concepts of pseudocode conventions and algorithmic analysis.
pub mod insertion_sort {
    //TODO break the lines for readability on github
    /// # Section 2.1: Insertion Sort
    ///
    /// Insertion sort is an efficient sort for sorting small amounts of elements.
    /// Behaviorally it works in a similar way to how most humans sort. Insertion sort
    /// does the sorting in place and is quite simple, not requiring any helper functions
    /// or more complex operations or accompanying arrays.
    ///
    /// A few notes:
    /// - Our loop starts at 1 as we compare the value at the index with the prior value not the following value.
    /// - This sort has a time complexity of O(n<sup>2</sup>).
    ///
    /// Insertion sort loops through each value of the array starting from the front and checks if the value in front of it is of a lower value. If the value is lower, the elements are swapped, this happens until the value is at the front of the array or the value in front of it lower. This process repeats until the entire array has been traversed through the first loop.
    ///
    /// The generic T has the Copy trait so that we can limit the types to more straight forward copyable types.
    /// PartialEq and PartialOrd are included for the needed comparisons of the generic types.
    ///
    /// # Arguments:
    /// - 'input' - The array that is being sorted.
    ///
    /// # Returns:
    /// This method has no return type as it modifies the input in place.
    ///
    /// # Special Notes:
    /// - In and effort to follow the books usage of arrays that start at 1 al of the logic of the function has been shifted over by one.
    pub fn sort<T: Copy + PartialEq + PartialOrd>(input: &mut Vec<T>) {
        for j in 2..input.len() + 1 {
            let key = input[j - 1];
            let mut i = j - 1;
            while i > 0 && input[i - 1] > key {
                input[i] = input[i - 1];
                i -= 1;
            }
            input[i] = key;
        }
    }

    #[cfg(test)]
    mod tests {

        /// A very basic test to ensure that an array of length one does not get modified.
        #[test]
        fn test_single_length_array() {
            let mut vec = vec![5];            super::sort(&mut vec);
            assert_eq!(vec, vec![5]);
        }

        #[test]
        fn test_sorted_array() {
            let mut vec = vec![1, 3, 6, 7, 45, 87];
            let target = vec![1, 3, 6, 7, 45, 87];
            super::sort(&mut vec);
            assert_eq!(vec, target);
        }

        #[test]
        fn test_reverse_sorted_array() {
            let mut vec = vec![754, 45, 23, 22, 5];
            let target = vec![5, 22, 23, 45, 754];
            super::sort(&mut vec);
            assert_eq!(vec, target);
        }

        #[test]
        fn test_array_with_duplicates() {
            let mut vec = vec![5, 45, 22, 22, 5];
            let target = vec![5, 5, 22, 22, 45];
            super::sort(&mut vec);
            assert_eq!(vec, target);
        }

        /// This test generates 10 random arrays and sorts them, the sort is compared against the std sort for vectors.
        #[test]
        fn test_random_array() {
            use rand::Rng;
            for _ in 0..10 {
                let mut arr = [0; 20];
                rand::thread_rng().fill(&mut arr);
                let mut vec = arr.to_vec();
                let mut target = vec.clone();
                super::sort(&mut vec);
                target.sort();
                assert_eq!(vec, target);
            }
        }
    }
}
