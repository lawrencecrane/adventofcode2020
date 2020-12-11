use std::iter::Iterator;

pub struct Tribonacci {}
pub struct TribonacciIntoIterator {
    last: [usize; 3],
}

impl IntoIterator for Tribonacci {
    type Item = usize;
    type IntoIter = TribonacciIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        TribonacciIntoIterator { last: [0, 1, 1] }
    }
}

impl Iterator for TribonacciIntoIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let current = self.last[0];

        self.last = [self.last[1], self.last[2], self.last.iter().sum()];

        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::Tribonacci;

    #[test]
    fn test_tribonacci() {
        let tri: Vec<usize> = Tribonacci {}.into_iter().take(11).collect();

        assert_eq!(tri, vec![0, 1, 1, 2, 4, 7, 13, 24, 44, 81, 149]);
    }
}
