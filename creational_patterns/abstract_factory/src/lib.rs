pub mod map_site;  // trait to encompass walls and doors

pub mod wall;      // "class" to represent a Wall

pub mod wall;      // "class" to represent a doors

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
