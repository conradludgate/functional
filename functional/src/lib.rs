pub use functional_macro::pipe;

#[cfg(test)]
mod tests {
    use super::pipe;

    fn call_twice(f: impl Fn(i32) -> i32) -> impl FnOnce(i32) -> i32 {
        move |x| f(f(x))
    }

    fn square(x: i32) -> i32 {
        x * x
    }

    #[test]
    #[pipe]
    fn partial_fn() {
        let f = square >> call_twice;

        let y = 4 >> f;

        assert_eq!(y, 256);
    }
}
