// Abstract factory for creating mazes.
use abstract_product::*;
use maze::*;

struct MazeFactory {
}

impl MazeFactory {

    fn make_maze() -> Maze {
        unimplemented!();
    }

    fn make_room() -> MazeRoom {
        unimplemented!();
    }

    fn make_wall() -> MazeWall {
        unimplemented!();
    }

    fn make_door() -> MazeDoor {
        unimplemented!();
    }

}
