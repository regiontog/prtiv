pub mod api;
pub mod combinators;

impl api::Token for u8 {}

#[cfg(test)]
mod tests {
    use combinators;

    #[test]
    fn string() {
        let p = combinators::string("Hello".as_bytes());

        assert!(p.run(String::from("Hello World!").as_bytes()).succeeded());
        assert!(!p.run(String::from("hello World!").as_bytes()).succeeded());
    }

    #[test]
    fn compose() {
        let px = &[combinators::string("Hello".as_bytes()), combinators::string(" World!".as_bytes())];
        let c = combinators::compose(px);

        assert!(c.run(String::from("Hello World!").as_bytes()).succeeded());
        assert!(!c.run(String::from("HelloWorld! ").as_bytes()).succeeded());
        assert!(!c.run(String::from("Hello World ").as_bytes()).succeeded());
    }

    #[test]
    fn read_outside_input() {
        let p = combinators::string("Hello".as_bytes());

        assert!(!p.run(String::from("Hell").as_bytes()).succeeded());
    }
}
