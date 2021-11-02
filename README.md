# Usage
```rs
use tinytest::unit_test;

unit_test!(test1, || some_function_in_scope("test").unwrap(), "expected output")
```
automatically gets translated in compile time to a standard test:
```rs
#[cfg(test)]
mod test1 {
    use super::*;

    #[test]
    fn tiny_test() {
        assert_eq!(some_function_in_scope("test").unwrap(), "expected output");
    }
}
```
the same applies to this larger closure:
```rs
use tinytest::unit_test;

unit_test!(test2, || {
    let mut c = some_function_in_scope("test").unwrap().chars()
    c.next();
    c.next_back();
    (
        c.collect::<String>(),
        some_other_function_in_scope(73)
    )
}, (
    "xpected outpu".to_string(),
    21
    )
)
```
that translates to:
```rs
#[cfg(test)]
mod test2 {
    use super::*;

    #[test]
    fn tiny_test() {
        assert_eq!(
            {
                let mut c = some_function_in_scope("test").unwrap().chars()
                c.next();
                c.next_back();
                (
                    c.collect::<String>(),
                    some_other_function_in_scope(73)
                )
            }, (
                c.collect::<String>(),
                some_other_function_in_scope(73)
            )
        );
    }
}
```
