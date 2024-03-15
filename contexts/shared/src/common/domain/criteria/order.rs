use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderField {
    value: String,
}

impl TryFrom<String> for OrderField {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self { value })
    }
}

impl Display for OrderField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderType {
    Asc,
    Desc,
    None,
}

impl TryFrom<String> for OrderType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "ASC" => Ok(Self::Asc),
            "DESC" => Ok(Self::Desc),
            "NONE" => Ok(Self::None),
            _ => Err("Invalid OrderType".to_string()),
        }
    }
}

impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asc => write!(f, "ASC"),
            Self::Desc => write!(f, "DESC"),
            Self::None => write!(f, "NONE"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Order {
    order_by: OrderField,
    order_type: OrderType,
}

impl Order {
    pub fn new(order_by: OrderField, order_type: OrderType) -> Self {
        Self {
            order_by,
            order_type,
        }
    }

    pub fn from_values(values: (String, String)) -> Result<Self, String> {
        let order_by = OrderField::try_from(values.0)?;
        let order_type = OrderType::try_from(values.1)?;

        Ok(Self {
            order_by,
            order_type,
        })
    }

    pub fn order_by(&self) -> &OrderField {
        &self.order_by
    }

    pub fn order_type(&self) -> &OrderType {
        &self.order_type
    }

    pub fn is_none(&self) -> bool {
        self.order_type == OrderType::None
    }

    pub fn is_asc(&self) -> bool {
        self.order_type == OrderType::Asc
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.order_by, self.order_type)
    }
}
