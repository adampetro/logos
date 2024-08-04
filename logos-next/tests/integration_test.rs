use logos_next::Logos;

#[derive(Logos, PartialEq, Debug)]
enum Token {
    #[token("foo")]
    Foo,

    #[token("bar")]
    Bar,

    #[regex("[0-9]+")]
    Number,

    #[regex("[abc][def][ghi]")]
    AbcDefGhi,
}

#[test]
fn test() {
    let mut lexer = Token::lexer("");
    assert_eq!(lexer.next(), None);

    let mut lexer = Token::lexer("foo");
    assert_eq!(lexer.next(), Some(Ok(Token::Foo)));

    let mut lexer = Token::lexer("bar");
    assert_eq!(lexer.next(), Some(Ok(Token::Bar)));

    let mut lexer = Token::lexer("123");
    assert_eq!(lexer.next(), Some(Ok(Token::Number)));
}
