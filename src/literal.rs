// Adapted from https://github.com/postgres/postgres/blob/b82c877e76e2398409e823773413079668cf1881/src/backend/utils/adt/quote.c#L47
/// Same as Postgres's [quote_literal](https://www.postgresql.org/docs/current/functions-string.html) function.
///
/// The literal is always wrapped in a pair of single quotes. If the literal contains a backslash, it is also prefixed with an `E`. A backslash or a single quote is repeated.
///
/// # Examples
///
/// ```
/// use pg_escape::quote_literal;
///
/// let quoted_literal = quote_literal(r"Some\Literal'");
/// assert_eq!(quoted_literal, r"E'Some\\Literal'''");
/// ```
pub fn quote_literal(literal: &str) -> String {
    let mut needs_e_prefix = false;
    let mut num_repeat_chars = 0;
    for c in literal.chars() {
        if c == '\\' {
            needs_e_prefix = true;
            num_repeat_chars += 1;
        } else if c == '\'' {
            num_repeat_chars += 1;
        }
    }

    const NUM_QUOTES: usize = 2;
    let capacity =
        literal.len() + NUM_QUOTES + num_repeat_chars + if needs_e_prefix { 1 } else { 0 };
    let mut result = String::with_capacity(capacity);

    if needs_e_prefix {
        result.push('E');
    }

    result.push('\'');
    for c in literal.chars() {
        if c == '\\' || c == '\'' {
            result.push(c);
        }
        result.push(c);
    }
    result.push('\'');

    result
}

#[cfg(test)]
mod test {
    use crate::quote_literal;

    fn run_test(literal: &str, expected_quoted_literal: &str) {
        let actual_quoted_literal = quote_literal(literal);
        assert_eq!(actual_quoted_literal, expected_quoted_literal);
    }

    #[test]
    fn empty_string_is_not_prefixed_with_e() {
        run_test("", "''");
    }

    #[test]
    fn backslash_is_prefixed_with_e() {
        run_test(r"\", r"E'\\'");
    }

    #[test]
    fn quote_is_not_prefixed_with_e() {
        run_test(r"'", r"''''");
    }

    #[test]
    fn quotes_and_backslashes_are_escaped() {
        run_test(r"asdf'qwer\uiop", r"E'asdf''qwer\\uiop'");
    }
}
