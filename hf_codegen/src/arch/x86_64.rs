use alloc::vec::Vec;
use iced_x86::code_asm::*;
use hf_parser::*;

use crate::CodegenError;

fn translate_expr(expr: Expr, a: &mut CodeAssembler) -> Result<(), CodegenError> {
    match expr {
        Expr::Add => {
            a.add(byte_ptr(r8), 0x1).map_err(|e| CodegenError::Generic(format!("{e}")))?;
        },
        Expr::Sub => {
            a.sub(byte_ptr(r8), 0x1).map_err(|e| CodegenError::Generic(format!("{e}")))?;
        },
        _ => unimplemented!(),
    }
    Ok(())
}

fn translate_token(token: Token, a: &mut CodeAssembler) -> Result<(), CodegenError> {
    match token {
        Token::Expr(expr) => translate_expr(expr, a)?,
        Token::FuncDecl { name, children } => {
            // TODO: Implement the function stuff itself
            for child in children {
                translate_token(child, a)?;
            }
        },
        _ => unimplemented!(),
    }
    Ok(())
}

pub fn gen_code(addr: u64, modules: Vec<Module>) -> Result<Vec<u8>, CodegenError> {
    let mut a = CodeAssembler::new(64).map_err(|e| CodegenError::Generic(format!("{e}")))?;

    for module in modules {
        for token in module.tokens {
            translate_token(token, &mut a)?;
        }
    }

    a.ret().map_err(|e| CodegenError::Generic(format!("{e}")))?;

    let bytes = a.assemble(addr).map_err(|e| CodegenError::Generic(format!("{e}")))?;
    Ok(bytes)
}
