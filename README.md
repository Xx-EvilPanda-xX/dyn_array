# dyn_array - A simple, easy to use N-dimensional dynamic array

A DynArray can be constructed in two ways:

1. DynArray::new - specify dimensions and default value
2. DynArray::new_from_data - specify dimensions and a vector that contains pre initialized data

All indexing is done with std::ops::Index by passing in an array to specify which element is wanted. 
Ex:

let elem = arr[[10, 10, 10]];

DynArray can also be iterated over in a couple of ways. First, the `data` and `data_mut` provide a contiguous
slice of the elements without any regard to the dimensions of the DynArray. The provided slice can then be iterated.
There is also an `Iterator` implemention for DynArray that gives you immutable access to each element while also providing
the associated index.

Ex:

// 20 by 20 integer array
let mut arr = DynArray::new([20, 20], 3);

for i in arr.data_mut() {
    //mutable access to elements, but no indication of current index
}

for (index, elem) in &arr {
    //immutable access to elements and the current index
    let [x, y] = index;
}