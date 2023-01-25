use std::num::ParseFloatError;
#[allow(unused)]
#[derive(Debug)]
pub struct Money {
    amount: f32,
    currency: Currency,
}

impl Money {
    pub fn new(amount: f32, currency: Currency) -> Self {
        Self { amount, currency }
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}", self.amount, self.currency))
    }
}

impl std::str::FromStr for Money {
    type Err = MoneyError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.split_whitespace().collect::<Vec<_>>();
        match input[..] {
            [amount, currency] => Ok(Money::new(
                amount.parse().map_err(MoneyError::InvalidAmount)?,
                Currency::from_str(currency)?,
            )),
            _ => Err(MoneyError::InvalidArgs),
        }
    }
}

#[derive(Debug)]
pub enum Currency {
    Usd,
    Eur,
    Gbp,
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::Usd => write!(f, "$"),
            Currency::Eur => write!(f, "€"),
            Currency::Gbp => write!(f, "£"),
        }
    }
}

impl std::str::FromStr for Currency {
    type Err = MoneyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_uppercase().as_str() {
            "USD" | "$" => Ok(Self::Usd),
            "EUR" | "€" => Ok(Self::Eur),
            "GBP" | "£" => Ok(Self::Gbp),
            val => Err(MoneyError::InvalidCurrency(val.to_string())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MoneyError {
    #[error("Expected amount and currency")]
    InvalidArgs,
    #[error("Could not parse 'amount': {0:?}")]
    InvalidAmount(#[from] ParseFloatError),
    #[error("There is no currency with Code: {0:?}")]
    InvalidCurrency(String),
}
