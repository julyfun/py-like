#![allow(unused)]

use std::error::Error;
use std::fmt::{Debug, Display};
use std::io;
use std::str::FromStr;

pub fn input() -> String {
    let mut inner = String::new();
    std::io::stdin()
        .read_line(&mut inner)
        .expect("Failed to read from stdin");
    inner
}

/// Yandros - users.rust-lang.org
/// https://users.rust-lang.org/t/why-is-it-so-difficult-to-get-user-input-in-rust/27444/3
pub fn input_prompt(prompt: &'_ impl Display) -> String {
    print!("{}", prompt);
    let mut ret = String::new();
    std::io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read from stdin");
    ret
}

/// alice - users.rust-lang.org
/// https://users.rust-lang.org/t/why-is-it-so-difficult-to-get-user-input-in-rust/27444/3
pub fn input_ok<T: FromStr>() -> Result<T, Box<dyn Error>>
where
    <T as FromStr>::Err: Error + 'static,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    // Handle the errors outside
    Ok(input.trim().parse()?)
}

pub fn input_from<T: FromStr>() -> T
where
    <T as FromStr>::Err: Debug + 'static,
{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    input.trim().parse().unwrap()
}

/// coder3101 - stackoverflow.com
/// https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0
#[macro_export] // export to the root of the crate
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("a String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("a String");
        let $out = inner.trim();
    };
}

macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}
