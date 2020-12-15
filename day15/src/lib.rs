use std::collections::HashMap;

pub fn play_memory_game(starting: &Vec<usize>, until: usize) -> usize {
    MemoryGame::new(starting)
        .into_iter()
        .take(until)
        .last()
        .unwrap_or(0)
}

impl Iterator for MemoryGame {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        None
    }
}

impl MemoryGame {
    fn new(starting: &Vec<usize>) -> Self {
        Self {
            history: MemoryGame::initialize_history(starting),
        }
    }

    fn initialize_history(starting: &Vec<usize>) -> History {
        let mut history: History = HashMap::new();

        starting.iter().enumerate().for_each(|(i, x)| {
            history.insert(*x, (Some(i), None));
        });

        history
    }
}

struct MemoryGame {
    history: History,
}

type History = HashMap<usize, (Option<usize>, Option<usize>)>;

#[cfg(test)]
mod tests {

    #[test]
    fn test_play_memory_game() {
        assert_eq!(super::play_memory_game(&vec![0, 3, 6], 2020), 436);
        assert_eq!(super::play_memory_game(&vec![1, 3, 2], 2020), 1);
        assert_eq!(super::play_memory_game(&vec![2, 1, 3], 2020), 10);
        assert_eq!(super::play_memory_game(&vec![1, 2, 3], 2020), 27);
        assert_eq!(super::play_memory_game(&vec![2, 3, 1], 2020), 78);
        assert_eq!(super::play_memory_game(&vec![3, 2, 1], 2020), 438);
        assert_eq!(super::play_memory_game(&vec![3, 1, 2], 2020), 1836);
    }
}
