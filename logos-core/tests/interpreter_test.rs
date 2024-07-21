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

    // panic!("stop here");

    assert_eq!(
        interpreter.collect::<Vec<_>>(),
        vec![
            Ok(Token::new("a".to_string(), b"a")),
            Ok(Token::new("b".to_string(), b"b")),
            Ok(Token::new("c".to_string(), b"c")),
            Ok(Token::new("d".to_string(), b"d")),
            Ok(Token::new("e".to_string(), b"e")),
            Ok(Token::new("f".to_string(), b"f")),
            Ok(Token::new("d".to_string(), b"d")),
            Ok(Token::new("e".to_string(), b"e")),
            Ok(Token::new("f".to_string(), b"f")),
            Ok(Token::new("number".to_string(), b"1234.567")),
        ]
    );
}
