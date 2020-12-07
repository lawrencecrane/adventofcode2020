use std::collections::HashMap;
use std::hash::Hash;
use std::iter::Iterator;

impl<I: Iterator> AocIterator for I {}

impl<I> Iterator for TakeEveryNth<I>
where
    I: Iterator,
    I::Item: Hash + Eq + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.underlying.next() {
            let rank = self.ranks.entry(x.clone()).or_insert(0);

            *rank += 1;

            if *rank == self.nth {
                return Some(x);
            }
        }

        None
    }
}

pub trait AocIterator: Iterator {
    fn take_every_nth(self, nth: usize) -> TakeEveryNth<Self>
    where
        Self: Sized,
        Self::Item: Hash + Eq + Clone,
    {
        TakeEveryNth {
            nth,
            ranks: HashMap::new(),
            underlying: self,
        }
    }
}

pub struct TakeEveryNth<I>
where
    I: Iterator,
{
    nth: usize,
    ranks: HashMap<I::Item, usize>,
    underlying: I,
}

#[cfg(test)]
mod tests {
    use super::AocIterator;

    #[test]
    fn test_duplicates_of_2() {
        assert_eq!(vec![1, 1, 1].iter().take_every_nth(2).count(), 1);

        assert_eq!(vec![1, 2, 1].iter().take_every_nth(2).count(), 1);

        assert_eq!(vec![1, 2, 3, 3, 3, 5].iter().take_every_nth(2).count(), 1);

        assert_eq!(
            vec![1, 2, 2, 3, 3, 3, 5].iter().take_every_nth(2).count(),
            2
        );

        assert_eq!(
            vec![1, 2, 2, 3, 3, 3, 5, 6, 7, 8, 8, 8, 8]
                .iter()
                .take_every_nth(2)
                .count(),
            3
        );
    }

    #[test]
    fn test_duplicates_of_3() {
        assert_eq!(vec![1, 1, 1].iter().take_every_nth(3).count(), 1);

        assert_eq!(vec![1, 2, 1].iter().take_every_nth(3).count(), 0);
    }
}
