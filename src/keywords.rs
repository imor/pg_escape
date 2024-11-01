//! The following identifiers are recognized as keywords, and will be quoted by the [quote_identifier](crate::quote_identifier) function:
//! * all
//! * analyse
//! * analyze
//! * and
//! * any
//! * array
//! * as
//! * asc
//! * asymmetric
//! * authorization
//! * between
//! * bigint
//! * binary
//! * bit
//! * boolean
//! * both
//! * case
//! * cast
//! * char
//! * character
//! * check
//! * coalesce
//! * collate
//! * collation
//! * column
//! * concurrently
//! * constraint
//! * create
//! * cross
//! * current_catalog
//! * current_date
//! * current_role
//! * current_schema
//! * current_time
//! * current_timestamp
//! * current_user
//! * dec
//! * decimal
//! * default
//! * deferrable
//! * desc
//! * distinct
//! * do
//! * else
//! * end
//! * except
//! * exists
//! * extract
//! * false
//! * fetch
//! * float
//! * for
//! * foreign
//! * freeze
//! * from
//! * full
//! * grant
//! * greatest
//! * group
//! * grouping
//! * having
//! * ilike
//! * in
//! * initially
//! * inner
//! * inout
//! * int
//! * integer
//! * intersect
//! * interval
//! * into
//! * is
//! * isnull
//! * join
//! * json
//! * json_array
//! * json_arrayagg
//! * json_exists
//! * json_object
//! * json_objectagg
//! * json_query
//! * json_scalar
//! * json_serialize
//! * json_table
//! * json_value
//! * lateral
//! * leading
//! * least
//! * left
//! * like
//! * limit
//! * localtime
//! * localtimestamp
//! * merge_action
//! * national
//! * natural
//! * nchar
//! * none
//! * normalize
//! * not
//! * notnull
//! * null
//! * nullif
//! * numeric
//! * offset
//! * on
//! * only
//! * or
//! * order
//! * out
//! * outer
//! * overlaps
//! * overlay
//! * placing
//! * position
//! * precision
//! * primary
//! * real
//! * references
//! * returning
//! * right
//! * row
//! * select
//! * session_user
//! * setof
//! * similar
//! * smallint
//! * some
//! * substring
//! * symmetric
//! * system_user
//! * table
//! * tablesample
//! * then
//! * time
//! * timestamp
//! * to
//! * trailing
//! * treat
//! * trim
//! * true
//! * union
//! * unique
//! * user
//! * using
//! * values
//! * varchar
//! * variadic
//! * verbose
//! * when
//! * where
//! * window
//! * with
//! * xmlattributes
//! * xmlconcat
//! * xmlelement
//! * xmlexists
//! * xmlforest
//! * xmlnamespaces
//! * xmlparse
//! * xmlpi
//! * xmlroot
//! * xmlserialize
//! * xmltable

// Keywords in this file are taken from https://github.com/postgres/postgres/blob/master/src/include/parser/kwlist.h
// Only keywords which are not UNRESERVED_KEYWORD are taken.
// TODO: Automtically generate code in this file instead of manually copying it.
use phf::phf_map;

