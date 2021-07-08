fn parent(i: usize) -> usize {
    i / 2
}

fn left(i: usize) -> usize {
    2 * i
}

fn right(i: usize) -> usize {
    2 * i + 1
}

struct Heap<'a, T: Copy + PartialEq + PartialOrd> {
    pub storage: &'a mut Vec<T>,
    pub heap_size: usize,
}

impl<'a, T: Copy + PartialEq + PartialOrd> Heap<'a, T> {
    pub fn new(array: &'a mut Vec<T>) -> Self {
        Heap {
            storage: array,
            heap_size: 0
        }
    }
}

fn max_heapify<T: Copy + PartialEq + PartialOrd>(heap: &mut Heap<T>, i: usize) {
    let l = left(i);
    let r = right(i);
    let mut largest;
    if l <= heap.heap_size && heap.storage[l] > heap.storage[i] {
        largest = l;
    } else {
        largest = i;
    }
    if r <= heap.heap_size && heap.storage[r] > heap.storage[largest] {
        largest = r;
    }
    if largest != i {
        let temp = heap.storage[i];
        heap.storage[i] = heap.storage[largest];
        heap.storage[largest] = temp;
        max_heapify(heap, largest)
    }
}

fn build_max_heap<T: Copy + PartialEq + PartialOrd>(array: &mut Vec<T>) -> Heap<T> {
    let mut heap = Heap::new(array);
    heap.heap_size = heap.storage.len();
    for i in heap.storage.len() / 2..1 {
        max_heapify(&mut heap, i)
    }
    heap
}

fn heapsort<T: Copy + PartialEq + PartialOrd>(array: &mut Vec<T>) {
    let heap = build_max_heap(array);
    for i in array.len()..2 {
        let temp = array[1];
        array[1] = array[i];
        array[i] = temp;
    }
}
