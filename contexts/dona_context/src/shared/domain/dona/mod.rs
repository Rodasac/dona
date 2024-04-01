use std::fmt::Display;

pub const ERR_INVALID_DONA_OPTION_METHOD: &str = "Invalid Dona Option Method";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum DonaOptionMethod {
    Manual,
    Paypal,
}

impl DonaOptionMethod {
    pub fn new(value: String) -> Result<Self, String> {
        match value.as_str() {
            "manual" => Ok(Self::Manual),
            "paypal" => Ok(Self::Paypal),
            _ => Err(ERR_INVALID_DONA_OPTION_METHOD.to_string()),
        }
    }
}

impl Display for DonaOptionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::Paypal => write!(f, "paypal"),
        }
    }
}

pub mod tests {
    use fake::{Dummy, Fake};
    use rand::seq::SliceRandom;

    use super::*;

    struct DonaOptionMethodFaker;

    impl Dummy<DonaOptionMethodFaker> for DonaOptionMethod {
        fn dummy_with_rng<R: rand::Rng + ?Sized>(
            _config: &DonaOptionMethodFaker,
            rng: &mut R,
        ) -> Self {
            let values = vec![DonaOptionMethod::Manual, DonaOptionMethod::Paypal];
            values.choose(rng).unwrap().clone()
        }
    }

    pub struct DonaOptionMethodMother;

    impl DonaOptionMethodMother {
        pub fn random() -> DonaOptionMethod {
            DonaOptionMethodFaker.fake()
        }

        pub fn create(value: Option<String>) -> DonaOptionMethod {
            match value {
                Some(value) => DonaOptionMethod::new(value).unwrap(),
                None => Self::random(),
            }
        }
    }
}
