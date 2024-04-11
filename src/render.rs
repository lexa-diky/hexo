use crate::cst::{CstAtom, CstAtomUnresolved, CstFile, CstStatementConst};
use crate::encoding::to_shrunk_bytes;
use std::collections::HashMap;

pub(crate) fn render_cst(cst_file: CstFile) -> Vec<u8> {
    let emits = cst_file.emits();
    let mut buf = Vec::new();
    emits.iter().for_each(|emit| {
        let atoms = emit.atoms.iter()
            .map(|atom| eval_atom(atom))
            .collect::<Vec<_>>();
        buf.extend(atoms.iter().flatten());
    });

    buf
}

pub(crate) fn eval_atom(atom: &CstAtom) -> Vec<u8> {
    match atom {
        CstAtom::Resolved { value } => value.to_vec(),
        _ => panic!("unresolved atom are not supported in renderer"),
    }
}
