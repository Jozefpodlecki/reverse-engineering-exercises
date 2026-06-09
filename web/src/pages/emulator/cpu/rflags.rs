#[derive(Clone, PartialEq)]
pub struct Rflags {
    pub cf: bool,  // Carry flag
    pub pf: bool,  // Parity flag
    pub af: bool,  // Adjust flag
    pub zf: bool,  // Zero flag
    pub sf: bool,  // Sign flag
    pub of: bool,  // Overflow flag
    pub df: bool,  // Direction flag
    pub if_: bool, // Interrupt flag
    pub tf: bool,  // Trap flag
    pub nt: bool,  // Nested task
    pub rf: bool,  // Resume flag
    pub vm: bool,  // Virtual 8086 mode
    pub ac: bool,  // Alignment check
    pub vif: bool, // Virtual interrupt flag
    pub vip: bool, // Virtual interrupt pending
    pub id: bool,  // ID flag (CPUID support)
    pub lock: bool, // Lock prefix (emulator only, not real CPU)
}

impl Rflags {
    pub fn new() -> Self {
        Self {
            cf: false,
            pf: false,
            af: false,
            zf: false,
            sf: false,
            of: false,
            df: false,
            if_: false,
            tf: false,
            nt: false,
            rf: false,
            vm: false,
            ac: false,
            vif: false,
            vip: false,
            id: false,
            lock: false,
        }
    }
    
    pub fn to_u64(&self) -> u64 {
        let mut value = 0;
        
        if self.cf { value |= 1 << 0; }
        if self.pf { value |= 1 << 2; }
        if self.af { value |= 1 << 4; }
        if self.zf { value |= 1 << 6; }
        if self.sf { value |= 1 << 7; }
        if self.tf { value |= 1 << 8; }
        if self.if_ { value |= 1 << 9; }
        if self.df { value |= 1 << 10; }
        if self.of { value |= 1 << 11; }
        if self.nt { value |= 1 << 14; }
        if self.rf { value |= 1 << 16; }
        if self.vm { value |= 1 << 17; }
        if self.ac { value |= 1 << 18; }
        if self.vif { value |= 1 << 19; }
        if self.vip { value |= 1 << 20; }
        if self.id { value |= 1 << 21; }
        
        value | 2 // Bit 1 is always 1
    }
    
    pub fn from_u64(&mut self, value: u64) {
        self.cf = (value >> 0) & 1 != 0;
        self.pf = (value >> 2) & 1 != 0;
        self.af = (value >> 4) & 1 != 0;
        self.zf = (value >> 6) & 1 != 0;
        self.sf = (value >> 7) & 1 != 0;
        self.tf = (value >> 8) & 1 != 0;
        self.if_ = (value >> 9) & 1 != 0;
        self.df = (value >> 10) & 1 != 0;
        self.of = (value >> 11) & 1 != 0;
        self.nt = (value >> 14) & 1 != 0;
        self.rf = (value >> 16) & 1 != 0;
        self.vm = (value >> 17) & 1 != 0;
        self.ac = (value >> 18) & 1 != 0;
        self.vif = (value >> 19) & 1 != 0;
        self.vip = (value >> 20) & 1 != 0;
        self.id = (value >> 21) & 1 != 0;
    }
    
    pub fn update_add(&mut self, dst: u64, src: u64, result: u64) {
        self.zf = result == 0;
        self.sf = (result >> 63) & 1 == 1;
        self.cf = (dst as u128 + src as u128) > u64::MAX as u128;
        self.of = ((dst ^ result) & (src ^ result) & (1 << 63)) != 0;
        self.af = ((dst & 0xF) + (src & 0xF)) > 0xF;
        self.pf = (result.count_ones() % 2) == 0;
    }
    
    pub fn update_sub(&mut self, dst: u64, src: u64, result: u64) {
        self.zf = result == 0;
        self.sf = (result >> 63) & 1 == 1;
        self.cf = dst < src;
        self.of = ((dst ^ src) & (dst ^ result) & (1 << 63)) != 0;
        self.af = (dst & 0xF) < (src & 0xF);
        self.pf = (result.count_ones() % 2) == 0;
    }
    
    pub fn update_inc(&mut self, result: u64) {
        self.zf = result == 0;
        self.sf = (result >> 63) & 1 == 1;
        self.of = result == 0x7FFFFFFFFFFFFFFF;
        self.af = (result & 0xF) == 0;
    }
    
    pub fn update_dec(&mut self, result: u64) {
        self.zf = result == 0;
        self.sf = (result >> 63) & 1 == 1;
        self.of = result == 0x8000000000000000;
        self.af = (result & 0xF) == 0xF;
    }
    
    pub fn update_xor(&mut self, result: u64) {
        self.zf = result == 0;
        self.sf = (result >> 63) & 1 == 1;
        self.cf = false;
        self.of = false;
        self.pf = (result.count_ones() % 2) == 0;
    }
    
    pub fn update_and(&mut self, result: u64) {
        self.zf = result == 0;
        self.sf = (result >> 63) & 1 == 1;
        self.cf = false;
        self.of = false;
        self.pf = (result.count_ones() % 2) == 0;
    }
    
    pub fn update_or(&mut self, result: u64) {
        self.zf = result == 0;
        self.sf = (result >> 63) & 1 == 1;
        self.cf = false;
        self.of = false;
        self.pf = (result.count_ones() % 2) == 0;
    }
}

impl Default for Rflags {
    fn default() -> Self {
        Self::new()
    }
}