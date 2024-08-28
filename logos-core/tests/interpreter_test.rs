use logos_core::{
    interpreter::{Interpreter, Token},
    Lexer, SimpleVariantMatch, Specification,
};

#[test]
fn test_interpreter() {
    let lexer = Lexer::new(vec![
        (Specification::Byte(b'a'), SimpleVariantMatch::new("a", 1)),
        (Specification::Byte(b'b'), SimpleVariantMatch::new("b", 1)),
        (Specification::Byte(b'c'), SimpleVariantMatch::new("c", 1)),
        (Specification::Byte(b'd'), SimpleVariantMatch::new("d", 1)),
        (Specification::Byte(b'e'), SimpleVariantMatch::new("e", 1)),
        (Specification::Byte(b'f'), SimpleVariantMatch::new("f", 1)),
        (
            Specification::new_loop(3, None, Specification::new_str_sequence("def")),
            SimpleVariantMatch::new("def", 18),
        ),
        (
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
            SimpleVariantMatch::new("number", 2),
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
        (
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
            SimpleVariantMatch::new("composite", 6),
        ),
        (Specification::Byte(b'd'), SimpleVariantMatch::new("d", 2)),
        (Specification::Byte(b'e'), SimpleVariantMatch::new("e", 2)),
        (Specification::Byte(b'f'), SimpleVariantMatch::new("f", 2)),
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
        (Specification::Byte(b'a'), SimpleVariantMatch::new("a", 2)),
        (
            Specification::new_str_sequence("aa"),
            SimpleVariantMatch::new("aa", 4),
        ),
        (
            Specification::new_str_sequence("aaa"),
            SimpleVariantMatch::new("aaa", 6),
        ),
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
        (
            Specification::new_any(vec![
                Specification::new_str_sequence("true"),
                Specification::new_str_sequence("false"),
            ]),
            SimpleVariantMatch::new("boolean", 8),
        ),
        (
            Specification::Byte(b'{'),
            SimpleVariantMatch::new("open_brace", 2),
        ),
        (
            Specification::Byte(b'}'),
            SimpleVariantMatch::new("close_brace", 2),
        ),
        (
            Specification::Byte(b'['),
            SimpleVariantMatch::new("open_bracket", 2),
        ),
        (
            Specification::Byte(b']'),
            SimpleVariantMatch::new("close_bracket", 2),
        ),
        (
            Specification::Byte(b':'),
            SimpleVariantMatch::new("colon", 2),
        ),
        (
            Specification::Byte(b','),
            SimpleVariantMatch::new("comma", 2),
        ),
        (
            Specification::new_str_sequence("null"),
            SimpleVariantMatch::new("null", 8),
        ),
        (
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
            SimpleVariantMatch::new("number", 2),
        ),
        (
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
            SimpleVariantMatch::new("string", 4),
        ),
        (
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
            SimpleVariantMatch::new("ignored", 2),
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

#[test]
fn test_longer_match_lower_priority() {
    let lexer = Lexer::new(vec![
        (
            Specification::utf8("[a-z]+").unwrap(),
            SimpleVariantMatch::new("word", 2),
        ),
        (
            Specification::utf8("(abc)(def)?").unwrap(),
            SimpleVariantMatch::new("abc", 6),
        ),
    ])
    .unwrap();

    let interpreter = Interpreter::new(&lexer, b"abcd").unwrap();

    dbg!(&interpreter);

    assert_eq!(
        interpreter.collect::<Result<Vec<Token>, ()>>(),
        Ok(vec![Token::new("word", b"abcd")]),
    );
}

#[test]
fn test_overlap() {
    let lexer = Lexer::new(vec![
        (
            Specification::Byte(b'.'),
            SimpleVariantMatch::new("accessor", 2),
        ),
        (
            Specification::new_str_sequence("..."),
            SimpleVariantMatch::new("ellipsis", 6),
        ),
        (
            Specification::Byte(b' '),
            SimpleVariantMatch::new("whitespace", 2),
        ),
    ])
    .unwrap();

    let interpreter = Interpreter::new(&lexer, b". .. ...").unwrap();

    dbg!(&interpreter);

    assert_eq!(
        interpreter.collect::<Result<Vec<Token>, ()>>(),
        Ok(vec![
            Token::new("accessor", b"."),
            Token::new("whitespace", b" "),
            Token::new("accessor", b"."),
            Token::new("accessor", b"."),
            Token::new("whitespace", b" "),
            Token::new("ellipsis", b"..."),
        ]),
    );
}

#[test]
fn test_infinite_loop_overlap() {
    let lexer = Lexer::new(vec![
        (
            Specification::new_loop(
                1,
                None,
                Specification::new_str_sequence("abc")
            ),
            SimpleVariantMatch::new("abc", 6),
        ),
        (
            Specification::new_sequence(vec![
                Specification::new_loop(
                    1,
                    None,
                    Specification::new_str_sequence("abc")
                ),
                Specification::new_str_sequence("d"),
            ]),
            SimpleVariantMatch::new("abcd", 8),
        ),
    ])
    .unwrap();

    let interpreter = Interpreter::new(&lexer, b"abcabcd").unwrap();

    dbg!(&interpreter);

    assert_eq!(
        interpreter.collect::<Result<Vec<Token>, ()>>(),
        Ok(vec![Token::new("abcd", b"abcabcd")]),
    );
}

#[test]
fn test_css() {
    let lexer = Lexer::new(vec![
        (
            Specification::utf8("[+-]?[0-9]*[.]?[0-9]+(?:[eE][+-]?[0-9]+)?").unwrap(),
            SimpleVariantMatch::new("number", 2),
        )
    ]).unwrap();

    let interpreter = Interpreter::new(&lexer, b"3.14159").unwrap();

    dbg!(&interpreter);

    assert_eq!(
        interpreter.collect::<Result<Vec<Token>, ()>>(),
        Ok(vec![Token::new("number", b"3.14159")]),
    );
}
