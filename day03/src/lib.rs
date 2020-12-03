pub fn traverse_and_count_trees(world: &World) -> usize {
    0
}

fn travel(world: &World, slope: Coordinate) -> Coordinate {
    Coordinate { x: 0, y: 0 }
}

fn encountered_tree(world: &World) -> bool {
    false
}

pub struct World {
    pub map: Vec<String>,
    pub position: Coordinate,
}

pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
