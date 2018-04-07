use map_site::MapSite;

struct Room {
}

impl MapSite for Room {
    fn enter(&self) {
        println!("What a pretty room.");
    }
}
