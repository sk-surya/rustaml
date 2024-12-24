use std::hash::{Hash, Hasher};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    var_type: VarType,
    bounds: Bounds,
    index: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VarType {
    Continuous,
    Integer,
    Binary,
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub lower: Option<f64>,
    pub upper: Option<f64>,
}

impl Variable {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            var_type: VarType::Continuous,
            bounds: Bounds::default(),
            index: None,
        }
    }

    pub fn continuous(mut self) -> Self {
        self.var_type = VarType::Continuous;
        self
    }

    pub fn integer(mut self) -> Self {
        self.var_type = VarType::Integer;
        self
    }

    pub fn binary(mut self) -> Self {
        self.var_type = VarType::Binary;
        self.bounds = Bounds::new(Some(0.0), Some(1.0));
        self
    }

    pub fn bounds(mut self, lower: Option<f64>, upper: Option<f64>) -> Self {
        self.bounds = Bounds::new(lower, upper);
        self
    }
}

impl Hash for Variable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.var_type.hash(state);
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.var_type == other.var_type
    }
}

impl Eq for Variable {}

impl Bounds {
    pub fn new(lower: Option<f64>, upper: Option<f64>) -> Self {
        Self { lower, upper }
    }
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            lower: None,
            upper: None,
        }
    }
}