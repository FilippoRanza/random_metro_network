use std::slice::Iter;

pub struct LineSegmentItator<'a, T> {
    line: Iter<'a, T>,
    prev: Option<&'a T>,
}

impl<'a, T> LineSegmentItator<'a, T> {
    pub fn new(line: &'a [T]) -> Self {
        let mut line = line.iter();
        let prev = line.next();
        Self { line, prev }
    }
}

impl<'a, T> Iterator for LineSegmentItator<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.line.next()?;
        let prev = self.prev.unwrap();
        self.prev = Some(curr);
        Some((prev, curr))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_odd_len_line() {
        let lines = [1, 2, 3];
        let mut iter = LineSegmentItator::new(&lines);
        assert_eq!(iter.next(), Some((&1, &2)));
        assert_eq!(iter.next(), Some((&2, &3)));
        assert_eq!(iter.next(), None);

        let lines = [1, 2, 3, 4, 5];
        let mut iter = LineSegmentItator::new(&lines);
        assert_eq!(iter.next(), Some((&1, &2)));
        assert_eq!(iter.next(), Some((&2, &3)));
        assert_eq!(iter.next(), Some((&3, &4)));
        assert_eq!(iter.next(), Some((&4, &5)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_even_len_line() {
        let lines = [1, 2];
        let mut iter = LineSegmentItator::new(&lines);
        assert_eq!(iter.next(), Some((&1, &2)));
        assert_eq!(iter.next(), None);

        let lines = [1, 2, 3, 4];
        let mut iter = LineSegmentItator::new(&lines);
        assert_eq!(iter.next(), Some((&1, &2)));
        assert_eq!(iter.next(), Some((&2, &3)));
        assert_eq!(iter.next(), Some((&3, &4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_empty_line() {
        let lines: [usize; 0] = [];
        let mut iter = LineSegmentItator::new(&lines);
        assert_eq!(iter.next(), None);

        let lines = [1];
        let mut iter = LineSegmentItator::new(&lines);
        assert_eq!(iter.next(), None);
    }
}
