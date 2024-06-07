// TODO: Modify parser to provide list of
//       function declarations per scope
//       so functions defined later can be called.

use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashMap;
use iced_x86::code_asm::*;
use hf_parser::*;

use crate::CodegenError;

fn translate_expr(expr: Expr, a: &mut CodeAssembler) -> Result<(), CodegenError> {
    match expr {
        Expr::Add => a.add(byte_ptr(r8), 0x1),
        Expr::Sub => a.sub(byte_ptr(r8), 0x1),
        Expr::MoveRight => a.add(r8, 0x1),
        Expr::MoveLeft => a.sub(r8, 0x1),
    }.map_err(|e| CodegenError::Generic(format!("{e}")))?;
    Ok(())
}

fn translate_token(token: Token, a: &mut CodeAssembler, func_map: &mut HashMap<String, CodeLabel>) -> Result<(), CodegenError> {
    match token {
        Token::Expr(expr) => translate_expr(expr, a)?,
        Token::FuncDecl { name, children } => {
            let mut label = a.create_label();
            a.set_label(&mut label).map_err(|e| CodegenError::Generic(format!("{e}")))?;
            if func_map.insert(name.clone(), label).is_some() {
                return Err(CodegenError::FuncNameInUse(name));
            }
            let mut cur_scope_func_map = func_map.clone();
            for child in children {
                translate_token(child, a, &mut cur_scope_func_map)?;
            }
            a.ret().map_err(|e| CodegenError::Generic(format!("{e}")))?;
        },
        Token::FuncCall { name } => {
            if let Some(label) = func_map.get(&name) {
                a.call(*label).map_err(|e| CodegenError::Generic(format!("{e}")))?;
            } else {
                return Err(CodegenError::FuncNotDefined(name));
            }
        },
        _ => unimplemented!(),
    }
    Ok(())
}

pub fn gen_code(addr: u64, modules: Vec<Module>) -> Result<Vec<u8>, CodegenError> {
    let mut a = CodeAssembler::new(64).map_err(|e| CodegenError::Generic(format!("{e}")))?;
    let mut func_map = HashMap::new();

    for module in modules {
        for token in module.tokens {
            translate_token(token, &mut a, &mut func_map)?;
        }
    }

    a.ret().map_err(|e| CodegenError::Generic(format!("{e}")))?;

    let bytes = a.assemble(addr).map_err(|e| CodegenError::Generic(format!("{e}")))?;
    Ok(bytes)
}
