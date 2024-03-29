#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn type_of() {
        use py_like::type_of;
        let a = 1.0;
        assert_eq!(type_of(&a), "f64");
    }

    #[test]
    fn main() {
        println!("Hello, world!"); // wtf it's ok
    }
}
