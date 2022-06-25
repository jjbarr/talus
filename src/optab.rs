//TODO, move from &str to &[u8]
#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Roll,
    Rept,
    KLow,
    KHigh,
    DLow,
    DHigh,
    Eq,
    Lt,
    Gt,
    Ge,
    Le,
    Lkup,
    Bang,
    Twice,
    Reroll,
    Comment,
    Success,
    Brief,
    Debug,
    Map,
    Also
}

/// The fearsome operator table macro is used to define binding power for all
/// operators and define functions to look up an operator and its bp.
macro_rules! op_table {
    {
        left => {$($lop:literal => ($lstruct:expr, $lrbp:literal)),*},
        right => {$($rop:literal => ($rlbp:literal, $rstruct:expr)),*},
        infix => {$($iop:literal => ($ilbp:literal, $istruct:expr,
                                     $irbp:literal)),*}
    } => {
        pub fn left_op(op: &[u8]) -> Option<(Op, u16)> {
            match op {
                $($lop => Some(($lstruct, $lrbp)),)*
                    _ => None
            }
        }

        pub fn right_op(op: &[u8]) -> Option<(u16, Op)> {
            match op {
                $($rop => Some(($rlbp, $rstruct)),)*
                    _ => None
            }
        }

        pub fn infix_op(op: &[u8]) -> Option<(u16, Op, u16)> {
            match op {
                $($iop => Some(($ilbp, $istruct, $irbp)),)*
                    _ => None
            }
        }
    };
}

/* this is no op with a bp of 1 for dumb reasons. */

// So, I actually want to make this prefix free for leftmost-longest.
// Can we have l be a modifier operator or something?
op_table! {
    left => {
        b"-" => (Op::Neg, 14)
    },
    right => {
        b"[" => (7, Op::Lkup),
        b"!" => (7, Op::Bang),
        b"S" => (7, Op::Success),
        b"B" => (7, Op::Brief),
        b"D" => (7, Op::Debug),
        b"\\" => (7, Op::Map)
    },
    infix => {
        b"," => (1, Op::Also, 2),
        b"+" => (3, Op::Add, 4),
        b"-" => (3, Op::Sub, 4),
        b"/" => (5, Op::Div, 6),
        b"*" => (5, Op::Mul, 6),
        b"r" => (9, Op::Rept, 10),
        b"d" => (11, Op::Roll, 12),
        b"k" => (7, Op::KHigh, 13),
        b"kl" => (7, Op::KLow, 13),
        b"dh" => (7, Op::DHigh, 13),
        b"dl" => (7, Op::DLow, 13),
        b"=" => (7, Op::Eq, 13),
        b"<" => (7, Op::Lt, 13),
        b">" => (7, Op::Gt, 13),
        b"<=" => (7, Op::Le, 13),
        b">=" => (7, Op::Ge, 13),
        b"t" => (7, Op::Twice, 13),
        b"re" => (7, Op::Reroll, 13)
    }
}

pub fn is_op(op: &[u8]) -> bool {
    left_op(op).and(Some(()))
        .or_else(|| right_op(op).and(Some(())))
        .or_else(|| infix_op(op).and(Some(())))
        .map_or(false, |_| true)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_left() {
        assert_eq!(left_op(b"-"), Some((Op::Neg, 14)));
        assert_eq!(left_op(b"+"), None);
    }
    
    #[test]
    fn test_right() {
        assert_eq!(right_op(b"["), Some((7, Op::Lkup)));
        assert_eq!(right_op(b"M"), None);
    }

    #[test]
    fn test_infix() {
        assert_eq!(infix_op(b"<"), Some((7, Op::Lt, 14)));
        assert_eq!(infix_op(b"+"), Some((3, Op::Add, 4)));
        assert_eq!(infix_op(b"!"), None);
    }

    #[test]
    fn test_is_op() {
        assert_eq!(is_op(b"-"), true);
        assert_eq!(is_op(b"["), true);
        assert_eq!(is_op(b"\\"), true);
        assert_eq!(is_op(b"M"), false);
    }
}
