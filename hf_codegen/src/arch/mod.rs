pub mod x86_64;

use alloc::vec::Vec;
use hf_parser::Module;

pub enum Target {
    X86_64,
}

impl Target {
    pub fn gen_code(&self, addr: u64, modules: Vec<Module>) -> Result<Vec<u8>, crate::CodegenError> {
        match self {
            Self::X86_64 => x86_64::gen_code(addr, modules)
        }
    }
}
