mod planner;

use super::{
    parser::ast::{self, Expression},
    schema::Table
};


#[derive(Debug, PartialEq)]
pub enum Node {
    CreateTable {
        schema: Table
    },

}
