use super::{Node, Plan};
use crate::sql::{
    parser::ast,
    schema::{self, Table},
    types::Value,
};

macro_rules! planner_log {
    ($($arg:tt)*) => {
        #[cfg(test)]
        println!($($arg)*);
    };
}

pub struct Planner;

impl Planner {
    pub fn new() -> Self {
        planner_log!("[planner::new] create planner");
        Self {}
    }

    pub fn build(&mut self, stmt: ast::Statement) -> Plan {
        planner_log!("[planner::build] start");
        planner_log!("[planner::build] input statement:\n{:#?}", stmt);

        let node = self.build_statement(stmt);
        planner_log!("[planner::build] output node:\n{:#?}", node);

        let plan = Plan(node);
        planner_log!("[planner::build] output plan:\n{:#?}", plan);
        planner_log!("[planner::build] finish");

        plan
    }
    fn build_statement(&mut self, stmt: ast::Statement) -> Node {
        planner_log!("[planner::build_statement] dispatch statement");
        match stmt {
            ast::Statement::CreateTable { name, columns } => {
                planner_log!("[planner::build_statement] branch: CreateTable");
                planner_log!("[planner::create_table] table name: {}", name);
                planner_log!(
                    "[planner::create_table] ast column count: {}",
                    columns.len()
                );
                planner_log!("[planner::create_table] ast columns:\n{:#?}", columns);

                let columns = columns
                    .into_iter()
                    .enumerate()
                    .map(|(index, c)| {
                        planner_log!(
                            "[planner::create_table::column:{}] input ast column:\n{:#?}",
                            index,
                            c
                        );

                        let ast::Column {
                            name,
                            data_type,
                            nullable,
                            default,
                        } = c;

                        let nullable = nullable.unwrap_or(true);
                        planner_log!(
                            "[planner::create_table::column:{}] resolved nullable: {}",
                            index,
                            nullable
                        );

                        let default = match default {
                            Some(expr) => {
                                planner_log!(
                                    "[planner::create_table::column:{}] default expression:\n{:#?}",
                                    index,
                                    expr
                                );
                                let value = Value::from_expression(expr);
                                planner_log!(
                                    "[planner::create_table::column:{}] converted default value:\n{:#?}",
                                    index,
                                    value
                                );
                                Some(value)
                            }
                            None if nullable => {
                                planner_log!(
                                    "[planner::create_table::column:{}] no explicit default, nullable column uses NULL",
                                    index
                                );
                                Some(Value::Null)
                            }
                            None => {
                                planner_log!(
                                    "[planner::create_table::column:{}] no explicit default, NOT NULL column has no default",
                                    index
                                );
                                None
                            }
                        };

                        let column = schema::Column {
                            name,
                            data_type,
                            nullable,
                            default,
                        };
                        planner_log!(
                            "[planner::create_table::column:{}] output schema column:\n{:#?}",
                            index,
                            column
                        );

                        column
                    })
                    .collect();

                let schema = Table { name, columns };
                planner_log!(
                    "[planner::create_table] output table schema:\n{:#?}",
                    schema
                );

                Node::CreateTable { schema }
            }
            ast::Statement::Insert {
                table_name,
                columns,
                values,
            } => {
                planner_log!("[planner::build_statement] branch: Insert");
                planner_log!("[planner::insert] table name: {}", table_name);
                planner_log!("[planner::insert] input columns: {:#?}", columns);
                planner_log!("[planner::insert] row count: {}", values.len());
                planner_log!("[planner::insert] input values:\n{:#?}", values);

                let columns = match columns {
                    Some(columns) => {
                        planner_log!("[planner::insert] explicit column count: {}", columns.len());
                        columns
                    }
                    None => {
                        planner_log!(
                            "[planner::insert] no explicit column list, using empty column list"
                        );
                        Vec::new()
                    }
                };

                let node = Node::Insert {
                    table_name,
                    columns,
                    values,
                };
                planner_log!("[planner::insert] output node:\n{:#?}", node);

                node
            }
            ast::Statement::Select { table_name } => {
                planner_log!("[planner::build_statement] branch: Select");
                planner_log!("[planner::select] table name: {}", table_name);

                let node = Node::Scan { table_name };
                planner_log!("[planner::select] output node:\n{:#?}", node);

                node
            }
        }
    }
}
