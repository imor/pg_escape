use std::borrow::Cow;

use crate::keywords::parse_keyword;

/// Returns true if the character is a lowercase ascii alphanumeric or an underscore, else return false.
fn is_lower_alphanumeric_or_underscore(c: char) -> bool {
    matches!(c, 'a'..='z' | '0'..='9' | '_')
}

/// Returns true if the character is a lowercase ascii alphabetic or an underscore, else return false.
fn is_lower_alphabetic_or_underscore(c: char) -> bool {
    matches!(c, 'a'..='z' | '_')
}

//Adapted from https://github.com/postgres/postgres/blob/b82c877e76e2398409e823773413079668cf1881/src/backend/utils/adt/quote.c#L25
/// Returns an identifier quoted according to Postgres rules.
pub fn quote_identifier(identifier: &str) -> Cow<'_, str> {
    let mut chars = identifier.chars();
    let first = chars.next();
    let mut need_quoting = first
        .map(|c| !is_lower_alphabetic_or_underscore(c))
        .unwrap_or(false);

    let mut num_quotes = 0;
    for c in chars {
        if !is_lower_alphanumeric_or_underscore(c) {
            need_quoting = true;
        }
        if c == '"' {
            num_quotes += 1;
        }
    }

    if !need_quoting && parse_keyword(identifier).is_some() {
        need_quoting = true;
    }

    if need_quoting {
        let mut result = String::with_capacity(identifier.len() + num_quotes + 2);
        result.push('"');
        for c in identifier.chars() {
            if c == '"' {
                result.push('"');
            }
            result.push(c);
        }
        result.push('"');
        Cow::Owned(result)
    } else {
        Cow::Borrowed(identifier)
    }
}

#[cfg(test)]
mod tests {
    use crate::keywords::KEYWORDS;

    use super::quote_identifier;

    fn run_test(ident: &str, expected_quoted_ident: &str) {
        let actual_quoted_ident = quote_identifier(ident);
        assert_eq!(actual_quoted_ident, expected_quoted_ident);
    }

    #[test]
    pub fn test_quote_identifier() {
        run_test("", "");
        run_test("_", "_");
        run_test("a", "a");
    }

    #[test]
    pub fn keywords_are_quoted() {
        for keyword in KEYWORDS.keys() {
            let quoted_keyword = format!(r#""{keyword}""#);
            run_test(keyword, &quoted_keyword);
        }
    }
}
