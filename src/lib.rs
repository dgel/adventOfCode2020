use std::ops::{Index, IndexMut};

pub struct Mat<T: Clone> {
    data: Vec<T>,
    width: usize,
    height: usize,
}


impl<T:Clone> Mat<T> {
    pub fn new(width: usize, height: usize, val: T) -> Mat<T> {
        Mat {
            data: vec![val; width * height],
            width,
            height,
        }
    }
}

impl<T:Clone> Index<(usize, usize)> for Mat<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &T {
        if index.0 < self.width && index.1 < self.height {
            &self.data[index.0*self.width + index.1]
        } else {
            panic!("out of bounds!");
        }
    }
}

impl<T:Clone> IndexMut<(usize, usize)> for Mat<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        if index.0 < self.width && index.1 < self.height {
            &mut self.data[index.0*self.width + index.1]
        } else {
            panic!("out of bounds!");
        }
    }
}

