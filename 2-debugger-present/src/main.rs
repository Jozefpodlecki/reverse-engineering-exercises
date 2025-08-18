use std::io;
use windows::Win32::{Foundation::BOOL, System::Diagnostics::Debug::IsDebuggerPresent};

fn is_debugger_present() -> bool {
    unsafe { IsDebuggerPresent() != BOOL(0) }
}

fn main() {
    
    if is_debugger_present() {
        println!("Debugger is present!");
    } else {
        println!("No debugger detected.");
        // format_args_nl!("test");
        let test = format_args!("a");
        io::_print(test);
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}