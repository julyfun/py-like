fn main() {
    use py_like::input_prompt;
    let s = input_prompt(&"Enter a string for s: ");
    println!("s: {s}");
}
