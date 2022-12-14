use std::iter::Iterator;
use std::iter::IntoIterator;
use std::ops::{Index, IndexMut};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct DynArray<T, const D: usize> {
    dims: [usize; D],
    data: Box<[T]>,
}

impl<T: Clone, const D: usize> Clone for DynArray<T, D> {
    fn clone(&self) -> Self {
        DynArray {
            dims: self.dims,
            data: self.data.clone(),
        }
    }
}

impl<T: Clone, const D: usize> DynArray<T, D> {
    pub fn new(dims: [usize; D], x: T) -> Self {
        if dims.len() == 0 {
            panic!("cannot have an array with 0 dimensions");
        }

        let mut vec_len = 1;

        for dim in dims {
            vec_len *= dim;
        }

        Self {
            dims,
            data: vec![x; vec_len].into_boxed_slice()
        }
    }

    pub fn new_from_data(dims: [usize; D], data: Vec<T>) -> Self {
        if dims.len() == 0 {
            panic!("cannot have an array with 0 dimensions");
        }

        if data.len() != dims.iter().product() {
            panic!("Vec len not equal to dimensions");
        }

        Self {
            dims,
            data: data.into_boxed_slice(),
        }
    }
}

impl<T, const D: usize> DynArray<T, D> {
    pub fn dims(&self) -> &[usize] {
        &self.dims
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T, D> {
        self.into_iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T, D> {
        self.into_iter()
    }
}

impl<T> DynArray<T, 1> {
    pub fn width(&self) -> usize {
        self.dims[0]
    }
}

impl<T> DynArray<T, 2> {
    pub fn width(&self) -> usize {
        self.dims[0]
    }

    pub fn height(&self) -> usize {
        self.dims[1]
    }
}

impl<T> DynArray<T, 3> {
    pub fn width(&self) -> usize {
        self.dims[0]
    }

    pub fn height(&self) -> usize {
        self.dims[1]
    }

    pub fn depth(&self) -> usize {
        self.dims[2]
    }
}

impl<T, const D: usize> Index<[usize; D]> for DynArray<T, D> {
    type Output = T;
    fn index(&self, index: [usize; D]) -> &Self::Output {
        &self.data[get_index(&self.dims, &index)]
    }
}

impl<T, const D: usize> IndexMut<[usize; D]> for DynArray<T, D> {
    fn index_mut(&mut self, index: [usize; D]) -> &mut Self::Output {
        &mut self.data[get_index(&self.dims, &index)]
    }
}

fn get_index(dims: &[usize], index: &[usize]) -> usize {
    if !check_index(dims, index) {
        panic!("index out of bounds");
    }

    let mut idx = 0;

    for (i, dim) in index.iter().enumerate() {
        let mul: usize = dims[..i].iter().product();
        idx += dim * mul;
    }

    idx
}

fn check_index(dims: &[usize], index: &[usize]) -> bool {
    assert_eq!(dims.len(), index.len());

    for (i, dim) in index.iter().zip(dims) {
        if i >= dim {
            return false;
        }
    }

    true
}

pub struct Iter<'a, T, const D: usize> {
    arr: &'a DynArray<T, D>,
    index: [usize; D],
}

pub struct IterMut<'a, T, const D: usize> {
    arr: &'a mut DynArray<T, D>,
    index: [usize; D],
}

pub struct IterOwned<T, const D: usize> {
    arr: DynArray<T, D>,
    index: [usize; D],
}

impl<'a, T, const D: usize> Iterator for Iter<'a, T, D> {
    type Item = ([usize; D], &'a T);
    
    fn next(&mut self) -> Option<Self::Item> {
        let dims = self.arr.dims();
        let index = &mut self.index;
        if !check_index(dims, index) {
            return None;
        }

        let ret = Some((*index, self.arr.index(*index)));
        next_index(index, dims);
        ret
    }
}

impl<'a, T, const D: usize> Iterator for IterMut<'a, T, D> {
    type Item = ([usize; D], &'a mut T);
    
    fn next(&mut self) -> Option<Self::Item> {
        let index = &mut self.index;
        if !check_index(self.arr.dims(), index) {
            return None;
        }

        let idx = *index;
        let elem = self.arr.index_mut(idx) as *mut T;
        next_index(index, self.arr.dims());
        // SAFETY: return value is still bound by 'a and therfore can't be invalid.
        // Internal ref held by IterMut must not be accessed while a ref is given out
        // by this associated function. Under normal circumstances, this shouldn't be
        // possible as it is a private field.
        Some((idx, unsafe { &mut *elem }))
    }
}

impl<T: Default, const D: usize> Iterator for IterOwned<T, D> {
    type Item = ([usize; D], T);
    
    fn next(&mut self) -> Option<Self::Item> {
        let index = &mut self.index;
        if !check_index(self.arr.dims(), index) {
            return None;
        }
        
        let ret = Some((*index, std::mem::take(self.arr.index_mut(*index))));
        next_index(index, self.arr.dims());
        ret
    }
}

impl<'a, T, const D: usize> IntoIterator for &'a DynArray<T, D> {
    type Item = <Iter<'a, T, D> as Iterator>::Item;
    type IntoIter = Iter<'a, T, D>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            arr: self,
            index: [0; D]
        }
    }
}

impl<'a, T, const D: usize> IntoIterator for &'a mut DynArray<T, D> {
    type Item = <IterMut<'a, T, D> as Iterator>::Item;
    type IntoIter = IterMut<'a, T, D>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            arr: self,
            index: [0; D]
        }
    }
}

impl<T: Default, const D: usize> IntoIterator for DynArray<T, D> {
    type Item = <IterOwned<T, D> as Iterator>::Item;
    type IntoIter = IterOwned<T, D>;

    fn into_iter(self) -> Self::IntoIter {
        IterOwned {
            arr: self,
            index: [0; D]
        }
    }
}

fn next_index(index: &mut [usize], dims: &[usize]) {
    assert_eq!(index.len(), dims.len());

    for i in 0..dims.len() {
        index[i] += 1;
        if index[i] >= dims[i] {
            if i != dims.len() - 1 {
                index[i] = 0;
            }

            continue;
        } else {
            break;
        }
    }
}