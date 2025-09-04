#[derive(Debug, Copy, Clone)]
struct BigStruct {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
    f: i32,
    g: i32,
    h: i32,
}

fn rust_calling_struct(s: BigStruct) -> i32 {
    s.a + s.b + s.c + s.d + s.e + s.f + s.g + s.h
}

fn rust_calling(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> i32 {
    a + b + c + d + e + f + g
}

extern "C" fn c_calling(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> i32 {
    a + b + c + d + e + f + g + 1
}

extern "system" fn system_calling(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> i32 {
    a + b + c + d + e + f + g + 2
}

extern "fastcall" fn fastcall_calling(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> i32 {
    a + b + c + d + e + f + g + 3
}


fn main() {
    let r1 = rust_calling(1, 2, 3, 4, 5, 6, 7);
    let r2 = c_calling(1, 2, 3, 4, 5, 6, 7);
    let r3 = system_calling(1, 2, 3, 4, 5, 6, 7);
    let r4 = fastcall_calling(1, 2, 3, 4, 5, 6, 7);

    let strct = BigStruct { a:1, b:2, c:3, d:4, e:5, f:6, g:7, h:8 };
    rust_calling_struct(strct);

    println!("rust_calling_convention: {}", r1);
    println!("c_calling_convention: {}", r2);
    println!("system_calling_convention: {}", r3);
    println!("fastcall_calling_convention: {}", r4);
}