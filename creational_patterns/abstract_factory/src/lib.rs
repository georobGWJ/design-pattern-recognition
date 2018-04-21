pub mod map_site;  // trait to encompass walls and doors

pub mod wall;      // "class" to represent a Wall

pub mod door;      // "class" to represent a Door

pub mod room;      // "class" to represent a Room

pub mod maze;      // "class" to represent a Maze ()

// Define directions for player movement
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn value(&self) -> u32 {
        match *self {
            Direction::North => 0,
            Direction::East  => 1,
            Direction::South => 2,
            Direction::West  => 3,
        }
    }
}

pub fn cheese() {
    // silly function to test that main can see this library
    println!("Cheese sure is good!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
