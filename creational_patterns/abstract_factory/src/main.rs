extern crate abstract_factory;

use abstract_factory::Direction;
use abstract_factory::door::*;
use abstract_factory::wall::*;
use abstract_factory::room::*;
use abstract_factory::map_site::*;


fn main() {
    println!("\nI'm a library driver! ^_^");

    // Make a door, for testing
    let door = Door::new(23, 25);

    // And some walls. A room's not a room without them...
    let wall1 = Wall::new();
    let wall2 = Wall::new();
    let wall3 = Wall::new();

    let room_sides : Vec<Box<MapSite>> = vec![
        Box::new(wall1),
        Box::new(door),
        Box::new(wall2),
        Box::new(wall3)];

    let mut first_room = Room::new(23, room_sides);

    first_room.get_side(Direction::East).enter();
    let wall4 = Wall::new();
    println!("Changing a side in the room...");
    first_room.set_side(Direction::East, Box::new(wall4));
    first_room.get_side(Direction::East).enter();

    abstract_factory::cheese();
}
