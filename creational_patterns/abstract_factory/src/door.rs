use map_site::MapSite;

struct Door {
}
impl Door {
    // Constructor
    fn door() -> Door {
        unimplemented!();
    }
}

impl MapSite for Door {
    fn enter(&self) {
        println!("It's a wooden door.");
    }
}
