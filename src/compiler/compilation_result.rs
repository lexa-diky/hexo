pub(crate) struct Compilation {
    pub(crate) content: Vec<u8>
}

impl Compilation {

    pub(crate) fn empty() -> Self {
        Compilation {
            content: Vec::new()
        }
    }
}