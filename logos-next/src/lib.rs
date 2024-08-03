mod lexer;

pub use lexer::Lexer;

pub use logos_codegen_next::Logos;

pub trait Logos<'source>: Sized {
    fn lex(lexer: &mut Lexer<'source, Self>) -> Option<Result<Self, ()>>;

    fn lexer(source: &'source str) -> Lexer<'source, Self> {
        Lexer::new(source)
    }
}
