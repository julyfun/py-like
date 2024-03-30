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

    #[test]
    fn read() {
        use py_like::{read, read_str, read_vec, type_of};
        read!(a as i32);
        println!("{}", a);
        read_str!(s);
        println!("{}", s);
        read_vec!(vec as i32);
        println!("{:?}", vec);
        println!("{:?}", type_of(&vec));
    }
}
