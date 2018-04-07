use map_site::MapSite;

struct Door {
}

impl MapSite for Door {
    fn enter(&self) {
        println!("It's a wooden door.");
    }
}
