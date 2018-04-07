use map_site::MapSite;

struct Wall {
}

impl Wall {
    // Constructor
    fn wall() -> Wall {
        unimplemented!();
    }
}

impl MapSite for Wall {
    fn enter(&self) {
        println!("You're running into a wall! Ouch!");
    }
}
