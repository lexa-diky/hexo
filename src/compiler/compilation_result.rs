pub(crate) struct Compilation {
    pub(crate) content: Vec<u8>
}

impl Compilation {

    pub(crate) fn from(content: Vec<u8>) -> Self {
        Compilation {
            content
        }
    }
}