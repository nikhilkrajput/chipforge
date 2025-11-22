//! Recursive descent parser for Verilog

use chipforge_common::{Error, Result};
use chipforge_common::location::Location;
use super::lexer::{Lexer, Token};
use super::ast::*;

/// Verilog parser
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    /// Create a new parser
    pub fn new(mut lexer: Lexer<'a>) -> Result<Self> {
        let current_token = lexer.next_token()?;
        Ok(Parser {
            lexer,
            current_token,
        })
    }

    /// Get current location
    fn location(&self) -> Location {
        self.lexer.location()
    }

    /// Advance to next token
    fn advance(&mut self) -> Result<()> {
        self.current_token = self.lexer.next_token()?;
        Ok(())
    }

    /// Check if current token matches expected token
    fn expect(&mut self, expected: Token) -> Result<()> {
        if self.current_token == expected {
            self.advance()?;
            Ok(())
        } else {
            Err(Error::parse(
                format!("Expected {:?}, found {:?}", expected, self.current_token),
                self.location(),
            ))
        }
    }

    /// Check if current token is identifier and return it
    fn expect_identifier(&mut self) -> Result<String> {
        if let Token::Identifier(name) = &self.current_token {
            let result = name.clone();
            self.advance()?;
            Ok(result)
        } else {
            Err(Error::parse(
                format!("Expected identifier, found {:?}", self.current_token),
                self.location(),
            ))
        }
    }

    /// Parse a complete source file
    pub fn parse_source_file(&mut self) -> Result<SourceFile> {
        let mut items = Vec::new();

        while self.current_token != Token::Eof {
            items.push(self.parse_item()?);
        }

        Ok(SourceFile { items })
    }

    /// Parse a top-level item
    fn parse_item(&mut self) -> Result<Item> {
        match &self.current_token {
            Token::Module => Ok(Item::Module(self.parse_module()?)),
            _ => Err(Error::parse(
                format!("Unexpected token: {:?}", self.current_token),
                self.location(),
            )),
        }
    }

    /// Parse a module
    fn parse_module(&mut self) -> Result<Module> {
        let loc = self.location();
        self.expect(Token::Module)?;

        let name = self.expect_identifier()?;

        // Parse parameter list if present
        let params = if self.current_token == Token::Hash {
            self.advance()?;
            self.parse_parameter_port_list()?
        } else {
            Vec::new()
        };

        // Parse port list
        let ports = if self.current_token == Token::LParen {
            self.parse_port_list()?
        } else {
            Vec::new()
        };

        self.expect(Token::Semicolon)?;

        // Parse module items
        let mut items = Vec::new();
        while self.current_token != Token::EndModule {
            items.push(self.parse_module_item()?);
        }

        self.expect(Token::EndModule)?;

        Ok(Module {
            loc,
            name,
            params,
            ports,
            items,
        })
    }

    /// Parse parameter port list
    fn parse_parameter_port_list(&mut self) -> Result<Vec<Parameter>> {
        self.expect(Token::LParen)?;

        let mut params = Vec::new();

        if self.current_token != Token::RParen {
            loop {
                params.push(self.parse_parameter()?);

                if self.current_token == Token::Comma {
                    self.advance()?;
                } else {
                    break;
                }
            }
        }

        self.expect(Token::RParen)?;
        Ok(params)
    }

    /// Parse a parameter
    fn parse_parameter(&mut self) -> Result<Parameter> {
        let loc = self.location();

        // Optional parameter keyword
        if self.current_token == Token::Parameter {
            self.advance()?;
        }

        // Optional data type
        let data_type = if self.is_data_type() {
            Some(self.parse_data_type()?)
        } else {
            None
        };

        let name = self.expect_identifier()?;

        let default_value = if self.current_token == Token::Equal {
            self.advance()?;
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(Parameter {
            loc,
            name,
            data_type,
            default_value,
        })
    }

    /// Parse port list
    fn parse_port_list(&mut self) -> Result<Vec<Port>> {
        self.expect(Token::LParen)?;

        let mut ports = Vec::new();

        if self.current_token != Token::RParen {
            loop {
                ports.push(self.parse_port()?);

                if self.current_token == Token::Comma {
                    self.advance()?;
                } else {
                    break;
                }
            }
        }

        self.expect(Token::RParen)?;
        Ok(ports)
    }

    /// Parse a port
    fn parse_port(&mut self) -> Result<Port> {
        let loc = self.location();

        let direction = match &self.current_token {
            Token::Input => {
                self.advance()?;
                Some(PortDirection::Input)
            }
            Token::Output => {
                self.advance()?;
                Some(PortDirection::Output)
            }
            Token::Inout => {
                self.advance()?;
                Some(PortDirection::Inout)
            }
            _ => None,
        };

        // Skip optional data type for now
        if self.is_data_type() {
            self.parse_data_type()?;
        }

        let name = self.expect_identifier()?;

        Ok(Port {
            loc,
            direction,
            name,
        })
    }

    /// Check if current token starts a data type
    fn is_data_type(&self) -> bool {
        matches!(
            self.current_token,
            Token::Wire
                | Token::Reg
                | Token::Logic
                | Token::Bit
                | Token::Integer
                | Token::Real
                | Token::Time
                | Token::Byte
                | Token::Shortint
                | Token::Int
                | Token::Longint
        )
    }

    /// Parse data type
    fn parse_data_type(&mut self) -> Result<DataType> {
        let data_type = match &self.current_token {
            Token::Wire => {
                self.advance()?;
                DataType::Wire(self.parse_optional_range()?)
            }
            Token::Reg => {
                self.advance()?;
                DataType::Reg(self.parse_optional_range()?)
            }
            Token::Logic => {
                self.advance()?;
                DataType::Logic(self.parse_optional_range()?)
            }
            Token::Bit => {
                self.advance()?;
                DataType::Bit(self.parse_optional_range()?)
            }
            Token::Integer => {
                self.advance()?;
                DataType::Integer
            }
            Token::Real => {
                self.advance()?;
                DataType::Real
            }
            Token::Time => {
                self.advance()?;
                DataType::Time
            }
            Token::Byte => {
                self.advance()?;
                DataType::Byte
            }
            Token::Shortint => {
                self.advance()?;
                DataType::Shortint
            }
            Token::Int => {
                self.advance()?;
                DataType::Int
            }
            Token::Longint => {
                self.advance()?;
                DataType::Longint
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance()?;
                DataType::UserDefined(name)
            }
            _ => DataType::Implicit,
        };

        Ok(data_type)
    }

    /// Parse optional range [msb:lsb]
    fn parse_optional_range(&mut self) -> Result<Option<Range>> {
        if self.current_token == Token::LBracket {
            Ok(Some(self.parse_range()?))
        } else {
            Ok(None)
        }
    }

    /// Parse range [msb:lsb]
    fn parse_range(&mut self) -> Result<Range> {
        self.expect(Token::LBracket)?;
        let msb = self.parse_expression()?;
        self.expect(Token::Colon)?;
        let lsb = self.parse_expression()?;
        self.expect(Token::RBracket)?;

        Ok(Range { msb, lsb })
    }

    /// Parse module item
    fn parse_module_item(&mut self) -> Result<ModuleItem> {
        match &self.current_token {
            Token::Input | Token::Output | Token::Inout => {
                Ok(ModuleItem::PortDeclaration(self.parse_port_declaration()?))
            }
            Token::Wire => Ok(ModuleItem::NetDeclaration(self.parse_net_declaration()?)),
            Token::Reg | Token::Logic | Token::Integer => {
                Ok(ModuleItem::RegDeclaration(self.parse_reg_declaration()?))
            }
            Token::Parameter | Token::Localparam => {
                Ok(ModuleItem::ParameterDeclaration(self.parse_parameter_declaration()?))
            }
            Token::Assign => Ok(ModuleItem::ContinuousAssign(self.parse_continuous_assign()?)),
            Token::Always => Ok(ModuleItem::AlwaysBlock(self.parse_always_block()?)),
            Token::Initial => Ok(ModuleItem::InitialBlock(self.parse_initial_block()?)),
            Token::Generate => Ok(ModuleItem::GenerateBlock(self.parse_generate_block()?)),
            Token::Function => Ok(ModuleItem::FunctionDeclaration(self.parse_function_declaration()?)),
            Token::Task => Ok(ModuleItem::TaskDeclaration(self.parse_task_declaration()?)),
            Token::Identifier(_) => {
                Ok(ModuleItem::ModuleInstantiation(self.parse_module_instantiation()?))
            }
            _ => Err(Error::parse(
                format!("Unexpected token in module: {:?}", self.current_token),
                self.location(),
            )),
        }
    }

    /// Parse port declaration
    fn parse_port_declaration(&mut self) -> Result<PortDeclaration> {
        let loc = self.location();

        let direction = match &self.current_token {
            Token::Input => {
                self.advance()?;
                PortDirection::Input
            }
            Token::Output => {
                self.advance()?;
                PortDirection::Output
            }
            Token::Inout => {
                self.advance()?;
                PortDirection::Inout
            }
            _ => {
                return Err(Error::parse(
                    "Expected port direction".to_string(),
                    self.location(),
                ));
            }
        };

        let data_type = if self.is_data_type() {
            self.parse_data_type()?
        } else {
            DataType::Implicit
        };

        let mut names = vec![self.expect_identifier()?];

        while self.current_token == Token::Comma {
            self.advance()?;
            names.push(self.expect_identifier()?);
        }

        self.expect(Token::Semicolon)?;

        Ok(PortDeclaration {
            loc,
            direction,
            data_type,
            names,
        })
    }

    /// Parse net declaration
    fn parse_net_declaration(&mut self) -> Result<NetDeclaration> {
        let loc = self.location();
        self.expect(Token::Wire)?;

        let data_type = if self.current_token == Token::LBracket {
            DataType::Wire(Some(self.parse_range()?))
        } else {
            DataType::Wire(None)
        };

        let mut names = vec![self.expect_identifier()?];

        while self.current_token == Token::Comma {
            self.advance()?;
            names.push(self.expect_identifier()?);
        }

        self.expect(Token::Semicolon)?;

        Ok(NetDeclaration {
            loc,
            net_type: NetType::Wire,
            data_type,
            names,
        })
    }

    /// Parse reg declaration
    fn parse_reg_declaration(&mut self) -> Result<RegDeclaration> {
        let loc = self.location();

        let var_type = match &self.current_token {
            Token::Reg => {
                self.advance()?;
                VarType::Reg
            }
            Token::Logic => {
                self.advance()?;
                VarType::Logic
            }
            Token::Integer => {
                self.advance()?;
                VarType::Integer
            }
            _ => {
                return Err(Error::parse(
                    "Expected variable type".to_string(),
                    self.location(),
                ));
            }
        };

        let data_type = if self.current_token == Token::LBracket {
            DataType::Reg(Some(self.parse_range()?))
        } else {
            DataType::Reg(None)
        };

        let mut variables = Vec::new();
        loop {
            let name = self.expect_identifier()?;

            let dimensions = Vec::new(); // TODO: Parse array dimensions

            let initial_value = if self.current_token == Token::Equal {
                self.advance()?;
                Some(self.parse_expression()?)
            } else {
                None
            };

            variables.push(Variable {
                name,
                dimensions,
                initial_value,
            });

            if self.current_token == Token::Comma {
                self.advance()?;
            } else {
                break;
            }
        }

        self.expect(Token::Semicolon)?;

        Ok(RegDeclaration {
            loc,
            var_type,
            data_type,
            variables,
        })
    }

    /// Parse parameter declaration
    fn parse_parameter_declaration(&mut self) -> Result<ParameterDeclaration> {
        let loc = self.location();

        let is_local = if self.current_token == Token::Localparam {
            self.advance()?;
            true
        } else {
            self.expect(Token::Parameter)?;
            false
        };

        let data_type = if self.is_data_type() {
            Some(self.parse_data_type()?)
        } else {
            None
        };

        let mut params = Vec::new();
        loop {
            let name = self.expect_identifier()?;
            self.expect(Token::Equal)?;
            let value = self.parse_expression()?;
            params.push((name, value));

            if self.current_token == Token::Comma {
                self.advance()?;
            } else {
                break;
            }
        }

        self.expect(Token::Semicolon)?;

        Ok(ParameterDeclaration {
            loc,
            is_local,
            data_type,
            params,
        })
    }

    /// Parse continuous assignment
    fn parse_continuous_assign(&mut self) -> Result<ContinuousAssign> {
        let loc = self.location();
        self.expect(Token::Assign)?;

        let mut assignments = Vec::new();
        loop {
            let lhs = self.parse_expression()?;
            self.expect(Token::Equal)?;
            let rhs = self.parse_expression()?;

            assignments.push(Assignment { lhs, rhs });

            if self.current_token == Token::Comma {
                self.advance()?;
            } else {
                break;
            }
        }

        self.expect(Token::Semicolon)?;

        Ok(ContinuousAssign {
            loc,
            delay: None,
            assignments,
        })
    }

    /// Parse always block
    fn parse_always_block(&mut self) -> Result<AlwaysBlock> {
        let loc = self.location();
        self.expect(Token::Always)?;

        // Parse sensitivity list
        let sensitivity = if self.current_token == Token::At {
            self.advance()?;
            if self.current_token == Token::Star {
                self.advance()?;
                SensitivityList::Star
            } else if self.current_token == Token::LParen {
                self.advance()?;

                // Check for @(*)
                if self.current_token == Token::Star {
                    self.advance()?;
                    self.expect(Token::RParen)?;
                    SensitivityList::Star
                } else {
                    // Parse regular sensitivity list
                    let mut items = Vec::new();

                    loop {
                        let edge = match &self.current_token {
                            Token::Posedge => {
                                self.advance()?;
                                Some(Edge::Posedge)
                            }
                            Token::Negedge => {
                                self.advance()?;
                                Some(Edge::Negedge)
                            }
                            _ => None,
                        };

                        let expression = self.parse_expression()?;

                        items.push(SensitivityItem { edge, expression });

                        if self.current_token == Token::Comma || self.current_token == Token::Or {
                            self.advance()?;
                        } else {
                            break;
                        }
                    }

                    self.expect(Token::RParen)?;
                    SensitivityList::List(items)
                }
            } else {
                return Err(Error::parse(
                    "Expected '(' or '*' after '@'".to_string(),
                    self.location(),
                ));
            }
        } else {
            SensitivityList::Star
        };

        let statement = self.parse_statement()?;

        Ok(AlwaysBlock {
            loc,
            sensitivity,
            statement,
        })
    }

    /// Parse initial block
    fn parse_initial_block(&mut self) -> Result<InitialBlock> {
        let loc = self.location();
        self.expect(Token::Initial)?;
        let statement = self.parse_statement()?;

        Ok(InitialBlock { loc, statement })
    }

    /// Parse statement
    fn parse_statement(&mut self) -> Result<Statement> {
        match &self.current_token {
            Token::Begin => self.parse_seq_block(),
            Token::If => self.parse_if_statement(),
            Token::Case | Token::Casex | Token::Casez => self.parse_case_statement(),
            Token::For => self.parse_for_statement(),
            Token::While => self.parse_while_statement(),
            Token::Repeat => self.parse_repeat_statement(),
            Token::Forever => self.parse_forever_statement(),
            Token::Semicolon => {
                let loc = self.location();
                self.advance()?;
                Ok(Statement::Null(loc))
            }
            _ => {
                // Try to parse as assignment
                let loc = self.location();
                let lhs = self.parse_expression()?;

                match &self.current_token {
                    Token::Equal => {
                        self.advance()?;
                        let rhs = self.parse_expression()?;
                        self.expect(Token::Semicolon)?;
                        Ok(Statement::BlockingAssignment { loc, lhs, rhs })
                    }
                    Token::LessEqual => {
                        self.advance()?;
                        let rhs = self.parse_expression()?;
                        self.expect(Token::Semicolon)?;
                        Ok(Statement::NonBlockingAssignment {
                            loc,
                            lhs,
                            delay: None,
                            rhs,
                        })
                    }
                    Token::Semicolon => {
                        // Just an expression statement (function call)
                        self.advance()?;
                        if let Expression::FunctionCall { name, args, .. } = lhs {
                            Ok(Statement::Call {
                                loc,
                                name,
                                arguments: args,
                            })
                        } else {
                            Ok(Statement::Null(loc))
                        }
                    }
                    _ => Err(Error::parse(
                        format!("Unexpected token in statement: {:?}", self.current_token),
                        self.location(),
                    )),
                }
            }
        }
    }

    /// Parse sequential block
    fn parse_seq_block(&mut self) -> Result<Statement> {
        let loc = self.location();
        self.expect(Token::Begin)?;

        let name = if let Token::Colon = self.current_token {
            self.advance()?;
            Some(self.expect_identifier()?)
        } else {
            None
        };

        let mut statements = Vec::new();
        while self.current_token != Token::End {
            statements.push(self.parse_statement()?);
        }

        self.expect(Token::End)?;

        Ok(Statement::SeqBlock {
            loc,
            name,
            declarations: Vec::new(),
            statements,
        })
    }

    /// Parse if statement
    fn parse_if_statement(&mut self) -> Result<Statement> {
        let loc = self.location();
        self.expect(Token::If)?;
        self.expect(Token::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::RParen)?;

        let then_stmt = Box::new(self.parse_statement()?);

        let else_stmt = if self.current_token == Token::Else {
            self.advance()?;
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Statement::If {
            loc,
            condition,
            then_stmt,
            else_stmt,
        })
    }

    /// Parse case statement (simplified)
    fn parse_case_statement(&mut self) -> Result<Statement> {
        let loc = self.location();

        let case_type = match &self.current_token {
            Token::Case => CaseType::Case,
            Token::Casex => CaseType::Casex,
            Token::Casez => CaseType::Casez,
            _ => unreachable!(),
        };
        self.advance()?;

        self.expect(Token::LParen)?;
        let expr = self.parse_expression()?;
        self.expect(Token::RParen)?;

        let mut items = Vec::new();
        let mut default = None;

        while self.current_token != Token::EndCase {
            if self.current_token == Token::Default {
                self.advance()?;
                self.expect(Token::Colon)?;
                default = Some(Box::new(self.parse_statement()?));
            } else {
                let mut expressions = vec![self.parse_expression()?];
                while self.current_token == Token::Comma {
                    self.advance()?;
                    expressions.push(self.parse_expression()?);
                }
                self.expect(Token::Colon)?;
                let statement = self.parse_statement()?;
                items.push(CaseItem {
                    expressions,
                    statement,
                });
            }
        }

        self.expect(Token::EndCase)?;

        Ok(Statement::Case {
            loc,
            case_type,
            expr,
            items,
            default,
        })
    }

    /// Parse for statement (simplified)
    fn parse_for_statement(&mut self) -> Result<Statement> {
        let loc = self.location();
        self.expect(Token::For)?;
        self.expect(Token::LParen)?;

        let init = Box::new(self.parse_statement()?);
        let condition = self.parse_expression()?;
        self.expect(Token::Semicolon)?;

        // Parse update (simplified - just parse expression and semicolon)
        let _update_expr = self.parse_expression()?;
        let update = Box::new(Statement::Null(loc.clone())); // Simplified

        self.expect(Token::RParen)?;
        let body = Box::new(self.parse_statement()?);

        Ok(Statement::For {
            loc,
            init,
            condition,
            update,
            body,
        })
    }

    /// Parse while statement
    fn parse_while_statement(&mut self) -> Result<Statement> {
        let loc = self.location();
        self.expect(Token::While)?;
        self.expect(Token::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::RParen)?;
        let body = Box::new(self.parse_statement()?);

        Ok(Statement::While {
            loc,
            condition,
            body,
        })
    }

    /// Parse repeat statement
    fn parse_repeat_statement(&mut self) -> Result<Statement> {
        let loc = self.location();
        self.expect(Token::Repeat)?;
        self.expect(Token::LParen)?;
        let count = self.parse_expression()?;
        self.expect(Token::RParen)?;
        let body = Box::new(self.parse_statement()?);

        Ok(Statement::Repeat {
            loc,
            count,
            body,
        })
    }

    /// Parse forever statement
    fn parse_forever_statement(&mut self) -> Result<Statement> {
        let loc = self.location();
        self.expect(Token::Forever)?;
        let body = Box::new(self.parse_statement()?);

        Ok(Statement::Forever { loc, body })
    }

    /// Parse module instantiation (simplified)
    fn parse_module_instantiation(&mut self) -> Result<ModuleInstantiation> {
        let loc = self.location();
        let module_name = self.expect_identifier()?;

        // TODO: Parse parameter assignments

        let mut instances = Vec::new();
        loop {
            let name = self.expect_identifier()?;
            self.expect(Token::LParen)?;

            let mut port_connections = Vec::new();
            if self.current_token != Token::RParen {
                loop {
                    // Simplified: only positional connections for now
                    let expr = self.parse_expression()?;
                    port_connections.push(PortConnection::Positional(expr));

                    if self.current_token == Token::Comma {
                        self.advance()?;
                    } else {
                        break;
                    }
                }
            }

            self.expect(Token::RParen)?;

            instances.push(Instance {
                name,
                port_connections,
            });

            if self.current_token == Token::Comma {
                self.advance()?;
            } else {
                break;
            }
        }

        self.expect(Token::Semicolon)?;

        Ok(ModuleInstantiation {
            loc,
            module_name,
            param_assignments: Vec::new(),
            instances,
        })
    }

    /// Parse generate block (stub)
    fn parse_generate_block(&mut self) -> Result<GenerateBlock> {
        let loc = self.location();
        self.expect(Token::Generate)?;

        let items = Vec::new();
        while self.current_token != Token::EndGenerate {
            // Simplified: skip for now
            self.advance()?;
        }

        self.expect(Token::EndGenerate)?;

        Ok(GenerateBlock { loc, items })
    }

    /// Parse function declaration (stub)
    fn parse_function_declaration(&mut self) -> Result<FunctionDeclaration> {
        let loc = self.location();
        self.expect(Token::Function)?;

        let return_type = DataType::Implicit;
        let name = self.expect_identifier()?;

        // Skip to endfunction
        while self.current_token != Token::EndFunction {
            self.advance()?;
        }
        self.expect(Token::EndFunction)?;

        Ok(FunctionDeclaration {
            loc,
            return_type,
            name,
            ports: Vec::new(),
            items: Vec::new(),
            statement: None,
        })
    }

    /// Parse task declaration (stub)
    fn parse_task_declaration(&mut self) -> Result<TaskDeclaration> {
        let loc = self.location();
        self.expect(Token::Task)?;

        let name = self.expect_identifier()?;

        // Skip to endtask
        while self.current_token != Token::EndTask {
            self.advance()?;
        }
        self.expect(Token::EndTask)?;

        Ok(TaskDeclaration {
            loc,
            name,
            ports: Vec::new(),
            items: Vec::new(),
            statement: None,
        })
    }

    /// Parse expression with precedence climbing
    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_ternary()
    }

    /// Parse ternary conditional
    fn parse_ternary(&mut self) -> Result<Expression> {
        let expr = self.parse_logical_or()?;

        if self.current_token == Token::Question {
            let loc = self.location();
            self.advance()?;
            let true_expr = Box::new(self.parse_expression()?);
            self.expect(Token::Colon)?;
            let false_expr = Box::new(self.parse_expression()?);

            Ok(Expression::Ternary {
                loc,
                cond: Box::new(expr),
                true_expr,
                false_expr,
            })
        } else {
            Ok(expr)
        }
    }

    /// Parse logical OR
    fn parse_logical_or(&mut self) -> Result<Expression> {
        let mut left = self.parse_logical_and()?;

        while self.current_token == Token::LogicalOr {
            let loc = self.location();
            self.advance()?;
            let right = self.parse_logical_and()?;
            left = Expression::Binary {
                loc,
                op: BinaryOp::LogicalOr,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse logical AND
    fn parse_logical_and(&mut self) -> Result<Expression> {
        let mut left = self.parse_bitwise_or()?;

        while self.current_token == Token::LogicalAnd {
            let loc = self.location();
            self.advance()?;
            let right = self.parse_bitwise_or()?;
            left = Expression::Binary {
                loc,
                op: BinaryOp::LogicalAnd,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse bitwise OR
    fn parse_bitwise_or(&mut self) -> Result<Expression> {
        let mut left = self.parse_bitwise_xor()?;

        while self.current_token == Token::BitwiseOr {
            let loc = self.location();
            self.advance()?;
            let right = self.parse_bitwise_xor()?;
            left = Expression::Binary {
                loc,
                op: BinaryOp::BitwiseOr,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse bitwise XOR
    fn parse_bitwise_xor(&mut self) -> Result<Expression> {
        let mut left = self.parse_bitwise_and()?;

        while self.current_token == Token::BitwiseXor {
            let loc = self.location();
            self.advance()?;
            let right = self.parse_bitwise_and()?;
            left = Expression::Binary {
                loc,
                op: BinaryOp::BitwiseXor,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse bitwise AND
    fn parse_bitwise_and(&mut self) -> Result<Expression> {
        let mut left = self.parse_equality()?;

        while self.current_token == Token::BitwiseAnd {
            let loc = self.location();
            self.advance()?;
            let right = self.parse_equality()?;
            left = Expression::Binary {
                loc,
                op: BinaryOp::BitwiseAnd,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse equality operators
    fn parse_equality(&mut self) -> Result<Expression> {
        let mut left = self.parse_relational()?;

        loop {
            let op = match &self.current_token {
                Token::EqualEqual => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                Token::EqualEqualEqual => BinaryOp::CaseEqual,
                Token::NotEqualEqual => BinaryOp::CaseNotEqual,
                _ => break,
            };

            let loc = self.location();
            self.advance()?;
            let right = self.parse_relational()?;
            left = Expression::Binary {
                loc,
                op,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse relational operators
    fn parse_relational(&mut self) -> Result<Expression> {
        let mut left = self.parse_shift()?;

        loop {
            let op = match &self.current_token {
                Token::Less => BinaryOp::Less,
                Token::LessEqual => BinaryOp::LessEqual,
                Token::Greater => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                _ => break,
            };

            let loc = self.location();
            self.advance()?;
            let right = self.parse_shift()?;
            left = Expression::Binary {
                loc,
                op,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse shift operators
    fn parse_shift(&mut self) -> Result<Expression> {
        let mut left = self.parse_additive()?;

        loop {
            let op = match &self.current_token {
                Token::ShiftLeft => BinaryOp::ShiftLeft,
                Token::ShiftRight => BinaryOp::ShiftRight,
                Token::ArithShiftLeft => BinaryOp::ArithShiftLeft,
                Token::ArithShiftRight => BinaryOp::ArithShiftRight,
                _ => break,
            };

            let loc = self.location();
            self.advance()?;
            let right = self.parse_additive()?;
            left = Expression::Binary {
                loc,
                op,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse additive operators
    fn parse_additive(&mut self) -> Result<Expression> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let op = match &self.current_token {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => break,
            };

            let loc = self.location();
            self.advance()?;
            let right = self.parse_multiplicative()?;
            left = Expression::Binary {
                loc,
                op,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse multiplicative operators
    fn parse_multiplicative(&mut self) -> Result<Expression> {
        let mut left = self.parse_unary()?;

        loop {
            let op = match &self.current_token {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::Percent => BinaryOp::Mod,
                _ => break,
            };

            let loc = self.location();
            self.advance()?;
            let right = self.parse_unary()?;
            left = Expression::Binary {
                loc,
                op,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse unary operators
    fn parse_unary(&mut self) -> Result<Expression> {
        let op = match &self.current_token {
            Token::Plus => Some(UnaryOp::Plus),
            Token::Minus => Some(UnaryOp::Minus),
            Token::LogicalNot => Some(UnaryOp::LogicalNot),
            Token::BitwiseNot => Some(UnaryOp::BitwiseNot),
            Token::BitwiseAnd => Some(UnaryOp::BitwiseAnd),
            Token::BitwiseNand => Some(UnaryOp::BitwiseNand),
            Token::BitwiseOr => Some(UnaryOp::BitwiseOr),
            Token::BitwiseNor => Some(UnaryOp::BitwiseNor),
            Token::BitwiseXor => Some(UnaryOp::BitwiseXor),
            Token::BitwiseXnor => Some(UnaryOp::BitwiseXnor),
            _ => None,
        };

        if let Some(op) = op {
            let loc = self.location();
            self.advance()?;
            let expr = Box::new(self.parse_unary()?);
            Ok(Expression::Unary { loc, op, expr })
        } else {
            self.parse_postfix()
        }
    }

    /// Parse postfix operators (bit select, part select, function call)
    fn parse_postfix(&mut self) -> Result<Expression> {
        let mut expr = self.parse_primary()?;

        loop {
            match &self.current_token {
                Token::LBracket => {
                    let loc = self.location();
                    self.advance()?;
                    let index = Box::new(self.parse_expression()?);

                    if self.current_token == Token::Colon {
                        self.advance()?;
                        let lsb = index;
                        let msb = Box::new(self.parse_expression()?);
                        self.expect(Token::RBracket)?;
                        expr = Expression::PartSelect {
                            loc,
                            expr: Box::new(expr),
                            msb,
                            lsb,
                        };
                    } else {
                        self.expect(Token::RBracket)?;
                        expr = Expression::BitSelect {
                            loc,
                            expr: Box::new(expr),
                            index,
                        };
                    }
                }
                Token::LParen => {
                    if let Expression::Identifier(loc, name) = expr {
                        self.advance()?;
                        let mut args = Vec::new();

                        if self.current_token != Token::RParen {
                            loop {
                                args.push(self.parse_expression()?);
                                if self.current_token == Token::Comma {
                                    self.advance()?;
                                } else {
                                    break;
                                }
                            }
                        }

                        self.expect(Token::RParen)?;
                        expr = Expression::FunctionCall {
                            loc: loc.clone(),
                            name,
                            args,
                        };
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    /// Parse primary expression
    fn parse_primary(&mut self) -> Result<Expression> {
        match &self.current_token.clone() {
            Token::Number(num) => {
                let loc = self.location();
                let num = num.clone();
                self.advance()?;
                Ok(Expression::Number(loc, num))
            }
            Token::StringLiteral(s) => {
                let loc = self.location();
                let s = s.clone();
                self.advance()?;
                Ok(Expression::String(loc, s))
            }
            Token::Identifier(name) => {
                let loc = self.location();
                let name = name.clone();
                self.advance()?;
                Ok(Expression::Identifier(loc, name))
            }
            Token::LParen => {
                self.advance()?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            Token::LBrace => {
                let loc = self.location();
                self.advance()?;

                let mut exprs = Vec::new();

                if self.current_token != Token::RBrace {
                    loop {
                        exprs.push(self.parse_expression()?);
                        if self.current_token == Token::Comma {
                            self.advance()?;
                        } else {
                            break;
                        }
                    }
                }

                self.expect(Token::RBrace)?;

                Ok(Expression::Concat { loc, exprs })
            }
            _ => Err(Error::parse(
                format!("Unexpected token in expression: {:?}", self.current_token),
                self.location(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_module() {
        let input = r#"
module test(input a, output b);
    wire c;
    assign b = a;
endmodule
"#;

        let lexer = Lexer::new(input, "test.v");
        let mut parser = Parser::new(lexer).unwrap();
        let source = parser.parse_source_file().unwrap();

        assert_eq!(source.items.len(), 1);
    }

    #[test]
    fn test_parse_expression() {
        let input = "module m; assign x = a + b * c; endmodule";
        let lexer = Lexer::new(input, "test.v");
        let mut parser = Parser::new(lexer).unwrap();
        let source = parser.parse_source_file().unwrap();

        assert_eq!(source.items.len(), 1);
    }
}
