/// ```py
/// type(a)
/// ```
///
/// # Usage
///
/// ```rust
/// use py_like::type_of; // This is even a test..
/// let a = 1.0;
/// println!("{}", type_of(&a));
/// ```
pub fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
