use logos_derive::Logos;
use tests::assert_lex;

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
enum Token {
    #[regex("(de)+f")]
    Composite,

    #[token("d")]
    D,

    #[token("e")]
    E,

    #[token("f")]
    F,
}

#[test]
fn test() {
    assert_lex(
        "dedede",
        &[
            (Ok(Token::D), "d", 0..1),
            (Ok(Token::E), "e", 1..2),
            (Ok(Token::F), "d", 2..3),
            (Ok(Token::D), "e", 3..4),
            (Ok(Token::E), "d", 4..5),
            (Ok(Token::F), "e", 5..6),
        ],
    );
    assert_lex("dededef", &[(Ok(Token::Composite), "dededef", 0..7)]);
}
