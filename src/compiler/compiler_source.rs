pub(crate) trait CompilerSource {

    fn read(&self) -> String;
}

impl CompilerSource for &str {

    fn read(&self) -> String {
        return self.to_string();
    }
}
