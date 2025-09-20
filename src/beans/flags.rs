use crate::Helpers;

/// entry flags
///
/// see https://beancount.github.io/docs/beancount_design_doc.html#flag
/// note: rust does not allow string literal discriminants in an enum: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.repr-rust
///  but we CAN use [char](https://doc.rust-lang.org/std/primitive.char.html)s as [u8](https://doc.rust-lang.org/std/primitive.u8.html) bytes 
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub(crate) enum Flags {
    Conversion = b'C',
    Merging = b'M',
    /// posted
    Okay = b'*',
    Padding = b'P',
    Returns = b'R',
    Summarize = b'S',
    Transfer = b'T',
    /// plugin "beancount.plugins.unrealized" "Unrealized"
    Unrealized = b'U',
    /// unposted
    Warning = b'!',
}

impl From<Flags> for u8 {
    fn from(flag: Flags) -> Self {
        flag as u8
    }
}

impl TryFrom<u8> for Flags {
    type Error = Helpers;
    
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            b'C' | b'c' => Ok(Self::Conversion),
            b'M' | b'm' => Ok(Self::Merging),
            b'*' => Ok(Self::Okay),
            b'P' | b'p' => Ok(Self::Padding),
            b'R' | b'r' => Ok(Self::Returns),
            b'S' | b's' => Ok(Self::Summarize),
            b'T' | b't' => Ok(Self::Transfer),
            b'U' | b'u' => Ok(Self::Unrealized),
            b'!' => Ok(Self::Warning),
            _ => Err(Helpers::BeancountError(format!("Invalid flag byte: {}", byte))),
        }
    }
}

// TODO: impl From<char> for Flags
// TODO: impl From<Flags> for char

impl std::str::FromStr for Flags {
    type Err = Helpers;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // match s.chars().next() {
        //     Some('C') | Some('c') => Ok(Self::Conversion),
        // }

        match s {
            "C" | "c" => Ok(Self::Conversion),
            "M" | "m" => Ok(Self::Merging),
            "*" => Ok(Self::Okay),
            "P" | "p" => Ok(Self::Padding),
            "R" | "r" => Ok(Self::Returns),
            "S" | "s" => Ok(Self::Summarize),
            "T" | "t" => Ok(Self::Transfer),
            "U" | "u" => Ok(Self::Unrealized),
            "!" => Ok(Self::Warning),
            invalid_str => Err(Helpers::BeancountError(format!("conversion error: {invalid_str} is not a valid variant of Flags")))
        }
    }
}

impl Flags {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Conversion => "C",
            Self::Merging => "M",
            Self::Okay => "*",
            Self::Padding => "P",
            Self::Returns => "R",
            Self::Summarize => "S",
            Self::Transfer => "T",
            Self::Unrealized => "U",
            Self::Warning => "!"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    
    #[test]
    fn into() {
        assert_eq!(Flags::from_str("C"), Ok(Flags::Conversion));
        assert_eq!(Flags::from_str("c"), Ok(Flags::Conversion));
        assert_eq!(Flags::from_str("M"), Ok(Flags::Merging));
        assert_eq!(Flags::from_str("m"), Ok(Flags::Merging));
        assert_eq!(Flags::from_str("*"), Ok(Flags::Okay));
        assert_eq!(Flags::from_str("P"), Ok(Flags::Padding));
        assert_eq!(Flags::from_str("p"), Ok(Flags::Padding));
        assert_eq!(Flags::from_str("R"), Ok(Flags::Returns));
        assert_eq!(Flags::from_str("r"), Ok(Flags::Returns));
        assert_eq!(Flags::from_str("S"), Ok(Flags::Summarize));
        assert_eq!(Flags::from_str("s"), Ok(Flags::Summarize));
        assert_eq!(Flags::from_str("T"), Ok(Flags::Transfer));
        assert_eq!(Flags::from_str("t"), Ok(Flags::Transfer));
        assert_eq!(Flags::from_str("U"), Ok(Flags::Unrealized));
        assert_eq!(Flags::from_str("u"), Ok(Flags::Unrealized));
        assert_eq!(Flags::from_str("!"), Ok(Flags::Warning));
        // TODO: assert_eq!(Flags::from_str("A"), Err(Helpers::BeancountError));
    }
    
    #[test]
    fn from() {
        assert_eq!(Flags::Conversion.as_str(), "C");
        assert_ne!(Flags::Conversion.as_str(), "c");
        assert_eq!(Flags::Merging.as_str(), "M");
        assert_ne!(Flags::Merging.as_str(), "m");
        assert_eq!(Flags::Okay.as_str(), "*");
        assert_ne!(Flags::Okay.as_str(), "8");
        assert_eq!(Flags::Padding.as_str(), "P");
        assert_ne!(Flags::Padding.as_str(), "p");
        assert_eq!(Flags::Returns.as_str(), "R");
        assert_ne!(Flags::Returns.as_str(), "r");
        assert_eq!(Flags::Summarize.as_str(), "S");
        assert_ne!(Flags::Summarize.as_str(), "s");
        assert_eq!(Flags::Transfer.as_str(), "T");
        assert_ne!(Flags::Transfer.as_str(), "t");
        assert_eq!(Flags::Unrealized.as_str(), "U");
        assert_ne!(Flags::Unrealized.as_str(), "u");
        assert_eq!(Flags::Warning.as_str(), "!");
        assert_ne!(Flags::Warning.as_str(), "1");
    }
}