#[derive(Clone)]
pub(crate) enum Keyword {
    All,
    Analyse,
    Analyze,
    And,
    Any,
    Array,
    As,
    Asc,
    Asymmetric,
    Authorization,
    Between,
    Bigint,
    Binary,
    Bit,
    Boolean,
    Both,
    Case,
    Cast,
    Char,
    Character,
    Check,
    Coalesce,
    Collate,
    Collation,
    Column,
    Concurrently,
    Constraint,
    Create,
    Cross,
    CurrentCatalog,
    CurrentDate,
    CurrentRole,
    CurrentSchema,
    CurrentTime,
    CurrentTimestamp,
    CurrentUser,
    Dec,
    Decimal,
    Default,
    Deferrable,
    Desc,
    Distinct,
    Do,
    Else,
    End,
    Except,
    Exists,
    Extract,
    False,
    Fetch,
    Float,
    For,
    Foreign,
    Freeze,
    From,
    Full,
    Grant,
    Greatest,
    Group,
    Grouping,
    Having,
    Ilike,
    In,
    Initially,
    Inner,
    Inout,
    Int,
    Integer,
    Intersect,
    Interval,
    Into,
    Is,
    Isnull,
    Join,
    Json,
    JsonArray,
    JsonArrayagg,
    JsonExists,
    JsonObject,
    JsonObjectAgg,
    JsonQuery,
    JsonScalar,
    JsonSerialize,
    JsonTable,
    JsonValue,
    Lateral,
    Leading,
    Least,
    Left,
    Like,
    Limit,
    Localtime,
    Localtimestamp,
    MergeAction,
    National,
    Natural,
    Nchar,
    None,
    Normalize,
    Not,
    Notnull,
    Null,
    Nullif,
    Numeric,
    Offset,
    On,
    Only,
    Or,
    Order,
    Out,
    Outer,
    Overlaps,
    Overlay,
    Placing,
    Position,
    Precision,
    Primary,
    Real,
    References,
    Returning,
    Right,
    Row,
    Select,
    SessionUser,
    Setof,
    Similar,
    Smallint,
    Some,
    Substring,
    Symmetric,
    SystemUser,
    Table,
    Tablesample,
    Then,
    Time,
    Timestamp,
    To,
    Trailing,
    Treat,
    Trim,
    True,
    Union,
    Unique,
    User,
    Using,
    Values,
    Varchar,
    Variadic,
    Verbose,
    When,
    Where,
    Window,
    With,
    Xmlattributes,
    Xmlconcat,
    Xmlelement,
    Xmlexists,
    Xmlforest,
    Xmlnamespaces,
    Xmlparse,
    Xmlpi,
    Xmlroot,
    Xmlserialize,
    Xmltable,
}

