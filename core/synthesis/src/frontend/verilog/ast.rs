//! Abstract Syntax Tree for Verilog

use chipforge_common::location::Location;
use super::lexer::NumberLiteral;

/// A complete Verilog source file
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub items: Vec<Item>,
}

/// Top-level items in a Verilog file
#[derive(Debug, Clone)]
pub enum Item {
    Module(Module),
    // TODO: Add more item types (package, interface, etc.)
}

/// Module declaration
#[derive(Debug, Clone)]
pub struct Module {
    pub loc: Location,
    pub name: String,
    pub params: Vec<Parameter>,
    pub ports: Vec<Port>,
    pub items: Vec<ModuleItem>,
}

/// Module items (inside a module)
#[derive(Debug, Clone)]
pub enum ModuleItem {
    PortDeclaration(PortDeclaration),
    NetDeclaration(NetDeclaration),
    RegDeclaration(RegDeclaration),
    ParameterDeclaration(ParameterDeclaration),
    ContinuousAssign(ContinuousAssign),
    AlwaysBlock(AlwaysBlock),
    InitialBlock(InitialBlock),
    ModuleInstantiation(ModuleInstantiation),
    GenerateBlock(GenerateBlock),
    FunctionDeclaration(FunctionDeclaration),
    TaskDeclaration(TaskDeclaration),
}

/// Parameter declaration
#[derive(Debug, Clone)]
pub struct Parameter {
    pub loc: Location,
    pub name: String,
    pub data_type: Option<DataType>,
    pub default_value: Option<Expression>,
}

