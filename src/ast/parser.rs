use crate::ast::type_statement::TypeStatement;
use crate::sem::typ::Type;
use crate::error::CResult;
use crate::lex::token::{Op, Token};
use crate::lex::tokenizer::LeveledToken;
use crate::lex::tokens::Tokens;

/// Something that can be created by parsing tokens.
pub trait Parser where Self: Sized {
    /// Whether the upcoming tokens match this type of parser.
    fn matches(tokens: &[LeveledToken]) -> bool;

    /// Parse the tokens into a statement for the abstract syntax tree.
    fn parse(tokens: &mut Tokens) -> CResult<Self>;

    /// Parse tokens if they match this parser, otherwise do nothing.
    fn maybe_parse(tokens: &mut Tokens) -> CResult<Option<Self>> {
        if Self::matches(tokens.remaining()) {
            Ok(Some(Self::parse(tokens)?))
        } else {
            Ok(None)
        }
    }
}

/// Parses a global name.
pub fn parse_global(tokens: &mut Tokens) -> CResult<String> {
    if let Token::Global(name) = tokens.token().clone() {
        tokens.step();
        Ok(name)
    } else {
        tokens.error(format!("Expected global name, got {:?} ", tokens.leveled_token()))
    }
}

/// Parses a local name.
pub fn parse_local(tokens: &mut Tokens) -> CResult<String> {
    if let Token::Local(name) = tokens.token().clone() {
        tokens.step();
        Ok(name)
    } else {
        tokens.error(format!("Expected local name, got {:?} ", tokens.leveled_token()))
    }
}

/// Parses parameters and an output type. Occurs in traits and lets.
pub fn parse_in_out_types(tokens: &mut Tokens, base_level: usize) -> CResult<(Vec<(String, TypeStatement)>, TypeStatement)> {
    let mut parameters = vec![];
    let mut output = None;

    while tokens.deeper_than(base_level) {
        if let Some(typ) = Type::maybe_parse(tokens)? {
            output = Some(typ);
            break;
        } else if matches!(tokens.token(), Token::Local(_)) {
            let parameter = parse_parameter(tokens)?;
            parameters.push(parameter)
        } else if matches!(tokens.token(), Token::Op(Op::Sub)) {
            tokens.step();

            if ! matches!(tokens.token(), Token::Op(Op::Gt)) {
                return tokens.error("Expected > after -".to_string())
            }
            tokens.step();

            output = Some(TypeStatement::parse(tokens)?);
            break;
        } else {
            return tokens.error(format!("Unexpected token {:?}", tokens.token()))
        }
    }

    let output = match output {
        Some(output) => output,
        None => return tokens.error("Expected output type".to_string())
    };

    Ok((parameters, output))
}

/// Parses a parameter name and type.
pub fn parse_parameter(tokens: &mut Tokens) -> CResult<(String, TypeStatement)> {
    let name = parse_local(tokens)?;
    let typ = TypeStatement::parse(tokens)?;
    Ok((name, typ))
}