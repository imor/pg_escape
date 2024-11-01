# pg_escape

`pg_escape` is a Rust library to escape Postgres flavoured SQL.

To avoid SQL injection attacks it is necessary to properly escape user input. This library provides functions for that.

## quote_identifier

Use `quote_identifier` to properly quote an identifier. An identifier names a database object. E.g. names of tables, columns, view etc. are identifiers. Inability to quote user supplied identifiers leads to SQL injection attacks. For example, if your system accepts a table name from a user and runs a `select * from <table_name>` query, it is vulnerable to SQL injection attacks if constructed like this:

```rust
let table_name = "users";//supplied by user
let query = format!("select * from {table_name}");
```

Instead, do this:

```rust
use pg_escape::quote_identifier;

let table_name = "users";//supplied by user
let quoted_table_name = quote_identifier(table_name);
let query = format!("select * from {quoted_table_name}");
```

## quote_literal

Use `quote_literal` to properly quote a literal. A literal is a value which is written literally in a SQL expression. Similar to `quote_identifier`, ensure that user supplied literals are quoted. For example, don't do this:

```rust
let user = "john";//supplied by user
let query = format!("select * from users where username = {user}");
```

Do this instead:

```rust
use pg_escape::quote_literal;

let user = "john";//supplied by user
let quoted_user = quote_literal(user);
let query = format!("select * from users where username = {quoted_user}");
```

## When not to use `pg_escape`

Many Postgres client libraries and clients provide an option to run prepared statements (aka parameterized queries). Use them if available. `pg_escape` is useful for those constrained environments where prepared statements are not available. One example of such an environment is if you are connected to Postgres over a replication connection. A replication connection only supports a [simple query protocol](https://www.postgresql.org/docs/current/protocol-flow.html#PROTOCOL-FLOW-SIMPLE-QUERY) as mentioned in the Postgres [streaming replication protocol document](https://www.postgresql.org/docs/current/protocol-replication.html).
