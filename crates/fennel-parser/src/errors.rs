use rowan::TextRange;

use crate::{models::ValueKind, syntax::SyntaxKind};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorKind {
    Unexpected(SyntaxKind),
    UnexpectedVarargs,
    MultiVarargs,
    UnexpectedEof,
    EmptyList,
    Dismatched,
    Unterminated(SyntaxKind),
    Undefined,
    Unused,
    GlobalConflict,
    MissingWhitespace,
    MacroWhitespace,
    InvalidSymbol,
    MethodNotAllowed,
    FieldAndMethodNotAllowed,
    LiteralCall(ValueKind),
    DirectCall(ValueKind),
    MultiCatch,
    CatchNotLast,
    Deprecated(&'static str, &'static str),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Error {
    pub range: TextRange,
    pub kind: ErrorKind,
}

impl Error {
    pub(crate) fn new(range: TextRange, kind: ErrorKind) -> Self {
        Error { range, kind }
    }
}
