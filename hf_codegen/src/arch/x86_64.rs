use alloc::vec::Vec;
use iced_x86::code_asm::*;
use hf_parser::*;

use crate::CodegenError;

pub fn gen_code(addr: u64, modules: Vec<Module>) -> Result<Vec<u8>, CodegenError> {
    let mut a = CodeAssembler::new(64).map_err(|e| CodegenError::Generic(format!("{e}")))?;
    a.ret().map_err(|e| CodegenError::Generic(format!("{e}")))?;

    let bytes = a.assemble(addr).map_err(|e| CodegenError::Generic(format!("{e}")))?;
    Ok(bytes)
}
