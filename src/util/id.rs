use lazy_static::lazy_static;
use std::fmt::{Debug, Display};
use std::sync::Mutex;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub(crate) struct HexoId {
    batch_id: u64,
    sequence_id: u64,
}

impl Display for HexoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}:{:x}", self.batch_id, self.sequence_id)
    }
}

impl Debug for HexoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}:{:x}", self.batch_id, self.sequence_id)
    }
}

lazy_static! {
    static ref COUNTER: Mutex<u64> = Mutex::new(0);
}

impl HexoId {
    pub(crate) fn next() -> HexoId {
        let mut counter = COUNTER.lock().unwrap();
        *counter += 1;
        HexoId {
            sequence_id: *counter,
            batch_id: 0,
        }
    }
}
