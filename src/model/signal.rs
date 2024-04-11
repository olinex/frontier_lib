// @author:    olinex
// @time:      2024/04/06

// self mods

// use other mods
use enum_group::EnumGroup;

// use self mods
use crate::error::LibError;

pub const SIG_COUNT: usize = 32;

#[derive(Clone, Copy, EnumGroup, Debug)]
pub enum Signal {
    DEF = 0,
    HUP = 1,
    INT = 2,
    QUIT = 3,
    ILL = 4,
    TRAP = 5,
    ABRT = 6,
    BUS = 7,
    FPE = 8,
    KILL = 9,
    USR1 = 10,
    SEGV = 11,
    USR2 = 12,
    PIPE = 13,
    ALRM = 14,
    TERM = 15,
    STKFLT = 16,
    CHLD = 17,
    CONT = 18,
    STOP = 19,
    TSTP = 20,
    TTIN = 21,
    TTOU = 22,
    URG = 23,
    XCPU = 24,
    XFSZ = 25,
    VTALRM = 26,
    PROF = 27,
    WINCH = 28,
    IO = 29,
    PWR = 30,
    SYS = 31,
}
impl TryFrom<usize> for Signal {
    type Error = LibError;
    fn try_from(value: usize) -> core::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DEF),
            1 => Ok(Self::HUP),
            2 => Ok(Self::INT),
            3 => Ok(Self::QUIT),
            4 => Ok(Self::ILL),
            5 => Ok(Self::TRAP),
            6 => Ok(Self::ABRT),
            7 => Ok(Self::BUS),
            8 => Ok(Self::FPE),
            9 => Ok(Self::KILL),
            10 => Ok(Self::USR1),
            11 => Ok(Self::SEGV),
            12 => Ok(Self::USR2),
            13 => Ok(Self::PIPE),
            14 => Ok(Self::ALRM),
            15 => Ok(Self::TERM),
            16 => Ok(Self::STKFLT),
            17 => Ok(Self::CHLD),
            18 => Ok(Self::CONT),
            19 => Ok(Self::STOP),
            20 => Ok(Self::TSTP),
            21 => Ok(Self::TTIN),
            22 => Ok(Self::TTOU),
            23 => Ok(Self::URG),
            24 => Ok(Self::XCPU),
            25 => Ok(Self::XFSZ),
            26 => Ok(Self::VTALRM),
            27 => Ok(Self::PROF),
            28 => Ok(Self::WINCH),
            29 => Ok(Self::IO),
            30 => Ok(Self::PWR),
            31 => Ok(Self::SYS),
            _ => Err(LibError::InvalidSignalNumber(value)),
        }
    }
}

bitflags! {
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct SignalFlags: u32 {
        const DEF = 1 << 0;
        const HUP = 1 << 1;
        const INT = 1 << 2;
        const QUIT = 1 << 3;
        const ILL = 1 << 4;
        const TRAP = 1 << 5;
        const ABRT = 1 << 6;
        const BUS = 1 << 7;
        const FPE = 1 << 8;
        const KILL = 1 << 9;
        const USR1 = 1 << 10;
        const SEGV = 1 << 11;
        const USR2 = 1 << 12;
        const PIPE = 1 << 13;
        const ALRM = 1 << 14;
        const TERM = 1 << 15;
        const STKFLT = 1 << 16;
        const CHLD = 1 << 17;
        const CONT = 1 << 18;
        const STOP = 1 << 19;
        const TSTP = 1 << 20;
        const TTIN = 1 << 21;
        const TTOU = 1 << 22;
        const URG = 1 << 23;
        const XCPU = 1 << 24;
        const XFSZ = 1 << 25;
        const VTALRM = 1 << 26;
        const PROF = 1 << 27;
        const WINCH = 1 << 28;
        const IO = 1 << 29;
        const PWR = 1 << 30;
        const SYS = 1 << 31;
    }
}
impl SignalFlags {
    pub fn trunc(&self) -> Signal {
        (self.bits().trailing_ones() as usize).try_into().unwrap()
    }
}
impl From<Signal> for SignalFlags {
    fn from(value: Signal) -> Self {
        Self::from_bits(1 << value as u32).unwrap()
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
pub struct SignalAction {
    handler: usize,
    mask: SignalFlags,
}
impl SignalAction {
    pub const fn default() -> Self {
        Self {
            handler: 0,
            mask: SignalFlags::empty(),
        }
    }

    pub fn new(handler: usize, mask: SignalFlags) -> Self {
        Self { handler, mask }
    }

    pub fn handler(&self) -> *const usize {
        self.handler as *const usize
    }

    pub fn mask(&self) -> SignalFlags {
        self.mask
    }
}

#[derive(Debug)]
pub struct SingalTable {
    inner: [SignalAction; SIG_COUNT],
}
impl SingalTable {
    pub fn new() -> Self {
        Self {
            inner: [SignalAction::default(); SIG_COUNT],
        }
    }

    pub fn get(&self, index: usize) -> SignalAction {
        self.inner[index]
    }

    pub fn set(&mut self, index: usize, action: SignalAction) {
        self.inner[index] = action;
    }
}
