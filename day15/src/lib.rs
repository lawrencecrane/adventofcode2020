use std::collections::HashMap;

pub fn play_memory_game(starting: &Vec<usize>, until: usize) -> usize {
    MemoryGame::new(starting)
        .into_iter()
        .take(until - starting.len())
        .last()
        .unwrap_or(0)
}

impl Iterator for MemoryGame {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.nturn += 1;

        self.last = match self.history.get(&self.last) {
            Some((Some(_), snd)) if snd.is_none() => 0,
            Some((Some(fst), Some(snd))) => *snd - *fst,
            _ => panic!(""),
        };

        self.insert_to_history(self.nturn, self.last);

        Some(self.last)
    }
}

impl MemoryGame {
    fn new(starting: &Vec<usize>) -> Self {
        let (history, last) = MemoryGame::initialize_history(starting);

        Self {
            history,
            last,
            nturn: starting.len(),
        }
    }

    fn initialize_history(starting: &Vec<usize>) -> (History, usize) {
        let mut history: History = HashMap::new();

        starting.iter().enumerate().for_each(|(i, x)| {
            MemoryGame::_insert_to_history(&mut history, i + 1, *x);
        });

        (history, *starting.last().unwrap())
    }

    fn insert_to_history(&mut self, nturn: usize, value: usize) {
        MemoryGame::_insert_to_history(&mut self.history, nturn, value);
    }

    fn _insert_to_history(history: &mut History, nturn: usize, value: usize) {
        match history.get_mut(&value) {
            None => {
                history.insert(value, (Some(nturn), None));
            }
            Some((Some(_), snd)) if snd.is_none() => {
                *snd = Some(nturn);
            }
            Some((Some(fst), Some(snd))) => {
                *fst = *snd;
                *snd = nturn;
            }
            _ => (),
        };
    }
}

struct MemoryGame {
    history: History,
    nturn: usize,
    last: usize,
}

type History = HashMap<usize, (Option<usize>, Option<usize>)>;

#[cfg(test)]
mod tests {

    #[test]
    fn test_play_memory_game_2020th() {
        assert_eq!(super::play_memory_game(&vec![0, 3, 6], 2020), 436);
        assert_eq!(super::play_memory_game(&vec![1, 3, 2], 2020), 1);
        assert_eq!(super::play_memory_game(&vec![2, 1, 3], 2020), 10);
        assert_eq!(super::play_memory_game(&vec![1, 2, 3], 2020), 27);
        assert_eq!(super::play_memory_game(&vec![2, 3, 1], 2020), 78);
        assert_eq!(super::play_memory_game(&vec![3, 2, 1], 2020), 438);
        assert_eq!(super::play_memory_game(&vec![3, 1, 2], 2020), 1836);
    }

    #[test]
    fn test_play_memory_game_30000000th() {
        assert_eq!(super::play_memory_game(&vec![0, 3, 6], 30000000), 175594);
        assert_eq!(super::play_memory_game(&vec![1, 3, 2], 30000000), 2578);
        assert_eq!(super::play_memory_game(&vec![2, 1, 3], 30000000), 3544142);
        assert_eq!(super::play_memory_game(&vec![1, 2, 3], 30000000), 261214);
        assert_eq!(super::play_memory_game(&vec![2, 3, 1], 30000000), 6895259);
        assert_eq!(super::play_memory_game(&vec![3, 2, 1], 30000000), 18);
        assert_eq!(super::play_memory_game(&vec![3, 1, 2], 30000000), 362);
    }
}
