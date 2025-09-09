use std::thread;

fn main() {

    let closure = || {
        for _ in 0..10000 {

        }
    };
    
    let builder = thread::Builder::new();
    let handle = unsafe { 
        builder.spawn_unchecked(closure).unwrap()
    };

    handle.join().unwrap();
}