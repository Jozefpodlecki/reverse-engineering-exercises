use std::{fs::File, io::{self, Read, Write}};

fn main() {
    
    let mut file = File::create("test.txt").unwrap();

    // Write::write_all(&mut file, b"test").unwrap();
    file.write_all(b"test").unwrap();

    let mut file = File::create("text.txt").unwrap();

    let mut buffer = vec![0; 10];
    file.read(&mut buffer).unwrap();
    

}
