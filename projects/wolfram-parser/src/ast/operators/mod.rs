use pratt::{Affix, Associativity, Precedence};

/// Wolfram Language operators
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum WolframOperator {
    /// [Not, !](https://reference.wolfram.com/language/ref/Not.html)
    Not,
    /// [Plus, +](https://reference.wolfram.com/language/ref/Plus.html)
    Plus,
    /// [Minus, -](https://reference.wolfram.com/language/ref/Minus.html)
    Minus,
    /// [Subtract, -](https://reference.wolfram.com/language/ref/Subtract.html)
    Subtract,
    /// [Times, *](https://reference.wolfram.com/language/ref/Times.html)
    Times,
    /// [Divide, /](https://reference.wolfram.com/language/ref/Divide.html)
    Divide,
    /// [Power, ^](https://reference.wolfram.com/language/ref/Power.html)
    Power,
    /// [Map, /@](https://reference.wolfram.com/language/ref/Map.html)
    Map,
    /// [Prefix, @](https://reference.wolfram.com/language/ref/Prefix.html)
    Prefix,
    /// [Postfix, //](https://reference.wolfram.com/language/ref/Postfix.html)
    Postfix,
    /// [Infix, ~~](https://reference.wolfram.com/language/ref/Infix.html)
    Infix(String),
}

impl WolframOperator {
    /// Get the precedence of expression
    pub fn precedence(&self) -> Precedence {
        // `Precedence /@ {Plus, Minus}`
        Precedence(match self {
            // prefix
            Self::Minus => 480,
            // infix
            Self::Plus => 310,
            Self::Subtract => 310,
            Self::Times => 400,
            Self::Divide => 470,
            Self::Power => 590,
            Self::Map => 620,
            _ => unimplemented!("{self:?}"),
        })
    }
    /// Get the precedence and associativity of an expression
    pub fn as_affix(&self) -> Affix {
        match self {
            // prefix
            Self::Minus => Affix::Prefix(self.precedence()),
            // infix
            Self::Plus | Self::Subtract | Self::Times | Self::Divide => Affix::Infix(self.precedence(), Associativity::Left),
            Self::Map => Affix::Infix(self.precedence(), Associativity::Left),
            Self::Power => Affix::Infix(self.precedence(), Associativity::Right),
            _ => unimplemented!("{self:?}"),
        }
    }
}
