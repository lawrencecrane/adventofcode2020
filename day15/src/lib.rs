use std::collections::HashMap;

pub fn play_memory_game(starting: &Vec<usize>, until: usize) -> usize {
    let mut history = initialize_history(starting);

    0
}

fn initialize_history(starting: &Vec<usize>) -> HashMap<usize, (Option<usize>, Option<usize>)> {
    let mut history: HashMap<usize, (Option<usize>, Option<usize>)> = HashMap::new();

    starting.iter().enumerate().for_each(|(i, x)| {
        history.insert(*x, (Some(i), None));
    });

    history
}

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
