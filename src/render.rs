use crate::cst::{CstAtom, CstAtomUnresolved, CstFile, CstStatementConst};
use crate::encoding::to_shrunk_bytes;
use std::collections::HashMap;

pub(crate) fn render_cst(cst_file: CstFile) -> Vec<u8> {
    let emits = cst_file.emits();
    let constants = cst_file.constants();

    let const_map = build_const_map(constants);

    let mut buf = Vec::new();
    emits.iter().for_each(|emit| {
        let atoms = emit
            .atoms
            .iter()
            .map(|atom| eval_atom(atom, const_map.clone()))
            .collect::<Vec<_>>();
        buf.extend(atoms.iter().flatten());
    });

    buf
}

pub(crate) fn build_const_map(constants: Vec<&CstStatementConst>) -> HashMap<String, Vec<CstAtom>> {
    let bindings: Vec<_> = constants
        .iter()
        .map(|cst_const| {
            let name = cst_const.name.to_string();
            let value = &cst_const.atoms;
            (name, value)
        })
        .collect();

    let buf = HashMap::new();
    bindings.into_iter().fold(buf, |mut acc, (name, value)| {
        acc.insert(name, value.to_vec());
        acc
    })
}

pub(crate) fn eval_atom(atom: &CstAtom, context: HashMap<String, Vec<CstAtom>>) -> Vec<u8> {
    match atom {
        CstAtom::Resolved { value } => value.to_vec(),
        CstAtom::Unresolved(unresolved) => match unresolved {
            CstAtomUnresolved::Const { name } => {
                let value = context.get(name).unwrap();
                value
                    .iter()
                    .map(|atom| eval_atom(atom, context.clone()))
                    .flatten()
                    .collect()
            }
            CstAtomUnresolved::Fn {
                name,
                params_flatten: params,
            } => eval_function(name, params.to_vec(), context.clone()),
        },
    }
}

pub(crate) fn resolve_atom(atom: &CstAtom, context: HashMap<String, Vec<CstAtom>>) -> Vec<CstAtom> {
    match atom {
        CstAtom::Resolved { value } => vec![CstAtom::Resolved {
            value: value.to_vec(),
        }],
        CstAtom::Unresolved(unresolved) => match unresolved {
            CstAtomUnresolved::Const { name } => {
                let value = context.get(name).unwrap();
                value
                    .iter()
                    .map(|atom| resolve_atom(atom, context.clone()))
                    .flatten()
                    .collect()
            }
            CstAtomUnresolved::Fn {
                name,
                params_flatten: params,
            } => {
                let resolved_params = params
                    .iter()
                    .map(|param| resolve_atom(param, context.clone()))
                    .flatten()
                    .collect();
                vec![CstAtom::Unresolved(CstAtomUnresolved::Fn {
                    name: name.to_string(),
                    params_flatten: resolved_params,
                })]
            }
        },
    }
}

pub(crate) fn eval_function(
    name: &str,
    params: Vec<CstAtom>,
    context: HashMap<String, Vec<CstAtom>>,
) -> Vec<u8> {
    let resolved_parameters: Vec<_> = resolve_params(params, context);

    match name {
        "len" => {
            let len_sum = resolved_parameters
                .iter()
                .fold(0, |acc, param| acc + param.len());
            return to_shrunk_bytes(len_sum as u32);
        }
        _ => panic!("Unknown function: {}", name),
    }
}

fn resolve_params(params: Vec<CstAtom>, context: HashMap<String, Vec<CstAtom>>) -> Vec<CstAtom> {
    params
        .iter()
        .map(|param| resolve_atom(param, context.clone()))
        .flatten()
        .collect()
}
