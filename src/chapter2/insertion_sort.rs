fn insertion_sort<T: Copy + PartialEq + PartialOrd>(input: &mut Vec<T>) -> &Vec<T> {
    for j in 2..input.len() {
        let key = input[j];
        let mut i = j - 1;
        while i > 0 && input[i] > key {
            input[i + 1] = input[i];
            i -= 1;
        }
        input[i + 1] = key;
    }
    return input
}