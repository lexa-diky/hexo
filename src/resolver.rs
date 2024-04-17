use std::collections::HashMap;

use crate::cst::{CstAtom, CstAtomStrip, CstAtomUnresolved, CstFile, CstFunctionParameter, CstStatement, CstStatementEmit, CstStatementFn};

struct ResolutionContext {
    constant_bindings: HashMap<String, CstAtomStrip>,
   // functions: HashMap<String, CstStatementFn>,
}

impl ResolutionContext {

    fn from(cst_file: &CstFile) -> ResolutionContext {
        let constant_bindings = Self::extract_constant_bindings(cst_file);

        ResolutionContext { constant_bindings: constant_bindings }
    }

    fn extract_constant_bindings(cst_file: &CstFile) -> HashMap<String, CstAtomStrip> {
        let bindings: Vec<_> = cst_file
            .constants()
            .iter()
            .map(|cst_const| {
                let name = cst_const.name.to_string();
                let value = &cst_const.atoms;
                (name, value)
            })
            .collect();

        let buf = HashMap::new();
        let build = bindings.into_iter().fold(buf, |mut acc, (name, value)| {
            acc.insert(name, value.clone());
            acc
        });
        build
    }
}

pub(crate) fn resolve_cst(cst_file: CstFile) -> CstFile {
    let context = ResolutionContext::from(&cst_file);

    let resolved_statements: Vec<_> = cst_file
        .statements
        .iter()
        .filter_map(|s| match s {
            CstStatement::Emit(it) => {
                Some(CstStatement::Emit(resolve_emit_statement(it, &context)))
            }
            _ => None,
        })
        .collect();

    CstFile {
        file_name: cst_file.file_name.clone(),
        statements: resolved_statements,
    }
}

fn resolve_emit_statement(
    strip: &CstStatementEmit,
    context: &ResolutionContext,
) -> CstStatementEmit {
    let mut buf = CstAtomStrip::empty();

    strip
        .atoms
        .iter()
        .for_each(|atom| buf.extend(resolve_atom(context, atom)));

    return CstStatementEmit { atoms: buf };
}

fn resolve_atom(context: &ResolutionContext, atom: &CstAtom) -> CstAtomStrip {
    let mut buf = CstAtomStrip::empty();

    match atom {
        CstAtom::Resolved { .. } => buf.push(atom.clone()),
        CstAtom::Unresolved(it) => buf.extend(resolve_unresolved_atom(it, context)),
    }

    return buf;
}

fn resolve_unresolved_atom(atom: &CstAtomUnresolved, context: &ResolutionContext) -> CstAtomStrip {
    return match atom {
        CstAtomUnresolved::Const { name } => resolve_const(name, context),
        CstAtomUnresolved::Fn { name, params } => resolve_function(name, params, context),
    };
}

fn resolve_const(name: &String, context: &ResolutionContext) -> CstAtomStrip {
    context.constant_bindings.get(name).unwrap().clone()
}

fn resolve_function(
    name: &String,
    params: &Vec<CstFunctionParameter>,
    context: &ResolutionContext,
) -> CstAtomStrip {
    match name.as_str() {
        "len" => {
            assert_eq!(params.len(), 1);

            let size = extract_param(0, params, context).len();

            return CstAtomStrip::from(vec![CstAtom::Resolved {
                value: vec![size as u8],
            }]);
        }
        "pad_right" => {
            assert_eq!(params.len(), 2);

            let padding = extract_param(0, params, context).as_usize();

            let data = extract_param(1, params, context);
            let mut data_vec = data.clamp_vec_u8().clone();
            data_vec.resize(padding, 0);

            return CstAtomStrip::from(vec![CstAtom::Resolved {
                value: data_vec,
            }]);
        }
        "pad_left" => {
            assert_eq!(params.len(), 2);

            let padding = extract_param(0, params, context).as_usize();

            let data = extract_param(1, params, context);
            let data_vec = data.clamp_vec_u8().clone();
            let mut buff = vec![];

            for _ in 0..(padding - data_vec.len()) {
                buff.push(0)
            }

            buff.extend(data_vec);

            return CstAtomStrip::from(vec![CstAtom::Resolved {
                value: buff,
            }]);
        }
        _ => {
            panic!("unknown function {}", name)
        }
    }
}

fn extract_param(idx: usize, params: &Vec<CstFunctionParameter>, context: &ResolutionContext) -> CstAtomStrip {
    let param1 = params[idx].clone();

    let resolved_params = resolve_param(context, param1);
    let arg1 = clamp_param(resolved_params);
    arg1
}

fn resolve_param(context: &ResolutionContext, param: CstFunctionParameter) -> Vec<CstAtomStrip> {
    param
        .params
        .iter()
        .map(|atom| resolve_atom(context, atom))
        .collect::<Vec<_>>()
}

fn clamp_param(param: Vec<CstAtomStrip>) -> CstAtomStrip {
    let mut buf = CstAtomStrip::empty();
    param.iter().for_each(|it| buf.extend(it.clone()));
    buf
}
