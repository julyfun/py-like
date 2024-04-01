/// Use a variable to get its type name (instead of using the type).
///
/// # Python Counterpart
///
/// ```py
/// type(a)
/// ```
///
/// # Examples
///
/// ```rust
/// use py_like::type_of; // This is even a test..
/// let a = 1.0;
/// println!("{}", type_of(&a));
/// ```
pub fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
