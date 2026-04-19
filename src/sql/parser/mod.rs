// 从 ast 模块引入 Column 结构体
use ast::Column;
// 从 lexer 模块引入 Keyword, Token
use lexer::{Keyword, Token};

// 从上级目录的 types 模块引入 DataType
use super::types::DataType;
// 引入错误类型和结果类型
use crate::error::{Error, Result};
// 引入 ast 模块中的 Consts 枚举
use crate::sql::parser::ast::Expression::Consts;

// 公开 ast 模块
pub mod ast;
// 不公开 lexer 模块（内部实现）
mod lexer;
// 引入 common 模块（包含 Parser 结构体和 ParserExt trait）
mod common;

// 从 common 模块公开 Parser 和 ParserExt
pub use common::{Parser, ParserExt};

// 注意：Parser 结构体及其构造函数 new() 在 common.rs 中定义
// 这里只实现业务相关的解析方法

// 为 Parser 实现业务解析方法
impl<'a> Parser<'a> {
    // 公开方法：解析 SQL 字符串，返回抽象语法树（Statement）
    // 入口函数，调用此方法完成整个解析流程
    pub fn parse(&mut self) -> Result<ast::Statement> {
        let stmt = self.parse_statement()?; // 调用 parse_statement 解析语句
        self.next_expect(Token::Semicolon)?; // 期望语句以分号结尾
                                             // 检查分号后是否还有 token，若有则报错（不允许多余内容）
        if let Some(token) = self.peek()? {
            return Err(Error::Parse(format!(
                "[Parser] Unexpected token {:?}",
                token
            )));
        }
        Ok(stmt)
    }

    // 私有方法：解析语句，根据首个 token 类型决定解析路径
    fn parse_statement(&mut self) -> Result<ast::Statement> {
        // peek 查看第一个 token 但不消费
        match self.peek()? {
            Some(Token::Keyword(Keyword::Create)) => self.parse_ddl(), // CREATE 语句
            Some(Token::Keyword(Keyword::Select)) => self.parse_select(), // SELECT 语句
            Some(Token::Keyword(Keyword::Insert)) => self.parse_insert(), // INSERT 语句
            Some(t) => Err(Error::Parse(format!("[Parser] Unexpected token {:?}", t))), // 未知 token
            None => Err(Error::Parse("[Parser] Unexpected end of input".to_string())),  // 输入为空
        }
    }

    // 私有方法：解析 DDL（数据定义语言），目前只支持 CREATE TABLE
    fn parse_ddl(&mut self) -> Result<ast::Statement> {
        // 消费第一个 token（应该是 CREATE）
        match self.next()? {
            Token::Keyword(Keyword::Create) => match self.next()? {
                // 消费第二个 token（应该是 TABLE）
                Token::Keyword(Keyword::Table) => self.parse_ddl_create_table(),
                token => Err(Error::Parse(format!(
                    "[Parser] Unexpected token {:?}",
                    token
                ))),
            },
            token => Err(Error::Parse(format!(
                "[Parser] Unexpected token {:?}",
                token
            ))),
        }
    }

    // 私有方法：解析 SELECT 语句
    // 语法：SELECT * FROM table_name
    fn parse_select(&mut self) -> Result<ast::Statement> {
        self.next_expect(Token::Keyword(Keyword::Select))?; // 期望 SELECT 关键字
        self.next_expect(Token::Asterisk)?; // 期望 *（所有列）
        self.next_expect(Token::Keyword(Keyword::From))?; // 期望 FROM 关键字

        let table_name = self.next_ident()?; // 解析表名（标识符）
        Ok(ast::Statement::Select { table_name }) // 返回 Select 语句
    }

    // 私有方法：解析 INSERT 语句
    // 语法：INSERT INTO table_name (col1, col2) VALUES (val1, val2), (val3, val4)
    fn parse_insert(&mut self) -> Result<ast::Statement> {
        self.next_expect(Token::Keyword(Keyword::Insert))?; // 期望 INSERT
        self.next_expect(Token::Keyword(Keyword::Into))?; // 期望 INTO

        let table_name = self.next_ident()?; // 解析表名

        // 解析可选的列名列表：INSERT INTO tbl (col1, col2)
        let columns = if self.next_if_token(Token::OpenParen).is_some() {
            let mut cols = Vec::new();
            loop {
                cols.push(self.next_ident()?); // 逐个解析列名
                match self.next()? {
                    Token::CloseParen => break, // 遇到 ) 结束列名列表
                    Token::Comma => continue,   // 遇到 , 继续解析下一个列名
                    token => {
                        return Err(Error::Parse(format!(
                            "[Parser] Unexpected token {:?}",
                            token
                        )))
                    }
                }
            }
            Some(cols)
        } else {
            None // 没有列名列表
        };

        self.next_expect(Token::Keyword(Keyword::Values))?; // 期望 VALUES 关键字

        // 解析多组值：VALUES (1, 2), (3, 4)
        let mut values = Vec::new();
        loop {
            self.next_expect(Token::OpenParen)?; // 期望 (
            let mut exprs = Vec::new();
            loop {
                exprs.push(self.parse_expression()?); // 解析每个表达式
                match self.next()? {
                    Token::CloseParen => break, // 遇到 ) 结束一组值
                    Token::Comma => continue,   // 遇到 , 继续解析下一个表达式
                    token => {
                        return Err(Error::Parse(format!(
                            "[Parser] Unexpected token {:?}",
                            token
                        )))
                    }
                }
            }
            values.push(exprs); // 保存一组值
            if self.next_if_token(Token::Comma).is_none() {
                break; // 没有更多组，跳出循环
            }
        }

        Ok(ast::Statement::Insert {
            table_name,
            columns,
            values,
        })
    }

