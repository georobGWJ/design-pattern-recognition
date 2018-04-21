pub mod map_site;  // trait to encompass walls and doors

pub mod abstract_product;  // trait to encompass concrete products

pub mod wall;      // "class" to represent a Wall

pub mod door;      // "class" to represent a Door

pub mod room;      // "class" to represent a Room

pub mod maze;      // "class" to represent a Maze ()

pub mod maze_game; // "class" to represent a MazeGame ()

pub mod maze_factory;  // abstract factory for building mazes


// Define directions for player movement
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn value(&self) -> usize {
        match *self {
            Direction::North => 0 as usize,
            Direction::East  => 1 as usize,
            Direction::South => 2 as usize,
            Direction::West  => 3 as usize,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
