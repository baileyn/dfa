use super::*;
use std::io;

#[test]
fn dfa_builder_from_string() {
    // DFA for (ab)*
    let builder = DFABuilder::from(&mut io::Cursor::new(
        r#"
    0
    0 a 1
    0 b 2
    1 a 2
    1 b 0
    2 a 2
    2 b 2
    "#,
    ));

    assert_eq!(true, builder.is_ok());
}

#[test]
fn builder_without_final_state_fails() {
    let builder = DFABuilder::from(&mut io::Cursor::new(
        r#"
            0 a 1
            0 b 1
            1 a 0
            1 b 0
        "#,
    ));

    assert_eq!(Err(DFABuilderError::NonIntegralFinalState), builder);
}

#[test]
fn builder_without_content_fails() {
    let builder = DFABuilder::from(&mut io::Cursor::new(""));

    assert_eq!(Err(DFABuilderError::EmptyStream), builder);
}

#[test]
fn builder_handles_excess_whitespace() {
    // DFA for (ab)*
    let builder = DFABuilder::from(&mut io::Cursor::new(
        r#"
    0
    0     a 1
    0 b     2
    1    a    2
    1       b     0
    2      a   2
    2   b      2
    "#,
    ));

    assert_eq!(true, builder.is_ok());
}

#[test]
fn dfa_with_unhandled_states_fails() {
    // Invalid DFA, state 3 is never introduced..
    let builder = DFABuilder::from(&mut io::Cursor::new(
        r#"
        0
        0 a 1
        0 b 2
        1 a 2
        1 b 0
        2 a 3
        2 b 3
        "#,
    ));

    // The builder should read the DFA fine.
    assert_eq!(true, builder.is_ok());
    let dfa = builder.unwrap().build();

    assert_eq!(true, dfa.is_none());
}

#[test]
fn simple_dfa_accepts_valid_strings() {
    // DFA for (ab)*
    let builder = DFABuilder::from(&mut io::Cursor::new(
        r#"
        0
        0 a 1
        0 b 2
        1 a 2
        1 b 0
        2 a 2
        2 b 2
        "#,
    ));

    // The builder should read the DFA fine.
    assert_eq!(true, builder.is_ok());
    let dfa = builder.unwrap().build();

    assert_eq!(true, dfa.is_some());
    let dfa = dfa.unwrap();

    assert_eq!(true, dfa.is_valid_string("abababab"));
    assert_eq!(
        true,
        dfa.is_valid_string("ababababababababababababababababab")
    );
    assert_eq!(true, dfa.is_valid_string("ab"));
    assert_eq!(true, dfa.is_valid_string(""));
}

#[test]
fn simple_dfa_rejects_invalid_strings() {
    // DFA for (ab)*
    let builder = DFABuilder::from(&mut io::Cursor::new(
        r#"
        0
        0 a 1
        0 b 2
        1 a 2
        1 b 0
        2 a 2
        2 b 2
        "#,
    ));

    assert_eq!(true, builder.is_ok());
    let dfa = builder.unwrap().build();

    assert_eq!(true, dfa.is_some());
    let dfa = dfa.unwrap();

    assert_eq!(false, dfa.is_valid_string("ababaabab"));
    assert_eq!(
        false,
        dfa.is_valid_string("ababababababababababbababababababab")
    );
    assert_eq!(false, dfa.is_valid_string("bab"));
    assert_eq!(false, dfa.is_valid_string("bb"));
}

#[test]
fn simple_dfa_with_different_final_state_accepts_valid_strings() {
    // DFA for ab*a
    let builder = DFABuilder::from(&mut io::Cursor::new(
        r#"
        2
        0 a 1
        0 b 4
        1 a 2
        1 b 3
        2 a 4
        2 b 4
        3 a 2
        3 b 3
        4 a 4
        4 b 4
        "#,
    ));

    assert_eq!(true, builder.is_ok());
    let dfa = builder.unwrap().build();

    assert_eq!(true, dfa.is_some());
    let dfa = dfa.unwrap();

    assert_eq!(true, dfa.is_valid_string("abbbbbbbbbba"));
    assert_eq!(true, dfa.is_valid_string("aa"));
    assert_eq!(true, dfa.is_valid_string("aba"));
}

#[test]
fn simple_dfa_with_different_final_state_rejects_invalid_strings() {
    // DFA for ab*a
    let builder = DFABuilder::from(&mut io::Cursor::new(
        r#"
        2
        0 a 1
        0 b 4
        1 a 2
        1 b 3
        2 a 4
        2 b 4
        3 a 2
        3 b 3
        4 a 4
        4 b 4
        "#,
    ));

    assert_eq!(true, builder.is_ok());
    let dfa = builder.unwrap().build();

    assert_eq!(true, dfa.is_some());
    let dfa = dfa.unwrap();

    assert_eq!(false, dfa.is_valid_string("abbbbbbbbbbaa"));
    assert_eq!(false, dfa.is_valid_string("aaa"));
    assert_eq!(false, dfa.is_valid_string(""));
}
