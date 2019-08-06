pub trait SemiDebug {
    fn semi_debug(&self) -> String;
}

impl<T: ToString> SemiDebug for T {
    fn semi_debug(&self) -> String {
        self.to_string().replace("\n", "\\n").replace("\r", "\\r")
    }
}
