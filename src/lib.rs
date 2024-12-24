pub mod expression;
pub mod model;
pub mod solvers;
pub mod utils;

pub use expression::{Expression, Term, Variable};
pub use model::Model;
pub use solvers::Solver;

// Create a prelude module for convenient imports
pub mod prelude {
    pub use crate::expression::{Expression, Term, Variable};
    pub use crate::model::Model;
    pub use crate::solvers::Solver;
    pub use crate::utils::Error;
}