// Trait to declare functions that Walls and Doors must implement

pub trait MapSite {
    fn enter(&self);  // This is like a C++ virtual function for mods implementing this trait
}
