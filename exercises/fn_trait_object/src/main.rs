use std::panic::RefUnwindSafe;

#[inline(never)]
fn call_fn_trait(
    fn_trait: &(dyn Fn() -> i32 + Sync + RefUnwindSafe),
    argc: isize,
    argv: *const *const u8,
    sigpipe: u8,
) -> isize {
    println!("{} {:?} {}", argc, argv, sigpipe);
    let result = fn_trait();
    println!("result: {}", result);

    123
}

fn main() {
    // let closure = || 1234567;
    let a = 1234567i32;
    let b = 99u64;
    let closure = || {
        a + b as i32
    };
    call_fn_trait(&closure, 0, std::ptr::null(), 0);
}