use crate::encoder::buffer::InstrBuf;

pub const fn syscall() -> InstrBuf {
    InstrBuf::from_array([0x0F, 0x05])
}

pub const fn sysenter() -> InstrBuf {
    InstrBuf::from_array([0x0F, 0x34])
}

pub const fn sysexit() -> InstrBuf {
    InstrBuf::from_array([0x0F, 0x35])
}

pub const fn ret() -> InstrBuf {
    InstrBuf::from_byte(0xC3)
}

pub const fn nop() -> InstrBuf {
    InstrBuf::from_byte(0x90)
}

pub const fn cpuid() -> InstrBuf {
    InstrBuf::from_array([0x0F, 0xA2])
}

pub const fn hlt() -> InstrBuf {
    InstrBuf::from_byte(0xF4)
}

pub const fn int3() -> InstrBuf {
    InstrBuf::from_byte(0xCC)
}

pub const fn rdtsc() -> InstrBuf {
    InstrBuf::from_array([0x0F, 0x31])
}

pub const fn leave() -> InstrBuf {
    InstrBuf::from_byte(0xC9)
}