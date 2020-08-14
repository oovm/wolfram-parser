use pratt::Precedence;

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
        Precedence(match self {
            Self::Plus => { 100 }
            Self::Minus => { 100 }
            Self::Times => { 100 }
            Self::Divide => { 100 }
            Self::Power => { 100 }
            Self::Prefix => { 100 }
            Self::Postfix => { 100 }
            Self::Subtract => { 100 }
            _ => unimplemented!()
        })
    }
}
