use std::cmp::PartialOrd;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(Debug)]
pub enum RuntimeValue {
    Integer(i64),
    Float(f64),
    String(String),
    Nil,
    Boolean(bool),
}

impl RuntimeValue {
    pub fn integer(value: i64) -> Self {
        Self::Integer(value)
    }
    pub fn float(value: f64) -> Self {
        Self::Float(value)
    }
    pub fn string(value: String) -> Self {
        Self::String(value)
    }
    pub fn nil() -> Self {
        Self::Nil
    }
    pub fn boolean(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl TryInto<i64> for &RuntimeValue {
    type Error = String;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            RuntimeValue::Integer(value) => Ok(*value),
            _ => Err("runtime value is not an Integer".to_string()),
        }
    }
}

impl TryInto<f64> for &RuntimeValue {
    type Error = String;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            RuntimeValue::Float(value) => Ok(*value),
            _ => Err("runtime value is not a Float".to_string()),
        }
    }
}

impl TryInto<String> for RuntimeValue {
    type Error = String;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            RuntimeValue::String(value) => Ok(value),
            _ => Err("runtime value is not a String".to_string()),
        }
    }
}

impl<'a> TryInto<&'a str> for &'a RuntimeValue {
    type Error = String;

    fn try_into(self) -> Result<&'a str, Self::Error> {
        match self {
            RuntimeValue::String(value) => Ok(value),
            _ => Err("runtime value is not a String".to_string()),
        }
    }
}

impl TryInto<bool> for &RuntimeValue {
    type Error = String;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            RuntimeValue::Boolean(value) => Ok(*value),
            _ => Err("runtime value is not a Boolean".to_string()),
        }
    }
}

impl TryInto<()> for &RuntimeValue {
    type Error = String;

    fn try_into(self) -> Result<(), Self::Error> {
        match self {
            RuntimeValue::Nil => Ok(()),
            _ => Err("runtime value is not a Nil".to_string()),
        }
    }
}

impl Not for &RuntimeValue {
    type Output = Option<RuntimeValue>;

    fn not(self) -> Self::Output {
        if let Ok(value) = <_ as TryInto<i64>>::try_into(self) {
            Some(RuntimeValue::boolean(!(value != 0)))
        } else if let Ok(value) = <_ as TryInto<f64>>::try_into(self) {
            Some(RuntimeValue::boolean(!(value != 0.)))
        } else if let Ok(value) = <_ as TryInto<bool>>::try_into(self) {
            Some(RuntimeValue::boolean(!value))
        } else if let Ok(_) = <_ as TryInto<&str>>::try_into(self) {
            Some(RuntimeValue::boolean(true))
        } else if let Ok(_) = <_ as TryInto<()>>::try_into(self) {
            Some(RuntimeValue::boolean(false))
        } else {
            None
        }
    }
}

impl Neg for &RuntimeValue {
    type Output = Option<RuntimeValue>;

    fn neg(self) -> Self::Output {
        if let Ok(value) = <_ as TryInto<i64>>::try_into(self) {
            Some(RuntimeValue::integer(-value))
        } else if let Ok(value) = <_ as TryInto<f64>>::try_into(self) {
            Some(RuntimeValue::float(-value))
        } else {
            None
        }
    }
}