    // 私有方法：解析 CREATE TABLE 语句
    // 语法：CREATE TABLE table_name (col1 int, col2 text)
    fn parse_ddl_create_table(&mut self) -> Result<ast::Statement> {
        let table_name = self.next_ident()?; // 解析表名
        self.next_expect(Token::OpenParen)?; // 期望 (

        // 循环解析所有列定义
        let mut columns = Vec::new();
        loop {
            columns.push(self.parse_ddl_column()?); // 解析列定义
            if self.next_if_token(Token::Comma).is_none() {
                break; // 没有逗号说明列定义完毕
            }
        }

        self.next_expect(Token::CloseParen)?; // 期望 )
        Ok(ast::Statement::CreateTable {
            name: table_name,
            columns,
        })
    }

    // 私有方法：解析列定义
    // 语法：column_name data_type [NULL | NOT NULL] [DEFAULT expr]
    fn parse_ddl_column(&mut self) -> Result<ast::Column> {
        let mut column = Column {
            name: self.next_ident()?, // 解析列名
            // 解析数据类型
            datatype: match self.next()? {
                Token::Keyword(Keyword::Int) | Token::Keyword(Keyword::Integer) => {
                    DataType::Integer
                }
                Token::Keyword(Keyword::Bool) | Token::Keyword(Keyword::Boolean) => {
                    DataType::Boolean
                }
                Token::Keyword(Keyword::Float) | Token::Keyword(Keyword::Double) => DataType::Float,
                Token::Keyword(Keyword::String)
                | Token::Keyword(Keyword::Text)
                | Token::Keyword(Keyword::Varchar) => DataType::String,
                token => {
                    return Err(Error::Parse(format!(
                        "[Parser] Unexpected token {:?}",
                        token
                    )))
                }
            },
            nullable: None, // 默认为空，后续解析
            default: None,  // 默认为空，后续解析
        };

        // 解析可选的 NULL/NOT NULL 和 DEFAULT 约束
        while let Some(Token::Keyword(keyword)) = self.next_if_keyword() {
            match keyword {
                Keyword::Null => column.nullable = Some(true), // 允许 NULL
                Keyword::Not => {
                    self.next_expect(Token::Keyword(Keyword::Null))?; // 期望 NOT NULL
                    column.nullable = Some(false); // 不允许 NULL
                }
                Keyword::Default => column.default = Some(self.parse_expression()?), // 默认值
                k => return Err(Error::Parse(format!("[Parser] Unexpected keyword {:?}", k))),
            }
        }

        Ok(column)
    }

    // 私有方法：解析表达式（目前只支持字面量）
    // 支持：数字、字符串、布尔值、NULL
    fn parse_expression(&mut self) -> Result<ast::Expression> {
        Ok(match self.next()? {
            Token::Number(n) => {
                // 判断是否为整数（纯数字）或浮点数（包含小数点）
                if n.chars().all(|c| c.is_ascii_digit()) {
                    // 解析为整数
                    ast::Consts::Integer(
                        n.parse()
                            .map_err(|e| Error::Parse(format!("Failed to parse integer: {}", e)))?,
                    )
                    .into()
                } else {
                    // 解析为浮点数
                    ast::Consts::Float(
                        n.parse()
                            .map_err(|e| Error::Parse(format!("Failed to parse float: {}", e)))?,
                    )
                    .into()
                }
            }
            Token::String(s) => ast::Consts::String(s).into(), // 字符串字面量
            Token::Keyword(Keyword::True) => ast::Consts::Boolean(true).into(), // 布尔 true
            Token::Keyword(Keyword::False) => ast::Consts::Boolean(false).into(), // 布尔 false
            Token::Keyword(Keyword::Null) => ast::Consts::Null.into(), // NULL 值
            t => {
                return Err(Error::Parse(format!(
                    "[Parser] Unexpected expression token {:?}",
                    t
                )))
            }
        })
    }
}
