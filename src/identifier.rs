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
/// Same as Postgres's [quote_ident](https://www.postgresql.org/docs/current/functions-string.html) function.
///
/// Returns the original identifier if it:
/// * Starts with a lowercase ASCII letter or underscore
/// * And contains only lowercase ASCII letters, digits, or underscore
/// * And is not a [SQL keyword](crate::keywords).
///
/// Otherwise, returns a quoted identifier. A quoted identifier:
/// * Starts and ends with double quotes.
/// * Any double quotes are replaced with two double quotes.
///
/// # Examples
///
/// ASCII identifiers returned as is:
/// ```
/// use pg_escape::quote_identifier;
///
/// let quoted_identifier = quote_identifier("_an_ident_1");
/// assert_eq!(quoted_identifier, "_an_ident_1");
/// ```
///
/// Non-ASCII identifier returned in quoted form:
/// ```
/// use pg_escape::quote_identifier;
///
/// let quoted_identifier = quote_identifier(r#"Some"Ident"#);
/// assert_eq!(quoted_identifier, r#""Some""Ident""#);
/// ```
///
/// Keywords are quoted:
/// ```
/// use pg_escape::quote_identifier;
///
/// let quoted_identifier = quote_identifier("select");
/// assert_eq!(quoted_identifier, r#""select""#);
/// ```
pub fn quote_identifier(identifier: &str) -> Cow<'_, str> {
    let mut chars = identifier.chars();
    let first = chars.next();
    let mut needs_quoting = first
        .map(|c| !is_lower_alphabetic_or_underscore(c))
        .unwrap_or(false);

    let mut num_quotes = 0;
    for c in chars {
        if !is_lower_alphanumeric_or_underscore(c) {
            needs_quoting = true;
        }
        if c == '"' {
            num_quotes += 1;
        }
    }

    if !needs_quoting && parse_keyword(identifier).is_some() {
        needs_quoting = true;
    }

    if needs_quoting {
        num_quotes += 2; //two extra quotes surrounding the identifier
        let capacity = identifier.len() + num_quotes;
        let mut result = String::with_capacity(capacity);
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
    pub fn empty_str_ident_is_not_quoted() {
        run_test("", "");
    }

    #[test]
    pub fn underscore_ident_is_not_quoted() {
        run_test("_", "_");
    }

    #[test]
    pub fn lowercase_alphabet_idents_are_not_quoted() {
        for c in 'a'..='z' {
            let ident = c.to_string();
            run_test(&ident, &ident);
        }
    }

    #[test]
    pub fn idents_with_numerics_in_the_middle_are_not_quoted() {
        for c in '0'..='9' {
            let ident = format!("_asdf{c}qwer");
            run_test(&ident, &ident);
        }
    }

    #[test]
    pub fn uppercase_alphabet_idents_are_quoted() {
        for c in 'A'..='Z' {
            let ident = c.to_string();
            let expected = format!(r#""{ident}""#);
            run_test(&ident, &expected);
        }
    }

    #[test]
    pub fn idents_starting_with_numerics_are_quoted() {
        for c in '0'..='9' {
            let ident = format!("{c}uiop");
            let expected = format!(r#""{ident}""#);
            run_test(&ident, &expected);
        }
    }

    #[test]
    pub fn keywords_are_quoted() {
        for keyword in KEYWORDS.keys() {
            let quoted_keyword = format!(r#""{keyword}""#);
            run_test(keyword, &quoted_keyword);
        }
    }

    #[test]
    pub fn idents_with_double_quote_are_quoted() {
        run_test(r#"""#, r#""""""#);
        run_test(r#"asdf"qwer"#, r#""asdf""qwer""#);
    }

    #[test]
    pub fn non_ascii_idents_are_quoted() {
        run_test("हिन्दी", r#""हिन्दी""#);
        run_test("mIxEdCaSe", r#""mIxEdCaSe""#);
    }
}
