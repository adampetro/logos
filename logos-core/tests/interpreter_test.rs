use logos_core::{
    interpreter::{Interpreter, Token},
    Lexer, SimpleVariantMatch, Specification,
};

#[test]
fn test_interpreter() {
    let lexer = Lexer::new(vec![
        SimpleVariantMatch::new("a", Specification::Byte(b'a'), None),
        SimpleVariantMatch::new("b", Specification::Byte(b'b'), None),
        SimpleVariantMatch::new("c", Specification::Byte(b'c'), None),
        SimpleVariantMatch::new("d", Specification::Byte(b'd'), None),
        SimpleVariantMatch::new("e", Specification::Byte(b'e'), None),
        SimpleVariantMatch::new("f", Specification::Byte(b'f'), None),
        SimpleVariantMatch::new(
            "def",
            Specification::new_loop(3, None, Specification::new_str_sequence("def")),
            None,
        ),
        SimpleVariantMatch::new(
            "number",
            Specification::new_sequence(vec![
                Specification::new_any(vec![
                    Specification::Byte(b'0'),
                    Specification::new_sequence(vec![
                        Specification::new_any(
                            (b'1'..=b'9').map(Specification::Byte).collect::<Vec<_>>(),
                        ),
                        Specification::new_loop(0, None, Specification::ascii_digit()),
                    ]),
                ]),
                Specification::new_loop(
                    0,
                    Some(1),
                    Specification::new_sequence(vec![
                        Specification::Byte(b'.'),
                        Specification::new_loop(1, None, Specification::ascii_digit()),
                    ]),
                ),
            ]),
            None,
        ),
    ])
    .unwrap();

    let interpreter = Interpreter::new(&lexer, b"abcdefdef1234.567").unwrap();

    dbg!(&interpreter);

    assert_eq!(
        interpreter.collect::<Result<Vec<Token>, ()>>(),
        Ok(vec![
            Token::new("a", b"a"),
            Token::new("b", b"b"),
            Token::new("c", b"c"),
            Token::new("d", b"d"),
            Token::new("e", b"e"),
            Token::new("f", b"f"),
            Token::new("d", b"d"),
            Token::new("e", b"e"),
            Token::new("f", b"f"),
            Token::new("number", b"1234.567"),
        ])
    );
}

#[test]
fn test_logos_bug() {
    let lexer = Lexer::new(vec![
        SimpleVariantMatch::new(
            "composite",
            Specification::new_sequence(vec![
                Specification::new_loop(
                    1,
                    None,
                    Specification::new_sequence(vec![
                        Specification::Byte(b'd'),
                        Specification::Byte(b'e'),
                    ]),
                ),
                Specification::Byte(b'f'),
            ]),
            None,
        ),
        SimpleVariantMatch::new("d", Specification::Byte(b'd'), None),
        SimpleVariantMatch::new("e", Specification::Byte(b'e'), None),
        SimpleVariantMatch::new("f", Specification::Byte(b'f'), None),
    ])
    .unwrap();

    let interpreter = Interpreter::new(&lexer, b"dedede").unwrap();

    dbg!(&interpreter);

    assert_eq!(
        interpreter.collect::<Result<Vec<Token>, ()>>(),
        Ok(vec![
            Token::new("d", b"d"),
            Token::new("e", b"e"),
            Token::new("d", b"d"),
            Token::new("e", b"e"),
            Token::new("d", b"d"),
            Token::new("e", b"e"),
        ]),
    );
}

#[test]
fn test_similar_tokens() {
    let lexer = Lexer::new(vec![
        SimpleVariantMatch::new("a", Specification::Byte(b'a'), None),
        SimpleVariantMatch::new("aa", Specification::new_str_sequence("aa"), None),
        SimpleVariantMatch::new("aaa", Specification::new_str_sequence("aaa"), None),
    ])
    .unwrap();

    let interpreter = Interpreter::new(&lexer, b"aaaa").unwrap();

    dbg!(&interpreter);

    assert_eq!(
        interpreter.collect::<Result<Vec<Token>, ()>>(),
        Ok(vec![Token::new("aaa", b"aaa"), Token::new("a", b"a"),]),
    );
}

