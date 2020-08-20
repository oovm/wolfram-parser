use super::*;

/// A lossless numerical representation of wolfram
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WolframNumber {
    /// The base of the number
    pub base: u8,
    /// The power of th number
    pub power: isize,
    /// Integer part of the number
    pub integer: String,
    /// The digits after `.`
    pub digits: String,
    /// The residual control
    pub control: NumberResidual,
    /// The define position
    pub span: Range<usize>,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NumberResidual {
    Accuracy(f64),
    Precision(f64),
    /// 53 log₁₀2 = 15.954589770191003
    MachinePrecision,
}

impl Eq for NumberResidual {}

impl PartialEq for NumberResidual {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Accuracy(a), Self::Accuracy(b)) => a == b,
            (Self::Precision(a), Self::Precision(b)) => a == b,
            (Self::MachinePrecision, Self::MachinePrecision) => true,
            _ => false,
        }
    }
}

impl Hash for NumberResidual {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Accuracy(v) => {
                state.write_u8(0);
                state.write_u64(v.to_bits());
            }
            Self::Precision(v) => {
                state.write_u8(1);
                state.write_u64(v.to_bits());
            }
            Self::MachinePrecision => {
                state.write_u8(2);
            }
        }
    }
}

impl Display for WolframNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.base != 10 {
            f.write_str(&format!("{}^^", self.base))?;
        }
        f.write_str(&self.integer)?;
        if !self.digits.is_empty() {
            f.write_str(".")?;
            f.write_str(&self.digits)?;
            match self.control {
                NumberResidual::Accuracy(v) => {
                    f.write_str("``")?;
                    f.write_str(&v.to_string())?;
                }
                NumberResidual::Precision(v) => {
                    f.write_str("`")?;
                    f.write_str(&v.to_string())?;
                }
                NumberResidual::MachinePrecision => {}
            }
        }
        if self.power != 0 {
            f.write_str("*^")?;
            f.write_str(&self.power.to_string())?;
        }
        Ok(())
    }
}

impl WolframNumber {
    /// Create a base 10 integer number
    pub fn integer(text: String, span: Range<usize>) -> Self {
        Self {
            base: 10,
            power: 0,
            integer: text,
            digits: String::new(),
            control: NumberResidual::Accuracy(f64::INFINITY),
            span,
        }
    }
    /// Create a base 10 decimal number
    pub fn decimal(integer: String, decimal: String, span: Range<usize>) -> Self {
        if decimal.len() < 15 {
            Self { base: 10, power: 0, integer, digits: decimal, control: NumberResidual::MachinePrecision, span }
        }
        else {
            let control = NumberResidual::Accuracy(decimal.len() as f64);
            Self { base: 10, power: 0, integer, digits: decimal, control, span }
        }
    }
}

#[test]
fn number_display() {
    let input = WolframNumber {
        base: 2,
        power: -2,
        integer: "101".to_string(),
        digits: "111".to_string(),
        control: NumberResidual::Precision(101.111),
        span: 0..17,
    };
    let output = "2^^101.111`101.111*^-2";
    assert_eq!(input.to_string(), output);
    let input = WolframNumber::integer("101".to_string(), 0..3);
    let output = "101";
    assert_eq!(input.to_string(), output);
}
