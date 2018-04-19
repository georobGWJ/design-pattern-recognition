use map_site::MapSite;

//#[derive(Debug, Clone, Copy)]
pub struct Wall {
}

impl Wall {
    // Constructor
    pub fn new() -> Wall {
        unimplemented!();
    }
}

impl MapSite for Wall {
    fn enter(&self) {
        println!("You're running into a wall! Ouch!");
    }
}
