#![no_std]

//! Write more compact unit tests with a small macro.

/// This macro takes a test name, a closure and its expected value, and translates them into a unit test.
/// # Examples
/// ```
/// unit_test!(test1, || some_function_in_scope("test").unwrap(), "expected output")
/// ```
/// automatically gets translated in compile time to a standard test:
/// ```
/// #[cfg(test)]
/// mod test1 {
///     use super::*;
///
///     #[test]
///     fn tiny_test() {
///         assert_eq!(some_function_in_scope("test").unwrap(), "expected output");
///     }
/// }
/// ```
/// the same applies for this more complex closure:
/// ```
/// unit_test!(test2, || {
///     let mut c = some_function_in_scope("test").unwrap().chars()
///     c.next();
///     c.next_back();
///     (
///         c.collect::<String>(),
///         some_other_function_in_scope(73)
///     )
/// }, (
///     "xpected outpu".to_string(),
///     21
///     )
/// )
/// ```
/// that translates to:
/// ```
/// #[cfg(test)]
/// mod test2 {
///     use super::*;
///
///     #[test]
///     fn tiny_test() {
///         assert_eq!(
///             {
///                 let mut c = some_function_in_scope("test").unwrap().chars()
///                 c.next();
///                 c.next_back();
///                 (
///                     c.collect::<String>(),
///                     some_other_function_in_scope(73)
///                 )
///             }, (
///                 c.collect::<String>(),
///                 some_other_function_in_scope(73)
///             )
///         );
///     }
/// }
/// ```
#[macro_export]
macro_rules! unit_test {
    ($name:ident, $fn:expr, $expected:expr) => {
        #[cfg(test)]
        mod $name {
            use super::*;

            #[test]
            fn tiny_test() {
                assert_eq!($fn(), $expected);
            }
        }
    };
}
