use std::io;

fn main() {
    
    let password = "uL964gA8PHtW8q=#Anbw";
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input == password {
        println!("Pass");
    }
    else {
        println!("Denied");
    }

    io::stdin().read_line(&mut input).unwrap();
}
