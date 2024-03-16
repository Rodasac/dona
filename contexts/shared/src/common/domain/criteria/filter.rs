use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilterField {
    value: String,
}

impl TryFrom<String> for FilterField {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("FilterField is empty".to_string());
        }

        Ok(Self { value })
    }
}

impl Display for FilterField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like,
    NotLike,
    In,
    NotIn,
}

impl FilterOperator {
    pub fn is_positive(&self) -> bool {
        !matches!(self, Self::NotEqual | Self::NotIn | Self::NotLike)
    }
}

impl TryFrom<String> for FilterOperator {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "=" => Ok(Self::Equal),
            "!=" => Ok(Self::NotEqual),
            ">" => Ok(Self::GreaterThan),
            ">=" => Ok(Self::GreaterThanOrEqual),
            "<" => Ok(Self::LessThan),
            "<=" => Ok(Self::LessThanOrEqual),
            "LIKE" => Ok(Self::Like),
            "NOT LIKE" => Ok(Self::NotLike),
            "IN" => Ok(Self::In),
            "NOT IN" => Ok(Self::NotIn),
            _ => Err("Invalid FilterOperator".to_string()),
        }
    }
}

impl Display for FilterOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Equal => write!(f, "="),
            Self::NotEqual => write!(f, "!="),
            Self::GreaterThan => write!(f, ">"),
            Self::GreaterThanOrEqual => write!(f, ">="),
            Self::LessThan => write!(f, "<"),
            Self::LessThanOrEqual => write!(f, "<="),
            Self::Like => write!(f, "LIKE"),
            Self::NotLike => write!(f, "NOT LIKE"),
            Self::In => write!(f, "IN"),
            Self::NotIn => write!(f, "NOT IN"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilterValue {
    value: String,
}

impl TryFrom<String> for FilterValue {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("FilterValue is empty".to_string());
        }

        Ok(Self { value })
    }
}

impl Display for FilterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Filter {
    field: FilterField,
    operator: FilterOperator,
    value: FilterValue,
}

impl Filter {
    pub fn new(field: FilterField, operator: FilterOperator, value: FilterValue) -> Self {
        Self {
            field,
            operator,
            value,
        }
    }

    pub fn from_values(values: HashMap<String, String>) -> Result<Self, String> {
        let field = values.get("field").ok_or("Field not found")?;
        let operator = values.get("operator").ok_or("Operator not found")?;
        let value = values.get("value").ok_or("Value not found")?;

        let field = FilterField::try_from(field.to_owned())?;
        let operator = FilterOperator::try_from(operator.to_owned())?;
        let value = FilterValue::try_from(value.to_owned())?;

        Ok(Self {
            field,
            operator,
            value,
        })
    }

    pub fn field(&self) -> &FilterField {
        &self.field
    }

    pub fn operator(&self) -> &FilterOperator {
        &self.operator
    }

    pub fn value(&self) -> &FilterValue {
        &self.value
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.field, self.operator, self.value)
    }
}
