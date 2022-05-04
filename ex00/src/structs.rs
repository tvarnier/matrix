pub struct Vector<K>
where
    K: Copy,
{
    array: Vec<K>,
    pub size: usize,
}

struct Matrix<K> {
    a: Vec<Vec<K>>,
}

impl<
        K: std::fmt::Debug
            + std::ops::Add<Output = K>
            + std::ops::Sub<Output = K>
            + std::ops::Mul<Output = K>,
    > Vector<K>
where
    K: Copy,
{
    pub fn new(array_values: Vec<K>) -> Vector<K> {
        Vector {
            size: array_values.len(),
            array: array_values,
        }
    }

    pub fn print(&self) {
        println!("{:?}", self.array);
    }

    pub fn add(&mut self, v: &Vector<K>) {
        if self.size == v.size {
            for id in 0..self.array.len() {
                self.array[id] = self.array[id] + v.array[id];
            }
        }
    }
    pub fn sub(&mut self, v: &Vector<K>) {
        if self.size == v.size {
            for id in 0..self.array.len() {
                self.array[id] = self.array[id] - v.array[id];
            }
        }
    }
    pub fn scl(&mut self, a: K) {
        for id in 0..self.array.len() {
            self.array[id] = self.array[id] * a;
        }
    }
    // fn add(&mut self, v: &Vector<K>);
    // fn sub(&mut self, v: &Vector<K>);
    // fn scl(&mut self, a: K);
}

impl<K> Matrix<K> {
    // fn add(&mut self, v: &Matrix<K>);
    // fn sub(&mut self, v: &Matrix<K>);
    // fn scl(&mut self, a: K);
}
