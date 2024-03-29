mod io;
mod types;

pub mod to;

pub use io::{input, input_from, input_ok, input_prompt};
pub use types::type_of;

#[cfg(test)]
mod test {
    #[test]
    fn it_works_too() {
        use crate::{input, input_from, type_of};
        use std::error::Error;
        let a = 1.0;
        println!("{}", type_of(&a));
        println!("{}", type_of(&type_of(&a)));
    }
}
