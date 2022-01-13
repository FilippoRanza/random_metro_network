use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct FloatMatrix<T> {
    matrix: Vec<FloatTable<T>>,
}

impl<T: Clone> FloatMatrix<T> {
    pub fn new(size: usize) -> Self {
        let matrix = vec![FloatTable::new(); size];
        Self { matrix }
    }

    pub fn insert(&mut self, i: usize, k: f64, v: T) {
        self.matrix[i].insert(k, v);
    }

    pub fn get(&self, i: usize, k: f64) -> Option<&T> {
        self.matrix[i].get(k)
    }
}

#[derive(Clone, Debug)]
pub struct FloatTable<T> {
    table: HashMap<u64, T>,
}

impl<T> FloatTable<T> {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: f64, v: T) {
        let k = k.to_bits();
        self.table.insert(k, v);
    }

    pub fn get(&self, k: f64) -> Option<&T> {
        let k = k.to_bits();
        self.table.get(&k)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_float_table() {
        let mut table = FloatTable::new();

        table.insert(0.566, 'a');
        table.insert(0.567, 'b');
        table.insert(0.568, 'c');
        table.insert(0.569, 'd');

        assert_eq!(table.get(0.566), Some(&'a'));
        assert_eq!(table.get(0.567), Some(&'b'));
        assert_eq!(table.get(0.568), Some(&'c'));
        assert_eq!(table.get(0.569), Some(&'d'));
        assert_eq!(table.get(0.5699), None);
    }

    #[test]
    fn test_float_matrix() {
        let mut matrix = FloatMatrix::new(4);

        matrix.insert(0, 0.4, 'a');
        matrix.insert(0, 0.41, 'b');
        matrix.insert(1, 0.48, 'a');

        assert_eq!(matrix.get(0, 0.4), Some(&'a'));
        assert_eq!(matrix.get(0, 0.41), Some(&'b'));
        assert_eq!(matrix.get(0, 0.42), None);
        assert_eq!(matrix.get(1, 0.48), Some(&'a'));
        assert_eq!(matrix.get(2, 0.42), None);
        assert_eq!(matrix.get(3, 0.42), None);
    }
}