pub(crate) static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "all" => Keyword::All,
    "analyse" => Keyword::Analyse,
    "analyze" => Keyword::Analyze,
    "and" => Keyword::And,
    "any" => Keyword::Any,
    "array" => Keyword::Array,
    "as" => Keyword::As,
    "asc" => Keyword::Asc,
    "asymmetric" => Keyword::Asymmetric,
    "authorization" => Keyword::Authorization,
    "between" => Keyword::Between,
    "bigint" => Keyword::Bigint,
    "binary" => Keyword::Binary,
    "bit" => Keyword::Bit,
    "boolean" => Keyword::Boolean,
    "both" => Keyword::Both,
    "case" => Keyword::Case,
    "cast" => Keyword::Cast,
    "char" => Keyword::Char,
    "character" => Keyword::Character,
    "check" => Keyword::Check,
    "coalesce" => Keyword::Coalesce,
    "collate" => Keyword::Collate,
    "collation" => Keyword::Collation,
    "column" => Keyword::Column,
    "concurrently" => Keyword::Concurrently,
    "constraint" => Keyword::Constraint,
    "create" => Keyword::Create,
    "cross" => Keyword::Cross,
    "current_catalog" => Keyword::CurrentCatalog,
    "current_date" => Keyword::CurrentDate,
    "current_role" => Keyword::CurrentRole,
    "current_schema" => Keyword::CurrentSchema,
    "current_time" => Keyword::CurrentTime,
    "current_timestamp" => Keyword::CurrentTimestamp,
    "current_user" => Keyword::CurrentUser,
    "dec" => Keyword::Dec,
    "decimal" => Keyword::Decimal,
    "default" => Keyword::Default,
    "deferrable" => Keyword::Deferrable,
    "desc" => Keyword::Desc,
    "distinct" => Keyword::Distinct,
    "do" => Keyword::Do,
    "else" => Keyword::Else,
    "end" => Keyword::End,
    "except" => Keyword::Except,
    "exists" => Keyword::Exists,
    "extract" => Keyword::Extract,
    "false" => Keyword::False,
    "fetch" => Keyword::Fetch,
    "float" => Keyword::Float,
    "for" => Keyword::For,
    "foreign" => Keyword::Foreign,
    "freeze" => Keyword::Freeze,
    "from" => Keyword::From,
    "full" => Keyword::Full,
    "grant" => Keyword::Grant,
    "greatest" => Keyword::Greatest,
    "group" => Keyword::Group,
    "grouping" => Keyword::Grouping,
    "having" => Keyword::Having,
    "ilike" => Keyword::Ilike,
    "in" => Keyword::In,
    "initially" => Keyword::Initially,
    "inner" => Keyword::Inner,
    "inout" => Keyword::Inout,
    "int" => Keyword::Int,
    "integer" => Keyword::Integer,
    "intersect" => Keyword::Intersect,
    "interval" => Keyword::Interval,
    "into" => Keyword::Into,
    "is" => Keyword::Is,
    "isnull" => Keyword::Isnull,
    "join" => Keyword::Join,
    "json" => Keyword::Json,
    "json_array" => Keyword::JsonArray,
    "json_arrayagg" => Keyword::JsonArrayagg,
    "json_exists" => Keyword::JsonExists,
    "json_object" => Keyword::JsonObject,
    "json_objectagg" => Keyword::JsonObjectAgg,
    "json_query" => Keyword::JsonQuery,
    "json_scalar" => Keyword::JsonScalar,
    "json_serialize" => Keyword::JsonSerialize,
    "json_table" => Keyword::JsonTable,
    "json_value" => Keyword::JsonValue,
    "lateral" => Keyword::Lateral,
    "leading" => Keyword::Leading,
    "least" => Keyword::Least,
    "left" => Keyword::Left,
    "like" => Keyword::Like,
    "limit" => Keyword::Limit,
    "localtime" => Keyword::Localtime,
    "localtimestamp" => Keyword::Localtimestamp,
    "merge_action" => Keyword::MergeAction,
    "national" => Keyword::National,
    "natural" => Keyword::Natural,
    "nchar" => Keyword::Nchar,
    "none" => Keyword::None,
    "normalize" => Keyword::Normalize,
    "not" => Keyword::Not,
    "notnull" => Keyword::Notnull,
    "null" => Keyword::Null,
    "nullif" => Keyword::Nullif,
    "numeric" => Keyword::Numeric,
    "offset" => Keyword::Offset,
    "on" => Keyword::On,
    "only" => Keyword::Only,
    "or" => Keyword::Or,
    "order" => Keyword::Order,
    "out" => Keyword::Out,
    "outer" => Keyword::Outer,
    "overlaps" => Keyword::Overlaps,
    "overlay" => Keyword::Overlay,
    "placing" => Keyword::Placing,
    "position" => Keyword::Position,
    "precision" => Keyword::Precision,
    "primary" => Keyword::Primary,
    "real" => Keyword::Real,
    "references" => Keyword::References,
    "returning" => Keyword::Returning,
    "right" => Keyword::Right,
    "row" => Keyword::Row,
    "select" => Keyword::Select,
    "session_user" => Keyword::SessionUser,
    "setof" => Keyword::Setof,
    "similar" => Keyword::Similar,
    "smallint" => Keyword::Smallint,
    "some" => Keyword::Some,
    "substring" => Keyword::Substring,
    "symmetric" => Keyword::Symmetric,
    "system_user" => Keyword::SystemUser,
    "table" => Keyword::Table,
    "tablesample" => Keyword::Tablesample,
    "then" => Keyword::Then,
    "time" => Keyword::Time,
    "timestamp" => Keyword::Timestamp,
    "to" => Keyword::To,
    "trailing" => Keyword::Trailing,
    "treat" => Keyword::Treat,
    "trim" => Keyword::Trim,
    "true" => Keyword::True,
    "union" => Keyword::Union,
    "unique" => Keyword::Unique,
    "user" => Keyword::User,
    "using" => Keyword::Using,
    "values" => Keyword::Values,
    "varchar" => Keyword::Varchar,
    "variadic" => Keyword::Variadic,
    "verbose" => Keyword::Verbose,
    "when" => Keyword::When,
    "where" => Keyword::Where,
    "window" => Keyword::Window,
    "with" => Keyword::With,
    "xmlattributes" => Keyword::Xmlattributes,
    "xmlconcat" => Keyword::Xmlconcat,
    "xmlelement" => Keyword::Xmlelement,
    "xmlexists" => Keyword::Xmlexists,
    "xmlforest" => Keyword::Xmlforest,
    "xmlnamespaces" => Keyword::Xmlnamespaces,
    "xmlparse" => Keyword::Xmlparse,
    "xmlpi" => Keyword::Xmlpi,
    "xmlroot" => Keyword::Xmlroot,
    "xmlserialize" => Keyword::Xmlserialize,
    "xmltable" => Keyword::Xmltable,
};

pub(crate) fn parse_keyword(keyword: &str) -> Option<Keyword> {
    KEYWORDS.get(keyword).cloned()
}