/// Port declaration
#[derive(Debug, Clone)]
pub struct Port {
    pub loc: Location,
    pub direction: Option<PortDirection>,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PortDirection {
    Input,
    Output,
    Inout,
}

/// Full port declaration inside module
#[derive(Debug, Clone)]
pub struct PortDeclaration {
    pub loc: Location,
    pub direction: PortDirection,
    pub data_type: DataType,
    pub names: Vec<String>,
}

/// Net declaration (wire, tri, etc.)
#[derive(Debug, Clone)]
pub struct NetDeclaration {
    pub loc: Location,
    pub net_type: NetType,
    pub data_type: DataType,
    pub names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NetType {
    Wire,
    Tri,
    Supply0,
    Supply1,
    Wand,
    Triand,
    Wor,
    Trior,
}

/// Register/variable declaration
#[derive(Debug, Clone)]
pub struct RegDeclaration {
    pub loc: Location,
    pub var_type: VarType,
    pub data_type: DataType,
    pub variables: Vec<Variable>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Reg,
    Logic,
    Integer,
    Real,
    Time,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub dimensions: Vec<Range>,
    pub initial_value: Option<Expression>,
}

/// Parameter declaration
#[derive(Debug, Clone)]
pub struct ParameterDeclaration {
    pub loc: Location,
    pub is_local: bool,
    pub data_type: Option<DataType>,
    pub params: Vec<(String, Expression)>,
}

/// Data type
#[derive(Debug, Clone)]
pub enum DataType {
    /// Implicit type (no explicit type specified)
    Implicit,
    /// Logic type (SystemVerilog)
    Logic(Option<Range>),
    /// Bit type (SystemVerilog)
    Bit(Option<Range>),
    /// Reg type (Verilog)
    Reg(Option<Range>),
    /// Wire type (Verilog)
    Wire(Option<Range>),
    /// Integer
    Integer,
    /// Real
    Real,
    /// Time
    Time,
    /// Byte (SystemVerilog)
    Byte,
    /// Shortint (SystemVerilog)
    Shortint,
    /// Int (SystemVerilog)
    Int,
    /// Longint (SystemVerilog)
    Longint,
    /// User-defined type
    UserDefined(String),
}

/// Range specification [msb:lsb]
#[derive(Debug, Clone)]
pub struct Range {
    pub msb: Expression,
    pub lsb: Expression,
}

/// Continuous assignment (assign statement)
#[derive(Debug, Clone)]
pub struct ContinuousAssign {
    pub loc: Location,
    pub delay: Option<Delay>,
    pub assignments: Vec<Assignment>,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub lhs: Expression,
    pub rhs: Expression,
}

/// Delay specification
#[derive(Debug, Clone)]
pub enum Delay {
    Value(Expression),
    MinTypMax(Expression, Expression, Expression),
}

/// Always block
#[derive(Debug, Clone)]
pub struct AlwaysBlock {
    pub loc: Location,
    pub sensitivity: SensitivityList,
    pub statement: Statement,
}

#[derive(Debug, Clone)]
pub enum SensitivityList {
    Star,  // @*
    List(Vec<SensitivityItem>),
}

#[derive(Debug, Clone)]
pub struct SensitivityItem {
    pub edge: Option<Edge>,
    pub expression: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Edge {
    Posedge,
    Negedge,
}

/// Initial block
#[derive(Debug, Clone)]
pub struct InitialBlock {
    pub loc: Location,
    pub statement: Statement,
}

/// Module instantiation
#[derive(Debug, Clone)]
pub struct ModuleInstantiation {
    pub loc: Location,
    pub module_name: String,
    pub param_assignments: Vec<ParamAssignment>,
    pub instances: Vec<Instance>,
}

#[derive(Debug, Clone)]
pub struct ParamAssignment {
    pub name: Option<String>,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub name: String,
    pub port_connections: Vec<PortConnection>,
}

#[derive(Debug, Clone)]
pub enum PortConnection {
    Named { name: String, connection: Option<Expression> },
    Positional(Expression),
}

/// Generate block
#[derive(Debug, Clone)]
pub struct GenerateBlock {
    pub loc: Location,
    pub items: Vec<GenerateItem>,
}

#[derive(Debug, Clone)]
pub enum GenerateItem {
    ModuleItem(ModuleItem),
    ForLoop(GenerateFor),
    IfElse(GenerateIf),
}

#[derive(Debug, Clone)]
pub struct GenerateFor {
    pub init: Box<Statement>,
    pub condition: Expression,
    pub update: Box<Statement>,
    pub body: Vec<GenerateItem>,
}

#[derive(Debug, Clone)]
pub struct GenerateIf {
    pub condition: Expression,
    pub then_items: Vec<GenerateItem>,
    pub else_items: Option<Vec<GenerateItem>>,
}

/// Function declaration
#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub loc: Location,
    pub return_type: DataType,
    pub name: String,
    pub ports: Vec<FunctionPort>,
    pub items: Vec<FunctionItem>,
    pub statement: Option<Statement>,
}

#[derive(Debug, Clone)]
pub struct FunctionPort {
    pub direction: PortDirection,
    pub data_type: DataType,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum FunctionItem {
    Declaration(RegDeclaration),
}

/// Task declaration
#[derive(Debug, Clone)]
pub struct TaskDeclaration {
    pub loc: Location,
    pub name: String,
    pub ports: Vec<FunctionPort>,
    pub items: Vec<FunctionItem>,
    pub statement: Option<Statement>,
}

/// Statements
#[derive(Debug, Clone)]
pub enum Statement {
    /// Null statement (;)
    Null(Location),
    /// Sequential block (begin...end)
    SeqBlock {
        loc: Location,
        name: Option<String>,
        declarations: Vec<RegDeclaration>,
        statements: Vec<Statement>,
    },
    /// Parallel block (fork...join)
    ParBlock {
        loc: Location,
        name: Option<String>,
        declarations: Vec<RegDeclaration>,
        statements: Vec<Statement>,
    },
    /// Blocking assignment (=)
    BlockingAssignment {
        loc: Location,
        lhs: Expression,
        rhs: Expression,
    },
    /// Non-blocking assignment (<=)
    NonBlockingAssignment {
        loc: Location,
        lhs: Expression,
        delay: Option<Delay>,
        rhs: Expression,
    },
    /// If statement
    If {
        loc: Location,
        condition: Expression,
        then_stmt: Box<Statement>,
        else_stmt: Option<Box<Statement>>,
    },
    /// Case statement
    Case {
        loc: Location,
        case_type: CaseType,
        expr: Expression,
        items: Vec<CaseItem>,
        default: Option<Box<Statement>>,
    },
    /// For loop
    For {
        loc: Location,
        init: Box<Statement>,
        condition: Expression,
        update: Box<Statement>,
        body: Box<Statement>,
    },
    /// While loop
    While {
        loc: Location,
        condition: Expression,
        body: Box<Statement>,
    },
    /// Repeat loop
    Repeat {
        loc: Location,
        count: Expression,
        body: Box<Statement>,
    },
    /// Forever loop
    Forever {
        loc: Location,
        body: Box<Statement>,
    },
    /// Task/function call
    Call {
        loc: Location,
        name: String,
        arguments: Vec<Expression>,
    },
    /// Delay statement
    Delay {
        loc: Location,
        delay: Delay,
        statement: Option<Box<Statement>>,
    },
    /// Event control
    EventControl {
        loc: Location,
        event: EventExpression,
        statement: Option<Box<Statement>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum CaseType {
    Case,
    Casex,
    Casez,
}

#[derive(Debug, Clone)]
pub struct CaseItem {
    pub expressions: Vec<Expression>,
    pub statement: Statement,
}

#[derive(Debug, Clone)]
pub enum EventExpression {
    Star,
    Edge(Edge, Expression),
    Expression(Expression),
    Or(Box<EventExpression>, Box<EventExpression>),
}

/// Expressions
#[derive(Debug, Clone)]
pub enum Expression {
    /// Number literal
    Number(Location, NumberLiteral),
    /// String literal
    String(Location, String),
    /// Identifier
    Identifier(Location, String),
    /// Bit select: expr[index]
    BitSelect {
        loc: Location,
        expr: Box<Expression>,
        index: Box<Expression>,
    },
    /// Part select: expr[msb:lsb]
    PartSelect {
        loc: Location,
        expr: Box<Expression>,
        msb: Box<Expression>,
        lsb: Box<Expression>,
    },
    /// Concatenation: {expr1, expr2, ...}
    Concat {
        loc: Location,
        exprs: Vec<Expression>,
    },
    /// Replication: {count{expr}}
    Replicate {
        loc: Location,
        count: Box<Expression>,
        expr: Box<Expression>,
    },
    /// Function call
    FunctionCall {
        loc: Location,
        name: String,
        args: Vec<Expression>,
    },
    /// Unary operator
    Unary {
        loc: Location,
        op: UnaryOp,
        expr: Box<Expression>,
    },
    /// Binary operator
    Binary {
        loc: Location,
        op: BinaryOp,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    /// Ternary conditional: cond ? true_expr : false_expr
    Ternary {
        loc: Location,
        cond: Box<Expression>,
        true_expr: Box<Expression>,
        false_expr: Box<Expression>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
    BitwiseAnd,    // Reduction AND
    BitwiseNand,
    BitwiseOr,     // Reduction OR
    BitwiseNor,
    BitwiseXor,    // Reduction XOR
    BitwiseXnor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Power,

    // Relational
    Equal,
    NotEqual,
    CaseEqual,
    CaseNotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Logical
    LogicalAnd,
    LogicalOr,

    // Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseXnor,

    // Shift
    ShiftLeft,
    ShiftRight,
    ArithShiftLeft,
    ArithShiftRight,
}

impl Expression {
    pub fn location(&self) -> &Location {
        match self {
            Expression::Number(loc, _) => loc,
            Expression::String(loc, _) => loc,
            Expression::Identifier(loc, _) => loc,
            Expression::BitSelect { loc, .. } => loc,
            Expression::PartSelect { loc, .. } => loc,
            Expression::Concat { loc, .. } => loc,
            Expression::Replicate { loc, .. } => loc,
            Expression::FunctionCall { loc, .. } => loc,
            Expression::Unary { loc, .. } => loc,
            Expression::Binary { loc, .. } => loc,
            Expression::Ternary { loc, .. } => loc,
        }
    }
}
