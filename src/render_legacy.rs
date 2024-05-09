use crate::cst_legacy::{CstAtom, CstFile};

#[derive(Debug)]
pub(crate) enum RenderError {
    UnresolvedAtom { atom: CstAtom },
}

pub(crate) fn render_cst(cst_file: CstFile) -> Result<Vec<u8>, RenderError> {
    let emits = cst_file.emits();
    let mut buf = Vec::new();

    for emit in emits {
        for atom in emit.atoms.iter() {
            let evaluated = eval_atom(atom)?;
            buf.extend(evaluated);
        }
    }

    Ok(buf)
}

pub(crate) fn eval_atom(atom: &CstAtom) -> Result<Vec<u8>, RenderError> {
    match atom {
        CstAtom::Resolved { value } => Ok(value.to_vec()),
        _ => Err(RenderError::UnresolvedAtom { atom: atom.clone() }),
    }
}
