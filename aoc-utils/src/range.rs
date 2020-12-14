use std::iter::Iterator;

pub struct Range {
    pub step: usize,
}
pub struct RangeIntoIterator {
    last: usize,
    step: usize,
}

impl IntoIterator for Range {
    type Item = usize;
    type IntoIter = RangeIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        RangeIntoIterator {
            last: 0,
            step: self.step,
        }
    }
}

impl Iterator for RangeIntoIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let current = self.last;

        self.last += self.step;

        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::Range;

    #[test]
    fn test_range() {
        let r: Vec<usize> = Range { step: 7 }.into_iter().take(4).collect();

        assert_eq!(r, vec![0, 7, 14, 21]);
    }
}
