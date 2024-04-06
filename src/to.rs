pub trait ToI32 {
    fn to_i32(&self) -> i32;
}

impl ToI32 for String {
    fn to_i32(&self) -> i32 {
        self.trim()
            .parse::<i32>()
            .expect("failed to parse into i32")
    }
}
