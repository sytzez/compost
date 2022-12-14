use crate::ast::type_statement::TypeStatement;

use crate::error::CResult;
use crate::lex::token::{Op, Token};
use crate::lex::tokens::Tokens;

/// Something that can be created by parsing tokens.
pub trait Parse
where
    Self: Sized,
{
    /// Whether the upcoming tokens match this type of parser.
    fn matches(tokens: &Tokens) -> bool;

    /// Parse the tokens into a statement for the abstract syntax tree.
    fn parse(tokens: &mut Tokens) -> CResult<Self>;

    /// Parse tokens if they match this parser, otherwise do nothing.
    fn maybe_parse(tokens: &mut Tokens) -> CResult<Option<Self>> {
        if Self::matches(tokens) {
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
        tokens.unexpected_token_error()
    }
}

/// Parses a local name.
pub fn parse_local(tokens: &mut Tokens) -> CResult<String> {
    if let Token::Local(name) = tokens.token().clone() {
        tokens.step();
        Ok(name)
    } else {
        tokens.unexpected_token_error()
    }
}

/// Parses parameters and an output type. Occurs in traits and lets.
pub fn parse_in_out_types(
    tokens: &mut Tokens,
    base_level: usize,
) -> CResult<(Vec<(String, TypeStatement)>, TypeStatement)> {
    let mut parameters = vec![];
    let mut output = None;

    while tokens.deeper_than(base_level) {
        tokens.expect("an input parameter name or an output type");
        if let Some(typ) = TypeStatement::maybe_parse(tokens)? {
            output = Some(typ);
            break;
        } else if matches!(tokens.token(), Token::Local(_)) {
            let parameter = parse_parameter(tokens)?;
            parameters.push(parameter)
        } else if matches!(tokens.token(), Token::Op(Op::Sub)) {
            tokens.step();

            if !matches!(tokens.token(), Token::Op(Op::Gt)) {
                return tokens.unexpected_token_error();
            }
            tokens.step();

            output = Some(TypeStatement::parse(tokens)?);
            break;
        } else {
            return tokens.unexpected_token_error();
        }
    }

    let output = match output {
        Some(output) => output,
        None => return tokens.unexpected_token_error(),
    };

    Ok((parameters, output))
}

/// Parses a parameter name and type.
pub fn parse_parameter(tokens: &mut Tokens) -> CResult<(String, TypeStatement)> {
    tokens.expect("Parameter name (lower case)");
    let name = parse_local(tokens)?;
    let typ = TypeStatement::parse(tokens)?;
    Ok((name, typ))
}
