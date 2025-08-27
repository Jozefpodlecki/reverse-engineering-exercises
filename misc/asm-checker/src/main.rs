use std::arch::asm;

use anyhow::Result;

#[derive(Debug)]
#[repr(align(16))]
struct AlignedF32([f32; 4]);

fn main() -> Result<()> {
  
    let data = AlignedF32([1.0, 2.0, 3.0, 4.0]);
    let mut buffer = AlignedF32([0.0; 4]);

    unsafe {
        asm!(
            "movups xmm0, [{src}]",
            "movaps [{dst}], xmm0", 
            src = in(reg) &data,
            dst = in(reg) &mut buffer,
            options(nostack, preserves_flags),
        );
    }

    println!("buffer = {:?}", buffer);


    Ok(())
}