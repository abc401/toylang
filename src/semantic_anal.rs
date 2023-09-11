use std::collections::HashMap;

use crate::parser::{
    Identifier, LExpression as LExp, Program, RExpression as RExp, Statement as Stmt,
};

#[derive(Debug)]
pub struct Symbol {
    pub ident: Identifier,
    pub rbp_offset: usize,
    pub initialized: bool,
}

#[derive(Debug)]
pub enum SematicError {
    RedeclareIdent(Identifier),
    UseOfUndeclaredIdent(Identifier),
    UseOfUninitializedIdent(Identifier),
}
use SematicError as SErr;

pub type SymTable = HashMap<String, Symbol>;

pub fn analyze(program: &Program) -> Result<SymTable, SematicError> {
    let mut symtable: SymTable = HashMap::new();
    let mut current_rbp_offset = 0;
    for stmt in program.iter() {
        match stmt {
            Stmt::Declare(ident) => {
                if symtable.contains_key(&ident.lexeme) {
                    let ident = &symtable.get(&ident.lexeme).unwrap().ident;
                    return Err(SErr::RedeclareIdent(ident.clone()));
                }
                current_rbp_offset += 4;
                symtable.insert(
                    ident.lexeme.clone(),
                    Symbol {
                        ident: ident.clone(),
                        rbp_offset: current_rbp_offset,
                        initialized: false,
                    },
                );
            }
            Stmt::Initialize(l_ident, rexp) => {
                if symtable.contains_key(&l_ident.lexeme) {
                    let ident = &symtable.get(&l_ident.lexeme).unwrap().ident;
                    return Err(SErr::RedeclareIdent(ident.clone()));
                }
                match rexp {
                    RExp::Ident(r_ident) => {
                        let r_sym = symtable.get(&r_ident.lexeme);
                        if r_sym.is_none() {
                            return Err(SErr::UseOfUndeclaredIdent(r_ident.clone()));
                        }
                        let r_sym = r_sym.unwrap();
                        if !r_sym.initialized {
                            return Err(SErr::UseOfUninitializedIdent(r_ident.clone()));
                        }
                    }
                    _ => (),
                }
                current_rbp_offset += 4;
                symtable.insert(
                    l_ident.lexeme.clone(),
                    Symbol {
                        ident: l_ident.clone(),
                        rbp_offset: current_rbp_offset,
                        initialized: true,
                    },
                );
            }
            Stmt::Assign(lexp, rexp) => {
                match rexp {
                    RExp::Ident(r_ident) => {
                        let r_sym = symtable.get(&r_ident.lexeme);
                        if r_sym.is_none() {
                            return Err(SErr::UseOfUndeclaredIdent(r_ident.clone()));
                        } else if !r_sym.unwrap().initialized {
                            return Err(SErr::UseOfUninitializedIdent(r_ident.clone()));
                        }
                    }
                    _ => {}
                }
                let LExp::Ident(l_ident) = lexp;
                let l_sym = symtable.get_mut(&l_ident.lexeme);
                if l_sym.is_none() {
                    return Err(SErr::UseOfUndeclaredIdent(l_ident.clone()));
                }
                l_sym.unwrap().initialized = true;
            }
        }
    }

    return Ok(symtable);
}
