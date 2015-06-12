use super::*;

fn test_token_defn() -> TokenDefinition {
    TokenDefinition::new(format!("::parser"),
                         format!("Token"),
                         vec![format!("'input")],
                         vec![(format!("("), format!("LPAREN")),
                              (format!("R"), format!("LPAREN"))])
}

#[test]
fn test_match_pattern() {
    let defn = test_token_defn();
    assert_eq!(defn.match_pattern("("), "::parser::Token::LPAREN(..)");
}