impl Add for &RuntimeValue {
    type Output = Option<RuntimeValue>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            RuntimeValue::Integer(lhs) => match rhs {
                RuntimeValue::Integer(rhs) => Some(RuntimeValue::Integer(lhs + rhs)),
                RuntimeValue::Float(rhs) => Some(RuntimeValue::Float(*lhs as f64 + rhs)),
                _ => None,
            },
            RuntimeValue::Float(lhs) => match rhs {
                RuntimeValue::Integer(rhs) => Some(RuntimeValue::Float(lhs + *rhs as f64)),
                RuntimeValue::Float(rhs) => Some(RuntimeValue::Float(lhs + rhs)),
                _ => None,
            },
            RuntimeValue::String(lhs) => match rhs {
                RuntimeValue::String(rhs) => Some(RuntimeValue::String(format!("{lhs}{rhs}"))),
                RuntimeValue::Integer(rhs) => Some(RuntimeValue::String(format!("{lhs}{rhs}"))),
                RuntimeValue::Float(rhs) => Some(RuntimeValue::String(format!("{lhs}{rhs}"))),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Sub for &RuntimeValue {
    type Output = Option<RuntimeValue>;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            RuntimeValue::Integer(lhs) => match rhs {
                RuntimeValue::Integer(rhs) => Some(RuntimeValue::Integer(lhs - rhs)),
                RuntimeValue::Float(rhs) => Some(RuntimeValue::Float(*lhs as f64 - rhs)),
                _ => None,
            },
            RuntimeValue::Float(lhs) => match rhs {
                RuntimeValue::Integer(rhs) => Some(RuntimeValue::Float(lhs - *rhs as f64)),
                RuntimeValue::Float(rhs) => Some(RuntimeValue::Float(lhs - rhs)),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Mul for &RuntimeValue {
    type Output = Option<RuntimeValue>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            RuntimeValue::Integer(lhs) => match rhs {
                RuntimeValue::Integer(rhs) => Some(RuntimeValue::Integer(lhs * rhs)),
                RuntimeValue::Float(rhs) => Some(RuntimeValue::Float(*lhs as f64 * rhs)),
                _ => None,
            },
            RuntimeValue::Float(lhs) => match rhs {
                RuntimeValue::Integer(rhs) => Some(RuntimeValue::Float(lhs * *rhs as f64)),
                RuntimeValue::Float(rhs) => Some(RuntimeValue::Float(lhs * rhs)),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Div for &RuntimeValue {
    type Output = Option<RuntimeValue>;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            RuntimeValue::Integer(lhs) => match rhs {
                RuntimeValue::Integer(rhs) => {
                    let result = *lhs as f64 / *rhs as f64;

                    Some(if result.fract() == 0.0 {
                        RuntimeValue::Integer(result as i64)
                    } else {
                        RuntimeValue::Float(result)
                    })
                }
                RuntimeValue::Float(rhs) => Some(RuntimeValue::Float(*lhs as f64 / rhs)),
                _ => None,
            },
            RuntimeValue::Float(lhs) => match rhs {
                RuntimeValue::Integer(rhs) => Some(RuntimeValue::Float(lhs / *rhs as f64)),
                RuntimeValue::Float(rhs) => Some(RuntimeValue::Float(lhs / rhs)),
                _ => None,
            },
            _ => None,
        }
    }
}

impl PartialEq for RuntimeValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            RuntimeValue::Integer(lhs) => match other {
                RuntimeValue::Integer(rhs) => lhs == rhs,
                RuntimeValue::Float(rhs) => *lhs as f64 == *rhs,
                _ => false,
            },
            RuntimeValue::Float(lhs) => match other {
                RuntimeValue::Integer(rhs) => *lhs == *rhs as f64,
                RuntimeValue::Float(rhs) => lhs == rhs,
                _ => false,
            },
            RuntimeValue::String(lhs) => match other {
                RuntimeValue::String(rhs) => lhs == rhs,
                _ => false,
            },
            RuntimeValue::Boolean(lhs) => match other {
                RuntimeValue::Boolean(rhs) => lhs == rhs,
                _ => false,
            },
            _ => false,
        }
    }
}

impl PartialOrd for RuntimeValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            RuntimeValue::Integer(lhs) => match other {
                RuntimeValue::Integer(rhs) => lhs.partial_cmp(rhs),
                RuntimeValue::Float(rhs) => (*lhs as f64).partial_cmp(rhs),
                _ => None,
            },
            RuntimeValue::Float(lhs) => match other {
                RuntimeValue::Integer(rhs) => lhs.partial_cmp(&(*rhs as f64)),
                RuntimeValue::Float(rhs) => lhs.partial_cmp(rhs),
                _ => None,
            },
            _ => None,
        }
    }
}