pub fn traverse_and_count_trees(world: World, slope: Coordinate) -> usize {
    _traverse_and_count_trees(world, slope, 0)
}

fn _traverse_and_count_trees(world: World, slope: Coordinate, ntrees: usize) -> usize {
    match travel(&world, &slope) {
        None => ntrees,
        Some(position) => {
            let w = World {
                map: world.map,
                position,
            };

            let found_tree = encountered_tree(&w);

            _traverse_and_count_trees(w, slope, ntrees + found_tree as usize)
        }
    }
}

fn travel(world: &World, slope: &Coordinate) -> Option<Coordinate> {
    let x = (world.position.x + slope.x) % world.map[0].len();
    let y = world.position.y + slope.y;

    match y < world.map.len() {
        true => Some(Coordinate { x, y }),
        false => None,
    }
}

fn encountered_tree(world: &World) -> bool {
    world.map[world.position.y]
        .chars()
        .nth(world.position.x)
        .map_or(false, |c| c == '#')
}

pub struct World<'a> {
    pub map: &'a Vec<String>,
    pub position: Coordinate,
}

pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_traverse_and_count_trees() {
        let map: Vec<String> = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(
            super::traverse_and_count_trees(
                super::World {
                    map: &map,
                    position: super::Coordinate { x: 0, y: 0 }
                },
                super::Coordinate { x: 3, y: 1 }
            ),
            7
        );
    }
}
