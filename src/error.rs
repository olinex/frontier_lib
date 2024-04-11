// @author:    olinex
// @time:      2024/04/09

// self mods

// use other mods
use enum_group::EnumGroup;
use thiserror_no_std::Error;

// use self mods

#[derive(Error, EnumGroup, Debug)]
pub enum LibError {
    #[groups(signal)]
    #[error("Invalid signal number {0}")]
    InvalidSignalNumber(usize),
}
