#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use logos::{Lexer, Logos as _};
use logos_derive::Logos;
use tests::assert_lex;
struct MockExtras {
    spaces: usize,
    line_breaks: usize,
    numbers: usize,
    byte_size: u8,
}
#[automatically_derived]
impl ::core::default::Default for MockExtras {
    #[inline]
    fn default() -> MockExtras {
        MockExtras {
            spaces: ::core::default::Default::default(),
            line_breaks: ::core::default::Default::default(),
            numbers: ::core::default::Default::default(),
            byte_size: ::core::default::Default::default(),
        }
    }
}
fn byte_size_2(lexer: &mut Lexer<Token>) {
    lexer.extras.byte_size = 2;
}
fn byte_size_4(lexer: &mut Lexer<Token>) {
    lexer.extras.byte_size = 4;
}
#[logos(extras = MockExtras)]
enum Token {
    #[token("\n", |lex|{lex.extras.line_breaks+= 1;logos::Skip})]
    #[regex(r"[ \t\f]", |lex|{lex.extras.spaces+= 1;logos::Skip})]
    #[regex("[a-zA-Z$_][a-zA-Z0-9$_]*")]
    Identifier,
    #[regex("[1-9][0-9]*|0", |lex|lex.extras.numbers+= 1)]
    Number,
    #[regex("0b[01]+")]
    Binary,
    #[regex("0x[0-9a-fA-F]+")]
    Hex,
    #[regex("(abc)+(def|xyz)?")]
    Abc,
    #[token("priv")]
    Priv,
    #[token("private")]
    Private,
    #[token("primitive")]
    Primitive,
    #[token("protected")]
    Protected,
    #[token("protectee")]
    Protectee,
    #[token("in")]
    In,
    #[token("instanceof")]
    Instanceof,
    #[regex("byte|bytes[1-9][0-9]?")]
    Byte,
    #[regex(
        "int(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)"
    )]
    Int,
    #[token("uint8", |lex|lex.extras.byte_size = 1)]
    #[token("uint16", byte_size_2)]
    #[token("uint32", byte_size_4)]
    Uint,
    #[token(".")]
    Accessor,
    #[token("...")]
    Ellipsis,
    #[token("{")]
    BraceOpen,
    #[token("}")]
    BraceClose,
    #[token("+")]
    OpAddition,
    #[token("++")]
    OpIncrement,
    #[token("=")]
    OpAssign,
    #[token("==")]
    OpEquality,
    #[token("===")]
    OpStrictEquality,
    #[token("=>")]
    FatArrow,
}
impl<'s> ::logos::Logos<'s> for Token {
    type Error = ();
    type Extras = MockExtras;
    type Source = str;
    fn lex(lex: &mut ::logos::Lexer<'s, Self>) {
        use ::logos::internal::{LexerInternal, CallbackResult};
        type Lexer<'s> = ::logos::Lexer<'s, Token>;
        enum State {
            GoTo187,
            GoTo199,
            GoTo16,
            GoTo131,
            GoTo144,
            GoTo178,
            GoTo126,
            GoTo233,
            GoTo139,
            GoTo198,
            GoTo241,
            GoTo224,
            GoTo19,
            GoTo2,
            GoTo130,
            GoTo44,
            GoTo189,
            GoTo206,
            GoTo121,
            GoTo27,
            GoTo155,
            GoTo5,
            GoTo180,
            GoTo214,
            GoTo193,
            GoTo146,
            GoTo13,
            GoTo222,
            GoTo120,
            GoTo154,
            GoTo149,
            GoTo4,
            GoTo132,
            GoTo226,
            GoTo157,
            GoTo29,
            GoTo12,
            GoTo140,
            GoTo191,
            GoTo123,
            GoTo229,
            GoTo148,
            GoTo7,
            GoTo33,
            GoTo161,
            GoTo195,
            GoTo208,
            GoTo242,
            GoTo28,
            GoTo15,
            GoTo143,
            GoTo156,
            GoTo122,
            GoTo228,
            GoTo32,
            GoTo160,
            GoTo211,
            GoTo125,
            GoTo31,
            GoTo142,
            GoTo159,
            GoTo231,
            GoTo150,
            GoTo1,
            GoTo129,
            GoTo210,
            GoTo124,
            GoTo239,
            GoTo30,
            GoTo137,
            GoTo158,
            GoTo171,
            GoTo17,
            GoTo128,
            GoTo145,
            GoTo34,
            GoTo179,
            GoTo221,
            GoTo127,
            GoTo153,
            GoTo8,
            GoTo136,
            GoTo170,
        }
        #[inline]
        fn pattern7(byte: u8) -> bool {
            match byte {
                b'0'..=b'9' => true,
                _ => false,
            }
        }
        #[inline]
        fn pattern0(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 1 > 0
        }
        #[inline]
        fn pattern6(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 16 > 0
        }
        #[inline]
        fn pattern2(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 2 > 0
        }
        #[inline]
        fn pattern4(byte: u8) -> bool {
            match byte {
                b'0'..=b'1' => true,
                _ => false,
            }
        }
        #[inline]
        fn pattern3(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 4 > 0
        }
        #[inline]
        fn pattern1(byte: u8) -> bool {
            const LUT: u64 = 35465847073801215u64;
            match 1u64.checked_shl(byte.wrapping_sub(48u8) as u32) {
                Some(shift) => LUT & shift != 0,
                None => false,
            }
        }
        #[inline]
        fn pattern5(byte: u8) -> bool {
            COMPACT_TABLE_0[byte as usize] & 8 > 0
        }
        static COMPACT_TABLE_0: [u8; 256] = [
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            31,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            7,
            23,
            23,
            23,
            23,
            23,
            23,
            23,
            7,
            23,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            0,
            0,
            0,
            0,
            31,
            0,
            29,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            27,
            31,
            31,
            31,
            31,
            31,
            31,
            31,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let mut state = State::GoTo242;
        let mut at: usize = 0;
        'state_machine: loop {
            match state {
                State::GoTo187 => {
                    enum Jump {
                        __,
                        J5,
                        J180,
                        J178,
                    }
                    const LUT: [Jump; 256] = {
                        use Jump::*;
                        [
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J178,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J180,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                        ]
                    };
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo4;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match LUT[byte as usize] {
                        Jump::J5 => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        Jump::J180 => {
                            at += 1usize;
                            {
                                state = State::GoTo180;
                                continue 'state_machine;
                            }
                        }
                        Jump::J178 => {
                            at += 1usize;
                            {
                                state = State::GoTo178;
                                continue 'state_machine;
                            }
                        }
                        Jump::__ => {
                            state = State::GoTo4;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo199 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo31;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern0(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo31;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo16 => {
                    while let Some(arr) = lex.read::<&[u8; 16]>() {
                        if pattern1(arr[0]) {
                            if pattern1(arr[1]) {
                                if pattern1(arr[2]) {
                                    if pattern1(arr[3]) {
                                        if pattern1(arr[4]) {
                                            if pattern1(arr[5]) {
                                                if pattern1(arr[6]) {
                                                    if pattern1(arr[7]) {
                                                        if pattern1(arr[8]) {
                                                            if pattern1(arr[9]) {
                                                                if pattern1(arr[10]) {
                                                                    if pattern1(arr[11]) {
                                                                        if pattern1(arr[12]) {
                                                                            if pattern1(arr[13]) {
                                                                                if pattern1(arr[14]) {
                                                                                    if pattern1(arr[15]) {
                                                                                        lex.bump_unchecked(16);
                                                                                        continue;
                                                                                    }
                                                                                    lex.bump_unchecked(15);
                                                                                    {
                                                                                        state = State::GoTo15;
                                                                                        continue 'state_machine;
                                                                                    }
                                                                                }
                                                                                lex.bump_unchecked(14);
                                                                                {
                                                                                    state = State::GoTo15;
                                                                                    continue 'state_machine;
                                                                                }
                                                                            }
                                                                            lex.bump_unchecked(13);
                                                                            {
                                                                                state = State::GoTo15;
                                                                                continue 'state_machine;
                                                                            }
                                                                        }
                                                                        lex.bump_unchecked(12);
                                                                        {
                                                                            state = State::GoTo15;
                                                                            continue 'state_machine;
                                                                        }
                                                                    }
                                                                    lex.bump_unchecked(11);
                                                                    {
                                                                        state = State::GoTo15;
                                                                        continue 'state_machine;
                                                                    }
                                                                }
                                                                lex.bump_unchecked(10);
                                                                {
                                                                    state = State::GoTo15;
                                                                    continue 'state_machine;
                                                                }
                                                            }
                                                            lex.bump_unchecked(9);
                                                            {
                                                                state = State::GoTo15;
                                                                continue 'state_machine;
                                                            }
                                                        }
                                                        lex.bump_unchecked(8);
                                                        {
                                                            state = State::GoTo15;
                                                            continue 'state_machine;
                                                        }
                                                    }
                                                    lex.bump_unchecked(7);
                                                    {
                                                        state = State::GoTo15;
                                                        continue 'state_machine;
                                                    }
                                                }
                                                lex.bump_unchecked(6);
                                                {
                                                    state = State::GoTo15;
                                                    continue 'state_machine;
                                                }
                                            }
                                            lex.bump_unchecked(5);
                                            {
                                                state = State::GoTo15;
                                                continue 'state_machine;
                                            }
                                        }
                                        lex.bump_unchecked(4);
                                        {
                                            state = State::GoTo15;
                                            continue 'state_machine;
                                        }
                                    }
                                    lex.bump_unchecked(3);
                                    {
                                        state = State::GoTo15;
                                        continue 'state_machine;
                                    }
                                }
                                lex.bump_unchecked(2);
                                {
                                    state = State::GoTo15;
                                    continue 'state_machine;
                                }
                            }
                            lex.bump_unchecked(1);
                            {
                                state = State::GoTo15;
                                continue 'state_machine;
                            }
                        }
                        {
                            state = State::GoTo15;
                            continue 'state_machine;
                        }
                    }
                    while lex.test(pattern1) {
                        lex.bump_unchecked(1);
                    }
                    {
                        state = State::GoTo15;
                        continue 'state_machine;
                    };
                }
                State::GoTo131 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::OpStrictEquality));
                    break;
                }
                State::GoTo144 => {
                    match lex.read_at::<&[u8; 2usize]>(at) {
                        Some(b"yz") => {
                            at += 2usize;
                            {
                                state = State::GoTo143;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo178 => {
                    match lex.read_at::<&[u8; 5usize]>(at) {
                        Some(b"itive") => {
                            at += 5usize;
                            {
                                state = State::GoTo179;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo126 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::BraceClose));
                    break;
                }
                State::GoTo233 => {
                    match lex.read_at::<&[u8; 1usize]>(at) {
                        Some(b"+") => {
                            at += 1usize;
                            {
                                state = State::GoTo128;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo127;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo139 => {
                    match lex.read_at::<&[u8; 2usize]>(at) {
                        Some(b"bc") => {
                            at += 2usize;
                            {
                                state = State::GoTo140;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo198 => {
                    enum Jump {
                        __,
                        J5,
                        J189,
                        J199,
                    }
                    const LUT: [Jump; 256] = {
                        use Jump::*;
                        [
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            J5,
                            J5,
                            J5,
                            J189,
                            J199,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                        ]
                    };
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo4;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match LUT[byte as usize] {
                        Jump::J5 => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        Jump::J189 => {
                            at += 1usize;
                            {
                                state = State::GoTo189;
                                continue 'state_machine;
                            }
                        }
                        Jump::J199 => {
                            at += 1usize;
                            {
                                state = State::GoTo199;
                                continue 'state_machine;
                            }
                        }
                        Jump::__ => {
                            state = State::GoTo4;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo241 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo129;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        b'>' => {
                            state = State::GoTo132;
                            continue 'state_machine;
                        }
                        b'=' => {
                            state = State::GoTo239;
                            continue 'state_machine;
                        }
                        _ => {
                            state = State::GoTo129;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo224 => {
                    match lex.read_at::<&[u8; 3usize]>(at) {
                        Some(b"int") => {
                            at += 3usize;
                            {
                                state = State::GoTo226;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo19 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::Abc));
                    break;
                }
                State::GoTo2 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    #[inline]
                    fn callback<'s>(
                        lex: &mut Lexer<'s>,
                    ) -> impl CallbackResult<'s, (), Token> {
                        lex.extras.spaces += 1;
                        logos::Skip
                    }
                    callback(lex).construct(|()| Token::Identifier, lex);
                    break;
                }
                State::GoTo130 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::OpEquality));
                    break;
                }
                State::GoTo44 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::Int));
                    break;
                }
                State::GoTo189 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo30;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern0(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo30;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo206 => {
                    match lex.read_at::<&[u8; 1usize]>(at) {
                        Some(b"n") => {
                            at += 1usize;
                            {
                                state = State::GoTo208;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo121 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    byte_size_2(lex).construct(|()| Token::Uint, lex);
                    break;
                }
                State::GoTo27 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::Priv));
                    break;
                }
                State::GoTo155 => {
                    match lex.read_at::<&[u8; 1usize]>(at) {
                        Some(b"4") => {
                            at += 1usize;
                            {
                                state = State::GoTo156;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo5 => {
                    while let Some(arr) = lex.read::<&[u8; 16]>() {
                        if pattern0(arr[0]) {
                            if pattern0(arr[1]) {
                                if pattern0(arr[2]) {
                                    if pattern0(arr[3]) {
                                        if pattern0(arr[4]) {
                                            if pattern0(arr[5]) {
                                                if pattern0(arr[6]) {
                                                    if pattern0(arr[7]) {
                                                        if pattern0(arr[8]) {
                                                            if pattern0(arr[9]) {
                                                                if pattern0(arr[10]) {
                                                                    if pattern0(arr[11]) {
                                                                        if pattern0(arr[12]) {
                                                                            if pattern0(arr[13]) {
                                                                                if pattern0(arr[14]) {
                                                                                    if pattern0(arr[15]) {
                                                                                        lex.bump_unchecked(16);
                                                                                        continue;
                                                                                    }
                                                                                    lex.bump_unchecked(15);
                                                                                    {
                                                                                        state = State::GoTo4;
                                                                                        continue 'state_machine;
                                                                                    }
                                                                                }
                                                                                lex.bump_unchecked(14);
                                                                                {
                                                                                    state = State::GoTo4;
                                                                                    continue 'state_machine;
                                                                                }
                                                                            }
                                                                            lex.bump_unchecked(13);
                                                                            {
                                                                                state = State::GoTo4;
                                                                                continue 'state_machine;
                                                                            }
                                                                        }
                                                                        lex.bump_unchecked(12);
                                                                        {
                                                                            state = State::GoTo4;
                                                                            continue 'state_machine;
                                                                        }
                                                                    }
                                                                    lex.bump_unchecked(11);
                                                                    {
                                                                        state = State::GoTo4;
                                                                        continue 'state_machine;
                                                                    }
                                                                }
                                                                lex.bump_unchecked(10);
                                                                {
                                                                    state = State::GoTo4;
                                                                    continue 'state_machine;
                                                                }
                                                            }
                                                            lex.bump_unchecked(9);
                                                            {
                                                                state = State::GoTo4;
                                                                continue 'state_machine;
                                                            }
                                                        }
                                                        lex.bump_unchecked(8);
                                                        {
                                                            state = State::GoTo4;
                                                            continue 'state_machine;
                                                        }
                                                    }
                                                    lex.bump_unchecked(7);
                                                    {
                                                        state = State::GoTo4;
                                                        continue 'state_machine;
                                                    }
                                                }
                                                lex.bump_unchecked(6);
                                                {
                                                    state = State::GoTo4;
                                                    continue 'state_machine;
                                                }
                                            }
                                            lex.bump_unchecked(5);
                                            {
                                                state = State::GoTo4;
                                                continue 'state_machine;
                                            }
                                        }
                                        lex.bump_unchecked(4);
                                        {
                                            state = State::GoTo4;
                                            continue 'state_machine;
                                        }
                                    }
                                    lex.bump_unchecked(3);
                                    {
                                        state = State::GoTo4;
                                        continue 'state_machine;
                                    }
                                }
                                lex.bump_unchecked(2);
                                {
                                    state = State::GoTo4;
                                    continue 'state_machine;
                                }
                            }
                            lex.bump_unchecked(1);
                            {
                                state = State::GoTo4;
                                continue 'state_machine;
                            }
                        }
                        {
                            state = State::GoTo4;
                            continue 'state_machine;
                        }
                    }
                    while lex.test(pattern0) {
                        lex.bump_unchecked(1);
                    }
                    {
                        state = State::GoTo4;
                        continue 'state_machine;
                    };
                }
                State::GoTo180 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo27;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern2(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        b'a' => {
                            state = State::GoTo170;
                            continue 'state_machine;
                        }
                        _ => {
                            state = State::GoTo27;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo214 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo120;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern0(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo120;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo193 => {
                    enum Jump {
                        __,
                        J5,
                        J187,
                        J195,
                    }
                    const LUT: [Jump; 256] = {
                        use Jump::*;
                        [
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J187,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J195,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                        ]
                    };
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo4;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match LUT[byte as usize] {
                        Jump::J5 => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        Jump::J187 => {
                            at += 1usize;
                            {
                                state = State::GoTo187;
                                continue 'state_machine;
                            }
                        }
                        Jump::J195 => {
                            at += 1usize;
                            {
                                state = State::GoTo195;
                                continue 'state_machine;
                            }
                        }
                        Jump::__ => {
                            state = State::GoTo4;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo146 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo34;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern3(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        b's' => {
                            state = State::GoTo148;
                            continue 'state_machine;
                        }
                        _ => {
                            state = State::GoTo34;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo13 => {
                    while let Some(arr) = lex.read::<&[u8; 16]>() {
                        if pattern4(arr[0]) {
                            if pattern4(arr[1]) {
                                if pattern4(arr[2]) {
                                    if pattern4(arr[3]) {
                                        if pattern4(arr[4]) {
                                            if pattern4(arr[5]) {
                                                if pattern4(arr[6]) {
                                                    if pattern4(arr[7]) {
                                                        if pattern4(arr[8]) {
                                                            if pattern4(arr[9]) {
                                                                if pattern4(arr[10]) {
                                                                    if pattern4(arr[11]) {
                                                                        if pattern4(arr[12]) {
                                                                            if pattern4(arr[13]) {
                                                                                if pattern4(arr[14]) {
                                                                                    if pattern4(arr[15]) {
                                                                                        lex.bump_unchecked(16);
                                                                                        continue;
                                                                                    }
                                                                                    lex.bump_unchecked(15);
                                                                                    {
                                                                                        state = State::GoTo12;
                                                                                        continue 'state_machine;
                                                                                    }
                                                                                }
                                                                                lex.bump_unchecked(14);
                                                                                {
                                                                                    state = State::GoTo12;
                                                                                    continue 'state_machine;
                                                                                }
                                                                            }
                                                                            lex.bump_unchecked(13);
                                                                            {
                                                                                state = State::GoTo12;
                                                                                continue 'state_machine;
                                                                            }
                                                                        }
                                                                        lex.bump_unchecked(12);
                                                                        {
                                                                            state = State::GoTo12;
                                                                            continue 'state_machine;
                                                                        }
                                                                    }
                                                                    lex.bump_unchecked(11);
                                                                    {
                                                                        state = State::GoTo12;
                                                                        continue 'state_machine;
                                                                    }
                                                                }
                                                                lex.bump_unchecked(10);
                                                                {
                                                                    state = State::GoTo12;
                                                                    continue 'state_machine;
                                                                }
                                                            }
                                                            lex.bump_unchecked(9);
                                                            {
                                                                state = State::GoTo12;
                                                                continue 'state_machine;
                                                            }
                                                        }
                                                        lex.bump_unchecked(8);
                                                        {
                                                            state = State::GoTo12;
                                                            continue 'state_machine;
                                                        }
                                                    }
                                                    lex.bump_unchecked(7);
                                                    {
                                                        state = State::GoTo12;
                                                        continue 'state_machine;
                                                    }
                                                }
                                                lex.bump_unchecked(6);
                                                {
                                                    state = State::GoTo12;
                                                    continue 'state_machine;
                                                }
                                            }
                                            lex.bump_unchecked(5);
                                            {
                                                state = State::GoTo12;
                                                continue 'state_machine;
                                            }
                                        }
                                        lex.bump_unchecked(4);
                                        {
                                            state = State::GoTo12;
                                            continue 'state_machine;
                                        }
                                    }
                                    lex.bump_unchecked(3);
                                    {
                                        state = State::GoTo12;
                                        continue 'state_machine;
                                    }
                                }
                                lex.bump_unchecked(2);
                                {
                                    state = State::GoTo12;
                                    continue 'state_machine;
                                }
                            }
                            lex.bump_unchecked(1);
                            {
                                state = State::GoTo12;
                                continue 'state_machine;
                            }
                        }
                        {
                            state = State::GoTo12;
                            continue 'state_machine;
                        }
                    }
                    while lex.test(pattern4) {
                        lex.bump_unchecked(1);
                    }
                    {
                        state = State::GoTo12;
                        continue 'state_machine;
                    };
                }
                State::GoTo222 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo121;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern0(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo121;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo120 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    #[inline]
                    fn callback<'s>(
                        lex: &mut Lexer<'s>,
                    ) -> impl CallbackResult<'s, (), Token> {
                        lex.extras.byte_size = 1;
                    }
                    callback(lex).construct(|()| Token::Uint, lex);
                    break;
                }
                State::GoTo154 => {
                    enum Jump {
                        __,
                        J5,
                        J157,
                        J159,
                        J158,
                        J160,
                        J155,
                    }
                    const LUT: [Jump; 256] = {
                        use Jump::*;
                        [
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J155,
                            J157,
                            J158,
                            J159,
                            J155,
                            J157,
                            J160,
                            J159,
                            J155,
                            J157,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                        ]
                    };
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo4;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match LUT[byte as usize] {
                        Jump::J5 => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        Jump::J157 => {
                            at += 1usize;
                            {
                                state = State::GoTo157;
                                continue 'state_machine;
                            }
                        }
                        Jump::J159 => {
                            at += 1usize;
                            {
                                state = State::GoTo159;
                                continue 'state_machine;
                            }
                        }
                        Jump::J158 => {
                            at += 1usize;
                            {
                                state = State::GoTo158;
                                continue 'state_machine;
                            }
                        }
                        Jump::J160 => {
                            at += 1usize;
                            {
                                state = State::GoTo160;
                                continue 'state_machine;
                            }
                        }
                        Jump::J155 => {
                            at += 1usize;
                            {
                                state = State::GoTo155;
                                continue 'state_machine;
                            }
                        }
                        Jump::__ => {
                            state = State::GoTo4;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo149 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo34;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern5(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        b'0'..=b'9' => {
                            state = State::GoTo150;
                            continue 'state_machine;
                        }
                        _ => {
                            state = State::GoTo34;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo4 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::Identifier));
                    break;
                }
                State::GoTo132 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::FatArrow));
                    break;
                }
                State::GoTo226 => {
                    enum Jump {
                        __,
                        J5,
                        J221,
                        J228,
                        J214,
                    }
                    const LUT: [Jump; 256] = {
                        use Jump::*;
                        [
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            J221,
                            J5,
                            J228,
                            J5,
                            J5,
                            J5,
                            J5,
                            J214,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                        ]
                    };
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo4;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match LUT[byte as usize] {
                        Jump::J5 => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        Jump::J221 => {
                            at += 1usize;
                            {
                                state = State::GoTo221;
                                continue 'state_machine;
                            }
                        }
                        Jump::J228 => {
                            at += 1usize;
                            {
                                state = State::GoTo228;
                                continue 'state_machine;
                            }
                        }
                        Jump::J214 => {
                            at += 1usize;
                            {
                                state = State::GoTo214;
                                continue 'state_machine;
                            }
                        }
                        Jump::__ => {
                            state = State::GoTo4;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo157 => {
                    match lex.read_at::<&[u8; 1usize]>(at) {
                        Some(b"2") => {
                            at += 1usize;
                            {
                                state = State::GoTo156;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo29 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::Primitive));
                    break;
                }
                State::GoTo12 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::Binary));
                    break;
                }
                State::GoTo140 => {
                    enum Jump {
                        __,
                        J5,
                        J142,
                        J144,
                        J139,
                    }
                    const LUT: [Jump; 256] = {
                        use Jump::*;
                        [
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            J139,
                            J5,
                            J5,
                            J142,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J144,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                        ]
                    };
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo19;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match LUT[byte as usize] {
                        Jump::J5 => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        Jump::J142 => {
                            at += 1usize;
                            {
                                state = State::GoTo142;
                                continue 'state_machine;
                            }
                        }
                        Jump::J144 => {
                            at += 1usize;
                            {
                                state = State::GoTo144;
                                continue 'state_machine;
                            }
                        }
                        Jump::J139 => {
                            at += 1usize;
                            {
                                state = State::GoTo139;
                                continue 'state_machine;
                            }
                        }
                        Jump::__ => {
                            state = State::GoTo19;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo191 => {
                    match lex.read_at::<&[u8; 1usize]>(at) {
                        Some(b"r") => {
                            at += 1usize;
                            {
                                state = State::GoTo193;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo123 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::Accessor));
                    break;
                }
                State::GoTo229 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo122;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern0(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo122;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo148 => {
                    match lex.read_at::<&[u8; 1usize]>(at) {
                        Some([b'1'..=b'9']) => {
                            at += 1usize;
                            {
                                state = State::GoTo149;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo7 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    #[inline]
                    fn callback<'s>(
                        lex: &mut Lexer<'s>,
                    ) -> impl CallbackResult<'s, (), Token> {
                        lex.extras.numbers += 1;
                    }
                    callback(lex).construct(|()| Token::Number, lex);
                    break;
                }
                State::GoTo33 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::Instanceof));
                    break;
                }
                State::GoTo161 => {
                    enum Jump {
                        __,
                        J5,
                        J157,
                        J159,
                        J158,
                        J160,
                        J155,
                    }
                    const LUT: [Jump; 256] = {
                        use Jump::*;
                        [
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J158,
                            J159,
                            J155,
                            J157,
                            J160,
                            J159,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                        ]
                    };
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo4;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match LUT[byte as usize] {
                        Jump::J5 => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        Jump::J157 => {
                            at += 1usize;
                            {
                                state = State::GoTo157;
                                continue 'state_machine;
                            }
                        }
                        Jump::J159 => {
                            at += 1usize;
                            {
                                state = State::GoTo159;
                                continue 'state_machine;
                            }
                        }
                        Jump::J158 => {
                            at += 1usize;
                            {
                                state = State::GoTo158;
                                continue 'state_machine;
                            }
                        }
                        Jump::J160 => {
                            at += 1usize;
                            {
                                state = State::GoTo160;
                                continue 'state_machine;
                            }
                        }
                        Jump::J155 => {
                            at += 1usize;
                            {
                                state = State::GoTo155;
                                continue 'state_machine;
                            }
                        }
                        Jump::__ => {
                            state = State::GoTo4;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo195 => {
                    match lex.read_at::<&[u8; 5usize]>(at) {
                        Some(b"tecte") => {
                            at += 5usize;
                            {
                                state = State::GoTo198;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo208 => {
                    enum Jump {
                        __,
                        J5,
                        J153,
                        J210,
                    }
                    const LUT: [Jump; 256] = {
                        use Jump::*;
                        [
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J210,
                            J153,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                        ]
                    };
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo32;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match LUT[byte as usize] {
                        Jump::J5 => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        Jump::J153 => {
                            at += 1usize;
                            {
                                state = State::GoTo153;
                                continue 'state_machine;
                            }
                        }
                        Jump::J210 => {
                            at += 1usize;
                            {
                                state = State::GoTo210;
                                continue 'state_machine;
                            }
                        }
                        Jump::__ => {
                            state = State::GoTo32;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo242 => {
                    enum Jump {
                        __,
                        J5,
                        J231,
                        J1,
                        J126,
                        J233,
                        J139,
                        J241,
                        J145,
                        J224,
                        J2,
                        J125,
                        J191,
                        J206,
                        J8,
                        J136,
                    }
                    const LUT: [Jump; 256] = {
                        use Jump::*;
                        [
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J2,
                            J1,
                            __,
                            J2,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J2,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J233,
                            __,
                            __,
                            J231,
                            __,
                            J136,
                            J8,
                            J8,
                            J8,
                            J8,
                            J8,
                            J8,
                            J8,
                            J8,
                            J8,
                            __,
                            __,
                            __,
                            J241,
                            __,
                            __,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            J139,
                            J145,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J206,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J191,
                            J5,
                            J5,
                            J5,
                            J5,
                            J224,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J125,
                            __,
                            J126,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                        ]
                    };
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            lex.end();
                            break;
                        }
                    };
                    match LUT[byte as usize] {
                        Jump::J5 => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        Jump::J231 => {
                            at += 1usize;
                            {
                                state = State::GoTo231;
                                continue 'state_machine;
                            }
                        }
                        Jump::J1 => {
                            at += 1usize;
                            {
                                state = State::GoTo1;
                                continue 'state_machine;
                            }
                        }
                        Jump::J126 => {
                            at += 1usize;
                            {
                                state = State::GoTo126;
                                continue 'state_machine;
                            }
                        }
                        Jump::J233 => {
                            at += 1usize;
                            {
                                state = State::GoTo233;
                                continue 'state_machine;
                            }
                        }
                        Jump::J139 => {
                            at += 1usize;
                            {
                                state = State::GoTo139;
                                continue 'state_machine;
                            }
                        }
                        Jump::J241 => {
                            at += 1usize;
                            {
                                state = State::GoTo241;
                                continue 'state_machine;
                            }
                        }
                        Jump::J145 => {
                            at += 1usize;
                            {
                                state = State::GoTo145;
                                continue 'state_machine;
                            }
                        }
                        Jump::J224 => {
                            at += 1usize;
                            {
                                state = State::GoTo224;
                                continue 'state_machine;
                            }
                        }
                        Jump::J2 => {
                            at += 1usize;
                            {
                                state = State::GoTo2;
                                continue 'state_machine;
                            }
                        }
                        Jump::J125 => {
                            at += 1usize;
                            {
                                state = State::GoTo125;
                                continue 'state_machine;
                            }
                        }
                        Jump::J191 => {
                            at += 1usize;
                            {
                                state = State::GoTo191;
                                continue 'state_machine;
                            }
                        }
                        Jump::J206 => {
                            at += 1usize;
                            {
                                state = State::GoTo206;
                                continue 'state_machine;
                            }
                        }
                        Jump::J8 => {
                            at += 1usize;
                            {
                                state = State::GoTo8;
                                continue 'state_machine;
                            }
                        }
                        Jump::J136 => {
                            at += 1usize;
                            {
                                state = State::GoTo136;
                                continue 'state_machine;
                            }
                        }
                        Jump::__ => {
                            lex.bump_unchecked(1);
                            lex.error();
                            break;
                        }
                    }
                }
                State::GoTo28 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::Private));
                    break;
                }
                State::GoTo15 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::Hex));
                    break;
                }
                State::GoTo143 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo19;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern0(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo19;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo156 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo44;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern0(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo44;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo122 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    byte_size_4(lex).construct(|()| Token::Uint, lex);
                    break;
                }
                State::GoTo228 => {
                    match lex.read_at::<&[u8; 1usize]>(at) {
                        Some(b"2") => {
                            at += 1usize;
                            {
                                state = State::GoTo229;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo32 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::In));
                    break;
                }
                State::GoTo160 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo44;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern6(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        b'0' | b'8' => {
                            state = State::GoTo156;
                            continue 'state_machine;
                        }
                        _ => {
                            state = State::GoTo44;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo211 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo33;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern0(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo33;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo125 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::BraceOpen));
                    break;
                }
                State::GoTo31 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::Protectee));
                    break;
                }
                State::GoTo142 => {
                    match lex.read_at::<&[u8; 2usize]>(at) {
                        Some(b"ef") => {
                            at += 2usize;
                            {
                                state = State::GoTo143;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo159 => {
                    match lex.read_at::<&[u8; 1usize]>(at) {
                        Some(b"6") => {
                            at += 1usize;
                            {
                                state = State::GoTo156;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo231 => {
                    match lex.read_at::<&[u8; 2usize]>(at) {
                        Some(b"..") => {
                            at += 2usize;
                            {
                                state = State::GoTo124;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo123;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo150 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo34;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern0(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo34;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo1 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    #[inline]
                    fn callback<'s>(
                        lex: &mut Lexer<'s>,
                    ) -> impl CallbackResult<'s, (), Token> {
                        lex.extras.line_breaks += 1;
                        logos::Skip
                    }
                    callback(lex).construct(|()| Token::Identifier, lex);
                    break;
                }
                State::GoTo129 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::OpAssign));
                    break;
                }
                State::GoTo210 => {
                    match lex.read_at::<&[u8; 7usize]>(at) {
                        Some(b"tanceof") => {
                            at += 7usize;
                            {
                                state = State::GoTo211;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo124 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::Ellipsis));
                    break;
                }
                State::GoTo239 => {
                    match lex.read_at::<&[u8; 1usize]>(at) {
                        Some(b"=") => {
                            at += 1usize;
                            {
                                state = State::GoTo131;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo130;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo30 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::Protected));
                    break;
                }
                State::GoTo137 => {
                    match lex.read_at::<&[u8; 1usize]>(at) {
                        Some([b'0'..=b'1']) => {
                            at += 1usize;
                            {
                                state = State::GoTo13;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            lex.bump_unchecked(1);
                            lex.error();
                            break;
                        }
                    }
                }
                State::GoTo158 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo4;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern6(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        b'0' | b'8' => {
                            state = State::GoTo156;
                            continue 'state_machine;
                        }
                        _ => {
                            state = State::GoTo4;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo171 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo28;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern0(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo28;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo17 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                lex.bump_unchecked(1);
                                lex.error();
                                break;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern1(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo16;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            lex.bump_unchecked(1);
                            lex.error();
                            break;
                        }
                    }
                }
                State::GoTo128 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::OpIncrement));
                    break;
                }
                State::GoTo145 => {
                    match lex.read_at::<&[u8; 3usize]>(at) {
                        Some(b"yte") => {
                            at += 3usize;
                            {
                                state = State::GoTo146;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo34 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::Byte));
                    break;
                }
                State::GoTo179 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo29;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        byte if pattern0(byte) => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo29;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo221 => {
                    match lex.read_at::<&[u8; 1usize]>(at) {
                        Some(b"6") => {
                            at += 1usize;
                            {
                                state = State::GoTo222;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo127 => {
                    lex.bump_unchecked(at);
                    at = 0;
                    lex.set(Ok(Token::OpAddition));
                    break;
                }
                State::GoTo153 => {
                    enum Jump {
                        __,
                        J5,
                        J161,
                        J160,
                        J157,
                        J159,
                        J158,
                        J155,
                        J154,
                    }
                    const LUT: [Jump; 256] = {
                        use Jump::*;
                        [
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            J154,
                            J161,
                            J157,
                            J158,
                            J159,
                            J155,
                            J157,
                            J160,
                            J159,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            J5,
                            __,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            J5,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                            __,
                        ]
                    };
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo4;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match LUT[byte as usize] {
                        Jump::J5 => {
                            at += 1usize;
                            {
                                state = State::GoTo5;
                                continue 'state_machine;
                            }
                        }
                        Jump::J161 => {
                            at += 1usize;
                            {
                                state = State::GoTo161;
                                continue 'state_machine;
                            }
                        }
                        Jump::J160 => {
                            at += 1usize;
                            {
                                state = State::GoTo160;
                                continue 'state_machine;
                            }
                        }
                        Jump::J157 => {
                            at += 1usize;
                            {
                                state = State::GoTo157;
                                continue 'state_machine;
                            }
                        }
                        Jump::J159 => {
                            at += 1usize;
                            {
                                state = State::GoTo159;
                                continue 'state_machine;
                            }
                        }
                        Jump::J158 => {
                            at += 1usize;
                            {
                                state = State::GoTo158;
                                continue 'state_machine;
                            }
                        }
                        Jump::J155 => {
                            at += 1usize;
                            {
                                state = State::GoTo155;
                                continue 'state_machine;
                            }
                        }
                        Jump::J154 => {
                            at += 1usize;
                            {
                                state = State::GoTo154;
                                continue 'state_machine;
                            }
                        }
                        Jump::__ => {
                            state = State::GoTo4;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo8 => {
                    while let Some(arr) = lex.read::<&[u8; 16]>() {
                        if pattern7(arr[0]) {
                            if pattern7(arr[1]) {
                                if pattern7(arr[2]) {
                                    if pattern7(arr[3]) {
                                        if pattern7(arr[4]) {
                                            if pattern7(arr[5]) {
                                                if pattern7(arr[6]) {
                                                    if pattern7(arr[7]) {
                                                        if pattern7(arr[8]) {
                                                            if pattern7(arr[9]) {
                                                                if pattern7(arr[10]) {
                                                                    if pattern7(arr[11]) {
                                                                        if pattern7(arr[12]) {
                                                                            if pattern7(arr[13]) {
                                                                                if pattern7(arr[14]) {
                                                                                    if pattern7(arr[15]) {
                                                                                        lex.bump_unchecked(16);
                                                                                        continue;
                                                                                    }
                                                                                    lex.bump_unchecked(15);
                                                                                    {
                                                                                        state = State::GoTo7;
                                                                                        continue 'state_machine;
                                                                                    }
                                                                                }
                                                                                lex.bump_unchecked(14);
                                                                                {
                                                                                    state = State::GoTo7;
                                                                                    continue 'state_machine;
                                                                                }
                                                                            }
                                                                            lex.bump_unchecked(13);
                                                                            {
                                                                                state = State::GoTo7;
                                                                                continue 'state_machine;
                                                                            }
                                                                        }
                                                                        lex.bump_unchecked(12);
                                                                        {
                                                                            state = State::GoTo7;
                                                                            continue 'state_machine;
                                                                        }
                                                                    }
                                                                    lex.bump_unchecked(11);
                                                                    {
                                                                        state = State::GoTo7;
                                                                        continue 'state_machine;
                                                                    }
                                                                }
                                                                lex.bump_unchecked(10);
                                                                {
                                                                    state = State::GoTo7;
                                                                    continue 'state_machine;
                                                                }
                                                            }
                                                            lex.bump_unchecked(9);
                                                            {
                                                                state = State::GoTo7;
                                                                continue 'state_machine;
                                                            }
                                                        }
                                                        lex.bump_unchecked(8);
                                                        {
                                                            state = State::GoTo7;
                                                            continue 'state_machine;
                                                        }
                                                    }
                                                    lex.bump_unchecked(7);
                                                    {
                                                        state = State::GoTo7;
                                                        continue 'state_machine;
                                                    }
                                                }
                                                lex.bump_unchecked(6);
                                                {
                                                    state = State::GoTo7;
                                                    continue 'state_machine;
                                                }
                                            }
                                            lex.bump_unchecked(5);
                                            {
                                                state = State::GoTo7;
                                                continue 'state_machine;
                                            }
                                        }
                                        lex.bump_unchecked(4);
                                        {
                                            state = State::GoTo7;
                                            continue 'state_machine;
                                        }
                                    }
                                    lex.bump_unchecked(3);
                                    {
                                        state = State::GoTo7;
                                        continue 'state_machine;
                                    }
                                }
                                lex.bump_unchecked(2);
                                {
                                    state = State::GoTo7;
                                    continue 'state_machine;
                                }
                            }
                            lex.bump_unchecked(1);
                            {
                                state = State::GoTo7;
                                continue 'state_machine;
                            }
                        }
                        {
                            state = State::GoTo7;
                            continue 'state_machine;
                        }
                    }
                    while lex.test(pattern7) {
                        lex.bump_unchecked(1);
                    }
                    {
                        state = State::GoTo7;
                        continue 'state_machine;
                    };
                }
                State::GoTo136 => {
                    let byte = match {
                        match at {
                            0 => lex.read::<u8>(),
                            a => lex.read_at::<u8>(a),
                        }
                    } {
                        Some(byte) => byte,
                        None => {
                            {
                                state = State::GoTo7;
                                continue 'state_machine;
                            }
                            continue;
                        }
                    };
                    match byte {
                        b'b' => {
                            state = State::GoTo137;
                            continue 'state_machine;
                        }
                        b'x' => {
                            state = State::GoTo17;
                            continue 'state_machine;
                        }
                        _ => {
                            state = State::GoTo7;
                            continue 'state_machine;
                        }
                    }
                }
                State::GoTo170 => {
                    match lex.read_at::<&[u8; 2usize]>(at) {
                        Some(b"te") => {
                            at += 2usize;
                            {
                                state = State::GoTo171;
                                continue 'state_machine;
                            }
                        }
                        _ => {
                            state = State::GoTo5;
                            continue 'state_machine;
                        }
                    }
                }
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Token {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Token::Identifier => "Identifier",
                Token::Number => "Number",
                Token::Binary => "Binary",
                Token::Hex => "Hex",
                Token::Abc => "Abc",
                Token::Priv => "Priv",
                Token::Private => "Private",
                Token::Primitive => "Primitive",
                Token::Protected => "Protected",
                Token::Protectee => "Protectee",
                Token::In => "In",
                Token::Instanceof => "Instanceof",
                Token::Byte => "Byte",
                Token::Int => "Int",
                Token::Uint => "Uint",
                Token::Accessor => "Accessor",
                Token::Ellipsis => "Ellipsis",
                Token::BraceOpen => "BraceOpen",
                Token::BraceClose => "BraceClose",
                Token::OpAddition => "OpAddition",
                Token::OpIncrement => "OpIncrement",
                Token::OpAssign => "OpAssign",
                Token::OpEquality => "OpEquality",
                Token::OpStrictEquality => "OpStrictEquality",
                Token::FatArrow => "FatArrow",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Token {
    #[inline]
    fn clone(&self) -> Token {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Token {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Token {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Token {
    #[inline]
    fn eq(&self, other: &Token) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "empty"]
pub const empty: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("empty"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 113usize,
        start_col: 4usize,
        end_line: 113usize,
        end_col: 9usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(empty())),
};
fn empty() {
    let mut lex = Token::lexer("");
    match (&lex.next(), &None) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&lex.span(), &(0..0)) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "whitespace"]
pub const whitespace: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("whitespace"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 121usize,
        start_col: 4usize,
        end_line: 121usize,
        end_col: 14usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(whitespace())),
};
fn whitespace() {
    let mut lex = Token::lexer("     ");
    match (&lex.next(), &None) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&lex.span(), &(5..5)) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "operators"]
pub const operators: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("operators"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 129usize,
        start_col: 4usize,
        end_line: 129usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(operators())),
};
fn operators() {
    assert_lex(
        "=== == = => + ++",
        &[
            (Ok(Token::OpStrictEquality), "===", 0..3),
            (Ok(Token::OpEquality), "==", 4..6),
            (Ok(Token::OpAssign), "=", 7..8),
            (Ok(Token::FatArrow), "=>", 9..11),
            (Ok(Token::OpAddition), "+", 12..13),
            (Ok(Token::OpIncrement), "++", 14..16),
        ],
    );
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "punctuation"]
pub const punctuation: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("punctuation"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 144usize,
        start_col: 4usize,
        end_line: 144usize,
        end_col: 15usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(punctuation())),
};
fn punctuation() {
    assert_lex(
        "{ . .. ... }",
        &[
            (Ok(Token::BraceOpen), "{", 0..1),
            (Ok(Token::Accessor), ".", 2..3),
            (Ok(Token::Accessor), ".", 4..5),
            (Ok(Token::Accessor), ".", 5..6),
            (Ok(Token::Ellipsis), "...", 7..10),
            (Ok(Token::BraceClose), "}", 11..12),
        ],
    );
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "identifiers"]
pub const identifiers: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("identifiers"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 159usize,
        start_col: 4usize,
        end_line: 159usize,
        end_col: 15usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(identifiers())),
};
fn identifiers() {
    assert_lex(
        "It was the year when they finally immanentized the Eschaton.",
        &[
            (Ok(Token::Identifier), "It", 0..2),
            (Ok(Token::Identifier), "was", 3..6),
            (Ok(Token::Identifier), "the", 7..10),
            (Ok(Token::Identifier), "year", 11..15),
            (Ok(Token::Identifier), "when", 16..20),
            (Ok(Token::Identifier), "they", 21..25),
            (Ok(Token::Identifier), "finally", 26..33),
            (Ok(Token::Identifier), "immanentized", 34..46),
            (Ok(Token::Identifier), "the", 47..50),
            (Ok(Token::Identifier), "Eschaton", 51..59),
            (Ok(Token::Accessor), ".", 59..60),
        ],
    );
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "keywords"]
pub const keywords: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("keywords"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 179usize,
        start_col: 4usize,
        end_line: 179usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(keywords())),
};
fn keywords() {
    assert_lex(
        "priv private primitive protected protectee in instanceof",
        &[
            (Ok(Token::Priv), "priv", 0..4),
            (Ok(Token::Private), "private", 5..12),
            (Ok(Token::Primitive), "primitive", 13..22),
            (Ok(Token::Protected), "protected", 23..32),
            (Ok(Token::Protectee), "protectee", 33..42),
            (Ok(Token::In), "in", 43..45),
            (Ok(Token::Instanceof), "instanceof", 46..56),
        ],
    );
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "keywords_mix_identifiers"]
pub const keywords_mix_identifiers: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("keywords_mix_identifiers"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 195usize,
        start_col: 4usize,
        end_line: 195usize,
        end_col: 28usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(keywords_mix_identifiers())),
};
fn keywords_mix_identifiers() {
    assert_lex(
        "pri priv priva privb privat private privatee privateer",
        &[
            (Ok(Token::Identifier), "pri", 0..3),
            (Ok(Token::Priv), "priv", 4..8),
            (Ok(Token::Identifier), "priva", 9..14),
            (Ok(Token::Identifier), "privb", 15..20),
            (Ok(Token::Identifier), "privat", 21..27),
            (Ok(Token::Private), "private", 28..35),
            (Ok(Token::Identifier), "privatee", 36..44),
            (Ok(Token::Identifier), "privateer", 45..54),
        ],
    );
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "iterator"]
pub const iterator: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("iterator"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 212usize,
        start_col: 4usize,
        end_line: 212usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(iterator())),
};
fn iterator() {
    let tokens: Vec<_> = Token::lexer("pri priv priva private").collect();
    match (
        &tokens,
        &&[
            Ok(Token::Identifier),
            Ok(Token::Priv),
            Ok(Token::Identifier),
            Ok(Token::Private),
        ],
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "spanned_iterator"]
pub const spanned_iterator: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("spanned_iterator"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 227usize,
        start_col: 4usize,
        end_line: 227usize,
        end_col: 20usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(spanned_iterator())),
};
fn spanned_iterator() {
    let tokens: Vec<_> = Token::lexer("pri priv priva private").spanned().collect();
    match (
        &tokens,
        &&[
            (Ok(Token::Identifier), 0..3),
            (Ok(Token::Priv), 4..8),
            (Ok(Token::Identifier), 9..14),
            (Ok(Token::Private), 15..22),
        ],
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "numbers"]
pub const numbers: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("numbers"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 242usize,
        start_col: 4usize,
        end_line: 242usize,
        end_col: 11usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(numbers())),
};
fn numbers() {
    assert_lex(
        "0 1 2 3 4 10 42 1337",
        &[
            (Ok(Token::Number), "0", 0..1),
            (Ok(Token::Number), "1", 2..3),
            (Ok(Token::Number), "2", 4..5),
            (Ok(Token::Number), "3", 6..7),
            (Ok(Token::Number), "4", 8..9),
            (Ok(Token::Number), "10", 10..12),
            (Ok(Token::Number), "42", 13..15),
            (Ok(Token::Number), "1337", 16..20),
        ],
    );
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "invalid_tokens"]
pub const invalid_tokens: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("invalid_tokens"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 259usize,
        start_col: 4usize,
        end_line: 259usize,
        end_col: 18usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(invalid_tokens())),
};
fn invalid_tokens() {
    assert_lex::<
        Token,
    >(
        "@-/!",
        &[
            (Err(()), "@", 0..1),
            (Err(()), "-", 1..2),
            (Err(()), "/", 2..3),
            (Err(()), "!", 3..4),
        ],
    );
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "hex_and_binary"]
pub const hex_and_binary: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("hex_and_binary"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 272usize,
        start_col: 4usize,
        end_line: 272usize,
        end_col: 18usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(hex_and_binary())),
};
fn hex_and_binary() {
    assert_lex(
        "0x0672deadbeef 0b0100010011",
        &[
            (Ok(Token::Hex), "0x0672deadbeef", 0..14),
            (Ok(Token::Binary), "0b0100010011", 15..27),
        ],
    );
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "invalid_hex_and_binary"]
pub const invalid_hex_and_binary: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("invalid_hex_and_binary"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 283usize,
        start_col: 4usize,
        end_line: 283usize,
        end_col: 26usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(invalid_hex_and_binary())),
};
fn invalid_hex_and_binary() {
    assert_lex(
        "0x 0b",
        &[
            (Ok(Token::Number), "0", 0..1),
            (Ok(Token::Identifier), "x", 1..2),
            (Ok(Token::Number), "0", 3..4),
            (Ok(Token::Identifier), "b", 4..5),
        ],
    );
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "abcs"]
pub const abcs: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("abcs"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 296usize,
        start_col: 4usize,
        end_line: 296usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(abcs())),
};
fn abcs() {
    assert_lex(
        "abc abcabcabcabc abcdef abcabcxyz",
        &[
            (Ok(Token::Abc), "abc", 0..3),
            (Ok(Token::Abc), "abcabcabcabc", 4..16),
            (Ok(Token::Abc), "abcdef", 17..23),
            (Ok(Token::Abc), "abcabcxyz", 24..33),
        ],
    );
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "invalid_abcs"]
pub const invalid_abcs: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("invalid_abcs"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 309usize,
        start_col: 4usize,
        end_line: 309usize,
        end_col: 16usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(invalid_abcs())),
};
fn invalid_abcs() {
    assert_lex(
        "ab abca abcabcab abxyz abcxy abcdefxyz",
        &[
            (Ok(Token::Identifier), "ab", 0..2),
            (Ok(Token::Identifier), "abca", 3..7),
            (Ok(Token::Identifier), "abcabcab", 8..16),
            (Ok(Token::Identifier), "abxyz", 17..22),
            (Ok(Token::Identifier), "abcxy", 23..28),
            (Ok(Token::Identifier), "abcdefxyz", 29..38),
        ],
    );
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "bytes"]
pub const bytes: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("bytes"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 324usize,
        start_col: 4usize,
        end_line: 324usize,
        end_col: 9usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(bytes())),
};
fn bytes() {
    assert_lex(
        "byte bytes1 bytes32",
        &[
            (Ok(Token::Byte), "byte", 0..4),
            (Ok(Token::Byte), "bytes1", 5..11),
            (Ok(Token::Byte), "bytes32", 12..19),
        ],
    );
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "extras_and_callbacks"]
pub const extras_and_callbacks: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("extras_and_callbacks"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 336usize,
        start_col: 4usize,
        end_line: 336usize,
        end_col: 24usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(extras_and_callbacks())),
};
fn extras_and_callbacks() {
    let source = "foo  bar     \n 42\n     HAL=9000";
    let mut lex = Token::lexer(source);
    while lex.next().is_some() {}
    match (&lex.extras.spaces, &13) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&lex.extras.line_breaks, &2) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&lex.extras.numbers, &2) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "ints"]
pub const ints: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("ints"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 349usize,
        start_col: 4usize,
        end_line: 349usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(ints())),
};
fn ints() {
    assert_lex(
        "int8 int16 int24 int32 int40 int48 int56 int64 int72 int80 \
         int88 int96 int104 int112 int120 int128 int136 int144 int152 \
         int160 int168 int176 int184 int192 int200 int208 int216 int224 \
         int232 int240 int248 int256",
        &[
            (Ok(Token::Int), "int8", 0..4),
            (Ok(Token::Int), "int16", 5..10),
            (Ok(Token::Int), "int24", 11..16),
            (Ok(Token::Int), "int32", 17..22),
            (Ok(Token::Int), "int40", 23..28),
            (Ok(Token::Int), "int48", 29..34),
            (Ok(Token::Int), "int56", 35..40),
            (Ok(Token::Int), "int64", 41..46),
            (Ok(Token::Int), "int72", 47..52),
            (Ok(Token::Int), "int80", 53..58),
            (Ok(Token::Int), "int88", 59..64),
            (Ok(Token::Int), "int96", 65..70),
            (Ok(Token::Int), "int104", 71..77),
            (Ok(Token::Int), "int112", 78..84),
            (Ok(Token::Int), "int120", 85..91),
            (Ok(Token::Int), "int128", 92..98),
            (Ok(Token::Int), "int136", 99..105),
            (Ok(Token::Int), "int144", 106..112),
            (Ok(Token::Int), "int152", 113..119),
            (Ok(Token::Int), "int160", 120..126),
            (Ok(Token::Int), "int168", 127..133),
            (Ok(Token::Int), "int176", 134..140),
            (Ok(Token::Int), "int184", 141..147),
            (Ok(Token::Int), "int192", 148..154),
            (Ok(Token::Int), "int200", 155..161),
            (Ok(Token::Int), "int208", 162..168),
            (Ok(Token::Int), "int216", 169..175),
            (Ok(Token::Int), "int224", 176..182),
            (Ok(Token::Int), "int232", 183..189),
            (Ok(Token::Int), "int240", 190..196),
            (Ok(Token::Int), "int248", 197..203),
            (Ok(Token::Int), "int256", 204..210),
        ],
    );
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "uints"]
pub const uints: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("uints"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests/simple.rs",
        start_line: 393usize,
        start_col: 4usize,
        end_line: 393usize,
        end_col: 9usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(uints())),
};
fn uints() {
    let mut lex = Token::lexer("uint8 uint16 uint32");
    match (&lex.next(), &Some(Ok(Token::Uint))) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&lex.span(), &(0..5)) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&lex.extras.byte_size, &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&lex.next(), &Some(Ok(Token::Uint))) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&lex.span(), &(6..12)) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&lex.extras.byte_size, &2) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&lex.next(), &Some(Ok(Token::Uint))) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&lex.span(), &(13..19)) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&lex.extras.byte_size, &4) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&lex.next(), &None) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &abcs,
            &bytes,
            &empty,
            &extras_and_callbacks,
            &hex_and_binary,
            &identifiers,
            &ints,
            &invalid_abcs,
            &invalid_hex_and_binary,
            &invalid_tokens,
            &iterator,
            &keywords,
            &keywords_mix_identifiers,
            &numbers,
            &operators,
            &punctuation,
            &spanned_iterator,
            &uints,
            &whitespace,
        ],
    )
}
