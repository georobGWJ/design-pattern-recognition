use map_site::MapSite;

//#[derive(Debug)]
pub struct Wall {
}

impl Wall {
    // Constructor
    pub fn new() -> Wall {
        Wall {}
    }
}

impl MapSite for Wall {
    fn enter(&self) {
        println!("You're running into a wall! Ouch!");
    }
}