#[test]
fn test_json() {
    let lexer = Lexer::new(vec![
        SimpleVariantMatch::new(
            "boolean",
            Specification::new_any(vec![
                Specification::new_str_sequence("true"),
                Specification::new_str_sequence("false"),
            ]),
            None,
        ),
        SimpleVariantMatch::new("open_brace", Specification::Byte(b'{'), None),
        SimpleVariantMatch::new("close_brace", Specification::Byte(b'}'), None),
        SimpleVariantMatch::new("open_bracket", Specification::Byte(b'['), None),
        SimpleVariantMatch::new("close_bracket", Specification::Byte(b']'), None),
        SimpleVariantMatch::new("colon", Specification::Byte(b':'), None),
        SimpleVariantMatch::new("comma", Specification::Byte(b','), None),
        SimpleVariantMatch::new("null", Specification::new_str_sequence("null"), None),
        SimpleVariantMatch::new(
            "number",
            Specification::new_sequence(vec![
                Specification::new_loop(0, Some(1), Specification::Byte(b'-')),
                Specification::new_any(vec![
                    Specification::Byte(b'0'),
                    Specification::new_sequence(vec![
                        Specification::new_any(
                            (b'1'..=b'9').map(Specification::Byte).collect::<Vec<_>>(),
                        ),
                        Specification::new_loop(0, None, Specification::ascii_digit()),
                    ]),
                ]),
                Specification::new_loop(
                    0,
                    Some(1),
                    Specification::new_sequence(vec![
                        Specification::Byte(b'.'),
                        Specification::new_loop(1, None, Specification::ascii_digit()),
                    ]),
                ),
                Specification::new_loop(
                    0,
                    Some(1),
                    Specification::new_sequence(vec![
                        Specification::new_any(vec![
                            Specification::Byte(b'e'),
                            Specification::Byte(b'E'),
                        ]),
                        Specification::new_loop(
                            0,
                            Some(1),
                            Specification::new_any(vec![
                                Specification::Byte(b'+'),
                                Specification::Byte(b'-'),
                            ]),
                        ),
                        Specification::new_loop(1, None, Specification::ascii_digit()),
                    ]),
                ),
            ]),
            None,
        ),
        SimpleVariantMatch::new(
            "string",
            Specification::new_sequence(vec![
                Specification::Byte(b'"'),
                Specification::new_loop(
                    0,
                    None,
                    Specification::new_any(vec![
                        Specification::new_not(&[b'"', b'\\']),
                        Specification::new_sequence(vec![
                            Specification::Byte(b'\\'),
                            Specification::new_any(vec![
                                Specification::Byte(b'"'),
                                Specification::Byte(b'\\'),
                                Specification::Byte(b'/'),
                                Specification::Byte(b'b'),
                                Specification::Byte(b'f'),
                                Specification::Byte(b'n'),
                                Specification::Byte(b'r'),
                                Specification::Byte(b't'),
                                Specification::new_sequence(vec![
                                    Specification::Byte(b'u'),
                                    Specification::new_loop(
                                        4,
                                        None,
                                        Specification::ascii_hex_digit(),
                                    ),
                                ]),
                            ]),
                        ]),
                    ]),
                ),
                Specification::Byte(b'"'),
            ]),
            None,
        ),
        SimpleVariantMatch::new(
            "ignored",
            Specification::new_loop(
                1,
                None,
                Specification::new_any(vec![
                    Specification::Byte(b' '),
                    Specification::Byte(b'\t'),
                    Specification::Byte(b'\r'),
                    Specification::Byte(b'\n'),
                ]),
            ),
            None,
        ),
    ])
    .unwrap();

    let interpreter = Interpreter::new(&lexer, b"truefalse{}[]:,null3.14159e0\"string\"").unwrap();

    dbg!(&interpreter);

    assert_eq!(
        interpreter.collect::<Result<Vec<Token>, ()>>(),
        Ok(vec![
            Token::new("boolean", b"true"),
            Token::new("boolean", b"false"),
            Token::new("open_brace", b"{"),
            Token::new("close_brace", b"}"),
            Token::new("open_bracket", b"["),
            Token::new("close_bracket", b"]"),
            Token::new("colon", b":"),
            Token::new("comma", b","),
            Token::new("null", b"null"),
            Token::new("number", b"3.14159e0"),
            Token::new("string", b"\"string\""),
        ]),
    );
}
