use logos_core::{
    graph::{Error, Graph},
    Lexer, SimpleVariantMatch, Specification,
};

#[test]
fn test_equal_priority_overlap_error() {
    let lexer = Lexer::new(vec![
        (Specification::Byte(b'a'), SimpleVariantMatch::new("a", 2)),
        (
            Specification::new_loop(1, None, Specification::ascii_alphabetic()),
            SimpleVariantMatch::new("text", 2),
        ),
    ])
    .unwrap();

    let errors = Graph::for_lexer(&lexer).expect_err("expected error");

    assert_eq!(
        errors,
        vec![Error::VariantMatchesOverlapWithSamePriority(
            &lexer.variant_matches()[0].1,
            &lexer.variant_matches()[1].1
        )],
    );
}
