use logos_derive::Logos;
use tests::assert_lex;

// #[derive(Logos, Debug, Clone, Copy, PartialEq)]
// enum Token {
//     #[regex("(de)+f")]
//     Composite,

//     #[token("d")]
//     D,

//     #[token("e")]
//     E,

//     #[token("f")]
//     F,
// }

// #[test]
// fn test() {
//     assert_lex(
//         "dedede",
//         &[
//             (Ok(Token::D), "d", 0..1),
//             (Ok(Token::E), "e", 1..2),
//             (Ok(Token::F), "d", 2..3),
//             (Ok(Token::D), "e", 3..4),
//             (Ok(Token::E), "d", 4..5),
//             (Ok(Token::F), "e", 5..6),
//         ],
//     );
//     assert_lex("dededef", &[(Ok(Token::Composite), "dededef", 0..7)]);
// }

#[derive(Logos, PartialEq, Debug)]
enum OtherToken {
    #[token("foo")]
    Foo,

    #[token("bar")]
    Bar,

    #[regex("[0-9]+")]
    Number,

    #[regex("[abc][def][ghi]")]
    AbcDefGhi,

    #[regex("[a-zA-Z]+")]
    Text,
}

#[test]
fn test_other() {
    assert_lex("foo", &[(Ok(OtherToken::Foo), "foo", 0..3)]);
}

#[derive(Logos, PartialEq, Debug)]
#[logos(skip " ")]
enum Token2 {
    #[token(".")]
    Accessor,

    #[token("...")]
    Ellipsis,
}

#[test]
fn test_priority() {
    assert_lex(
        ". .. ...",
        &[
            (Ok(Token2::Accessor), ".", 0..1),
            (Ok(Token2::Accessor), ".", 2..3),
            (Ok(Token2::Accessor), ".", 3..4),
            (Ok(Token2::Ellipsis), "...", 5..8),
        ],
    );
    // assert_lex("ab", &[(Ok(Token2::Text), "ab", 0..2)]);
}
