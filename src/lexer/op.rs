use std::fmt;

#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq)]
pub enum NumOp {
    Add,
    Sub,
    Mul,
    Div,
    // Exp,
    // Mod,
}

impl fmt::Display for NumOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Add => "+",
                Self::Sub => "-",
                Self::Div => "/",
                Self::Mul => "*",
                // Self::Exp => "**",
                // Self::Mod => "%",
            }
        )
    }
}

#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq)]
pub enum UnaryOp {
    // -x
    Minus,
    // !x
    Not,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Minus => "-",
                Self::Not => "!",
            }
        )
    }
}

// TODO BitOp

#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq)]
pub enum CompOp {
    Equal,              // ==
    NotEqual,           // !=
    GreaterThan,        // >
    GreaterThanOrEqual, // >=
    LessThan,           // <
    LessThanOrEqual,    // <=
}

impl fmt::Display for CompOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Equal => "==",
                Self::NotEqual => "!=",
                Self::GreaterThan => ">",
                Self::GreaterThanOrEqual => ">=",
                Self::LessThan => "<",
                Self::LessThanOrEqual => "<=",
            }
        )
    }
}

#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq)]
pub enum LogOp {
    And,
    Or,
}

impl fmt::Display for LogOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::And => "&&",
                Self::Or => "||",
            }
        )
    }
}

// binary operator
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq)]
pub enum BinOp {
    Num(NumOp),
    // Bit(BitOp)
    Comp(CompOp),
    Log(LogOp),
    // TODO
    // Assign(AssignOp)
}

impl From<NumOp> for BinOp {
    fn from(op: NumOp) -> Self {
        Self::Num(op)
    }
}

impl From<CompOp> for BinOp {
    fn from(op: CompOp) -> Self {
        Self::Comp(op)
    }
}

impl From<LogOp> for BinOp {
    fn from(op: LogOp) -> Self {
        Self::Log(op)
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Num(ref op) => op.to_string(),
                Self::Comp(ref op) => op.to_string(),
                Self::Log(ref op) => op.to_string(),
            }
        )
    }
}

// TODO
// pub enum AssignOp
