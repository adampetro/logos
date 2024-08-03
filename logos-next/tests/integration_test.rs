use logos_next::Logos;

#[derive(Logos, PartialEq, Debug)]
enum Token {}

#[test]
fn test() {
    let mut lexer = Token::lexer("");
    assert_eq!(lexer.next(), None);
}
