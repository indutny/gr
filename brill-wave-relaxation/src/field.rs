use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Field {
    pub width: usize,
    values: Vec<f64>,
}

impl Field {
    pub fn new(width: usize, value: f64) -> Self {
        let size: usize = width * width * width;
        let mut values = Vec::with_capacity(size);

        for _i in 0..size {
            values.push(value);
        }

        Field { width, values }
    }
}

impl Index<(usize, usize, usize)> for Field {
    type Output = f64;

    fn index(&self, index: (usize, usize, usize)) -> &Self::Output {
        let (x, y, z) = index;
        &self.values[z * self.width * self.width + y * self.width + x]
    }
}

impl IndexMut<(usize, usize, usize)> for Field {
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        let (x, y, z) = index;
        &mut self.values[z * self.width * self.width + y * self.width + x]
    }
}
