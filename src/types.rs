pub type Bit = bool;
pub type Byte = u8;
pub type Word = u16;
pub type Num = usize;
pub type CpuFlag = usize;

/// 6502 CPU flags
pub struct CpuFlags { }
#[allow(dead_code)]
impl CpuFlags {
    /// Carry 
    pub const CARRY: CpuFlag              = 0;
    /// Zero page
    pub const ZERO: CpuFlag               = 1;
    /// Disable interrupt
    pub const INTERRUPTDISABLE: CpuFlag   = 2;
    /// Decimal mode
    pub const DECIMAL: CpuFlag            = 3;
    /// Interrupt (break)
    pub const BREAK: CpuFlag              = 4;
    /// Reserved for future use
    pub const FUTUREUSE: CpuFlag          = 5;
    /// Overflow
    pub const OVERFLOW: CpuFlag           = 6;
    /// Negative
    pub const NEGATIVE: CpuFlag           = 7;
}

