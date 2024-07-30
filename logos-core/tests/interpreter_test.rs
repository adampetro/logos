use logos_core::{
    interpreter::{Interpreter, Token},
    Lexer, Specification, Variant,
};

#[test]
fn test_interpreter() {
    let lexer = Lexer::new(vec![
        Variant::new("a".to_string(), Specification::Byte(b'a'), None),
        Variant::new("b".to_string(), Specification::Byte(b'b'), None),
        Variant::new("c".to_string(), Specification::Byte(b'c'), None),
        Variant::new("d".to_string(), Specification::Byte(b'd'), None),
        Variant::new("e".to_string(), Specification::Byte(b'e'), None),
        Variant::new("f".to_string(), Specification::Byte(b'f'), None),
        Variant::new(
            "def".to_string(),
            Specification::new_loop(3, None, Specification::new_str_sequence("def")),
            None,
        ),
        Variant::new(
            "number".to_string(),
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

    let interpreter = Interpreter::new(lexer, b"abcdefdef1234.567");

    dbg!(&interpreter);

    assert_eq!(
        interpreter.collect::<Result<Vec<Token>, ()>>(),
        Ok(vec![
            Token::new("a".to_string(), b"a"),
            Token::new("b".to_string(), b"b"),
            Token::new("c".to_string(), b"c"),
            Token::new("d".to_string(), b"d"),
            Token::new("e".to_string(), b"e"),
            Token::new("f".to_string(), b"f"),
            Token::new("d".to_string(), b"d"),
            Token::new("e".to_string(), b"e"),
            Token::new("f".to_string(), b"f"),
            Token::new("number".to_string(), b"1234.567"),
        ])
    );
}

#[test]
fn test_logos_bug() {
    let lexer = Lexer::new(vec![
        Variant::new(
            "composite".to_string(),
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
        Variant::new("d".to_string(), Specification::Byte(b'd'), None),
        Variant::new("e".to_string(), Specification::Byte(b'e'), None),
        Variant::new("f".to_string(), Specification::Byte(b'f'), None),
    ])
    .unwrap();

    let interpreter = Interpreter::new(lexer, b"dedede");

    dbg!(&interpreter);

    assert_eq!(
        interpreter.collect::<Result<Vec<Token>, ()>>(),
        Ok(vec![
            Token::new("d".to_string(), b"d"),
            Token::new("e".to_string(), b"e"),
            Token::new("d".to_string(), b"d"),
            Token::new("e".to_string(), b"e"),
            Token::new("d".to_string(), b"d"),
            Token::new("e".to_string(), b"e"),
        ]),
    );
}

#[test]
fn test_similar_tokens() {
    let lexer = Lexer::new(vec![
        Variant::new("a".to_string(), Specification::Byte(b'a'), None),
        Variant::new(
            "aa".to_string(),
            Specification::new_str_sequence("aa"),
            None,
        ),
        Variant::new(
            "aaa".to_string(),
            Specification::new_str_sequence("aaa"),
            None,
        ),
    ])
    .unwrap();

    let interpreter = Interpreter::new(lexer, b"aaaa");

    dbg!(&interpreter);

    assert_eq!(
        interpreter.collect::<Result<Vec<Token>, ()>>(),
        Ok(vec![
            Token::new("aaa".to_string(), b"aaa"),
            Token::new("a".to_string(), b"a"),
        ]),
    );
}

#[test]
fn test_json() {
    let lexer = Lexer::new(vec![
        Variant::new(
            "boolean".to_string(),
            Specification::new_any(vec![
                Specification::new_str_sequence("true"),
                Specification::new_str_sequence("false"),
            ]),
            None,
        ),
        Variant::new("open_brace".to_string(), Specification::Byte(b'{'), None),
        Variant::new("close_brace".to_string(), Specification::Byte(b'}'), None),
        Variant::new("open_bracket".to_string(), Specification::Byte(b'['), None),
        Variant::new("close_bracket".to_string(), Specification::Byte(b']'), None),
        Variant::new("colon".to_string(), Specification::Byte(b':'), None),
        Variant::new("comma".to_string(), Specification::Byte(b','), None),
        Variant::new(
            "null".to_string(),
            Specification::new_str_sequence("null"),
            None,
        ),
        Variant::new(
            "number".to_string(),
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
        Variant::new(
            "string".to_string(),
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
        Variant::new(
            "ignored".to_string(),
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

    let interpreter = Interpreter::new(lexer, b"truefalse{}[]:,null3.14159e0\"string\"");

    dbg!(&interpreter);

    assert_eq!(
        interpreter.collect::<Result<Vec<Token>, ()>>(),
        Ok(vec![
            Token::new("boolean".to_string(), b"true"),
            Token::new("boolean".to_string(), b"false"),
            Token::new("open_brace".to_string(), b"{"),
            Token::new("close_brace".to_string(), b"}"),
            Token::new("open_bracket".to_string(), b"["),
            Token::new("close_bracket".to_string(), b"]"),
            Token::new("colon".to_string(), b":"),
            Token::new("comma".to_string(), b","),
            Token::new("null".to_string(), b"null"),
            Token::new("number".to_string(), b"3.14159e0"),
            Token::new("string".to_string(), b"\"string\""),
        ]),
    );
}
