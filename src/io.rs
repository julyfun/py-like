#![allow(unused)]

use std::error::Error;
use std::fmt::{Debug, Display};
use std::io::{self, Write};
use std::str::FromStr;

/// Get user input, unfriendly to error handling.
pub fn input() -> String {
    let mut inner = String::new();
    std::io::stdin()
        .read_line(&mut inner)
        .expect("Failed to read from stdin");
    inner
}

/// Get user input with a prompt string.
///
/// - ref: [Yandros - users.rust-lang.org](https://users.rust-lang.org/t/why-is-it-so-difficult-to-get-user-input-in-rust/27444/3)
///
/// # Examples
///
/// ```no_run
/// use py_like::input_prompt;
/// let s = input_prompt(&"Enter a string for s: ");
/// println!("s: {s}");
/// ```
pub fn input_prompt(prompt: &'_ impl Display) -> String {
    print!("{}", prompt);
    // 不加 flush 的话 prompt 必须带换行符才会直接输出
    std::io::stdout().flush().expect("Flush failed");
    let mut ret = String::new();
    std::io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read from stdin");
    ret
}

/// Get user input and parse it to a specific type.
///
/// - ref: [alice - users.rust-lang.org](https://users.rust-lang.org/t/why-is-it-so-difficult-to-get-user-input-in-rust/27444/3)
pub fn input_ok<T: FromStr>() -> Result<T, Box<dyn Error>>
where
    <T as FromStr>::Err: Error + 'static,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    // Handle the errors outside
    Ok(input.trim().parse()?)
}

/// Get user input and parse it to a specific type, unwrap the result directly.
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

/// - ref: [coder3101 - stackoverflow.com](https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0)
#[macro_export] // export to the root of the crate
macro_rules! read {
    ($out:ident as $type:ty) => {
        let $out = {
            let mut inner = String::new();
            std::io::stdin().read_line(&mut inner).expect("a String");
            inner.trim().parse::<$type>().expect("Parsable")
        };
    };
}

#[macro_export]
macro_rules! read_str {
    ($out:ident) => {
        let $out = {
            let mut inner = String::new();
            std::io::stdin().read_line(&mut inner).expect("a String");
            inner.trim().to_string()
        };
    };
}

#[macro_export]
macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let $out = {
            let mut inner = String::new();
            std::io::stdin().read_line(&mut inner).unwrap();
            inner
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<$type>().unwrap())
                .collect::<Vec<$type>>()
        };
    };
}
