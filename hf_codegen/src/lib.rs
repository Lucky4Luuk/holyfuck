#![no_std]
#[macro_use] extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;

use thiserror_no_std::Error;

use hf_parser::Module;

mod arch;
pub use arch::Target;

#[derive(Debug, Error)]
pub enum CodegenError {
    #[error("generic error {0}")]
    Generic(String),
}

pub fn gen_code(target: Target, addr: u64, modules: Vec<Module>) -> Result<Vec<u8>, CodegenError> {
    target.gen_code(addr, modules)
}
