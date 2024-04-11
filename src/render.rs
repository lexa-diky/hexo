use crate::cst::{CstAtom, CstAtomUnresolved, CstFile, CstStatementConst};
use crate::encoding::to_shrunk_bytes;
use std::collections::HashMap;

pub(crate) fn render_cst(cst_file: CstFile) -> Vec<u8> {
    let emits = cst_file.emits();
    let mut buf = Vec::new();
    emits.iter().for_each(|emit| {
        let atoms = emit
            .atoms
            .iter()
            .map(|atom| eval_atom(atom))
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

pub(crate) fn eval_atom(atom: &CstAtom) -> Vec<u8> {
    match atom {
        CstAtom::Resolved { value } => value.to_vec(),
        _ => panic!("unresolved atom are not supported in renderer"),
    }
}

pub(crate) fn resolve_atom(atom: &CstAtom) -> Vec<CstAtom> {
    match atom {
        CstAtom::Resolved { value } => vec![CstAtom::Resolved {
            value: value.to_vec(),
        }],
        _ => panic!("unresolved atom are not supported in renderer"),
    }
}
