use std::collections::HashSet;
use std::hash::Hash;
use std::iter::Iterator;

impl<I: Iterator> AocIterator for I {}

impl<I> Iterator for Duplicate<I>
where
    I: Iterator,
    I::Item: Hash + Eq + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.underlying.next() {
            if self.seen.contains(&x) && !self.duplicates.contains(&x) {
                self.duplicates.insert(x.clone());

                return Some(x);
            }

            self.seen.insert(x.clone());
        }

        None
    }
}

pub trait AocIterator: Iterator {
    fn duplicates(self) -> Duplicate<Self>
    where
        Self: Sized,
        Self::Item: Hash + Eq + Clone,
    {
        Duplicate {
            seen: HashSet::new(),
            duplicates: HashSet::new(),
            underlying: self,
        }
    }
}

pub struct Duplicate<I>
where
    I: Iterator,
{
    seen: HashSet<I::Item>,
    duplicates: HashSet<I::Item>,
    underlying: I,
}

#[cfg(test)]
mod tests {
    use super::AocIterator;
    #[test]
    fn test_duplicates() {
        assert_eq!(vec![1, 2, 3, 3, 3, 5].iter().duplicates().count(), 1);
        assert_eq!(vec![1, 2, 2, 3, 3, 3, 5].iter().duplicates().count(), 2);
        assert_eq!(
            vec![1, 2, 2, 3, 3, 3, 5, 6, 7, 8, 8, 8, 8]
                .iter()
                .duplicates()
                .count(),
            3
        );
    }
}
