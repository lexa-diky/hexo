use std::collections::HashMap;

use crate::cst::{CstAtom, CstAtomStrip, CstAtomUnresolved, CstFile, CstStatement, CstStatementEmit};

struct ResolutionContext {
    bindings: HashMap<String, CstAtomStrip>,
}

impl ResolutionContext {
    fn from(cst_file: &CstFile) -> ResolutionContext {
        let bindings: Vec<_> = cst_file.constants()
            .iter().map(|cst_const| {
            let name = cst_const.name.to_string();
            let value = &cst_const.atoms;
            (name, value)
        }).collect();

        let buf = HashMap::new();
        let build = bindings.into_iter()
            .fold(buf, |mut acc, (name, value)| {
                acc.insert(name, value.clone());
                acc
            });

        ResolutionContext {
            bindings: build,
        }
    }
}

pub(crate) fn resolve_cst(cst_file: CstFile) -> CstFile {
    let context = ResolutionContext::from(&cst_file);

    let resolved_statements: Vec<_> = cst_file.statements.iter()
        .filter_map(|s|
            match s {
                CstStatement::Emit(it) =>
                    Some(CstStatement::Emit(resolve_emit_statement(it, &context))),
                _ => None
            }
        ).collect();

    CstFile {
        file_name: cst_file.file_name.clone(),
        statements: resolved_statements,
    }
}

fn resolve_emit_statement(strip: &CstStatementEmit, context: &ResolutionContext) -> CstStatementEmit {
    let mut buf = CstAtomStrip::empty();

    strip.atoms.iter().for_each(|atom| {
        buf.extend(resolve_atom(context, atom))
    });

    return CstStatementEmit {
        atoms: buf,
    };
}

fn resolve_atom(context: &ResolutionContext, atom: &CstAtom) -> CstAtomStrip {
    let mut buf = CstAtomStrip::empty();

    match atom {
        CstAtom::Resolved { .. } => { buf.push(atom.clone()) }
        CstAtom::Unresolved(it) => { buf.extend(resolve_unresolved_atom(it, context)) }
    }

    return buf;
}

fn resolve_unresolved_atom(atom: &CstAtomUnresolved, context: &ResolutionContext) -> CstAtomStrip {
    return match atom {
        CstAtomUnresolved::Const { name } =>
            resolve_const(name, context),
        CstAtomUnresolved::Fn { name, params_flatten } =>
            resolve_function(name, params_flatten, context)
    };
}

fn resolve_const(name: &String, context: &ResolutionContext) -> CstAtomStrip {
    context.bindings.get(name).unwrap().clone()
}

fn resolve_function(name: &String, params: &Vec<CstAtom>, context: &ResolutionContext) -> CstAtomStrip {
    match name.as_str() {
        "len" => {
            let resolved_params = params.iter()
                .map(|atom| resolve_atom(context, atom))
                .collect::<Vec<_>>();

            let arg1 =  resolved_params[0].clone();
            let size = arg1.iter().fold(0, |acc, it| it.len());

            return CstAtomStrip::from(vec![CstAtom::Resolved {
                value: vec![size as u8],
            }]);
        }
        _ => { panic!("unknown function {}", name) }
    }
}