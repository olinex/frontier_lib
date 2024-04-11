// @author:    olinex
// @time:      2023/10/11

// self mods

// use other mods

// use self mods

pub mod sysid {
    pub const DUP: usize = 27;
    pub const OPEN: usize = 56;
    pub const CLOSE: usize = 57;
    pub const PIPE: usize = 59;
    pub const READ: usize = 63;
    pub const WRITE: usize = 64;
    pub const EXIT: usize = 93;
    pub const YIELD: usize = 124;
    pub const KILL: usize = 129;
    pub const SIG_ACTION: usize = 134;
    pub const SIG_PROC_MASK: usize = 135;
    pub const SIG_RETURN: usize = 139;
    pub const GET_TIME: usize = 169;
    pub const GET_PID: usize = 172;
    pub const FORK: usize = 220;
    pub const EXEC: usize = 221;
    pub const WAIT_PID: usize = 260;
}

pub mod charater {
    pub const SPACE: char = ' ';
    pub const NULL: char = '\0';
}
