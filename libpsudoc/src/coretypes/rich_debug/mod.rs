mod control_flow_expression;
mod r#else;
mod expression;
mod r#if;
mod literal;
mod member_expression;
mod node;
mod operator_expression;
mod statement;
mod declaration;

use super::CompileSession;

pub trait RichDebug {
    fn rich_debug(&self, session: &CompileSession) -> String;
}

pub use control_flow_expression::*;
pub use expression::*;
pub use literal::*;
pub use member_expression::*;pub use node::*;
pub use operator_expression::*;
pub use r#else::*;
pub use r#if::*;
pub use statement::*;
pub use declaration::*;