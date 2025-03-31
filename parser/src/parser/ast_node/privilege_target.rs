// Alias: `PrivTarget`
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PrivilegeTarget {
    AllFunctionsInSchema(Vec<Str>),
    AllProceduresInSchema(Vec<Str>),
    AllRoutinesInSchema(Vec<Str>),
    AllSequencesInSchema(Vec<Str>),
    AllTablesInSchema(Vec<Str>),
    Database(Vec<Str>),
    Domain(Vec<QualifiedName>),
    ForeignDataWrapper(Vec<Str>),
    ForeignServer(Vec<Str>),
    Function(Vec<FunctionWithArgs>),
    Language(Vec<Str>),
    LargeObject(Vec<SignedNumber>),
    ParameterAcl(Vec<QualifiedName>),
    Procedure(Vec<FunctionWithArgs>),
    Routine(Vec<FunctionWithArgs>),
    Schema(Vec<Str>),
    Sequence(Vec<RelationName>),
    Table(Vec<RelationName>),
    Tablespace(Vec<Str>),
    Type(Vec<QualifiedName>),
}

use crate::parser::ast_node::FunctionWithArgs;
use crate::parser::ast_node::QualifiedName;
use crate::parser::ast_node::RelationName;
use crate::parser::ast_node::SignedNumber;
use postgres_basics::Str;
