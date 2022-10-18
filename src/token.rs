// use std::io::BufReader;
//
// pub enum CharType {
//     WhiteSpace,
//     Keyword, // A-Za-z0-9_
//     Operator, // .+-/()
// }
//
// impl From<u8> for CharType {
//
// }
//
// pub struct Chunk {
//     charType: CharType,
//     content: String,
// }
//
// pub enum Token {
//     Down(Level),
//     Up,
//     Next(Next),
//     Eof,
//     Kw(Keyword),
//     Op(Operator),
//     Val(Value),
//     RawType(Raw),
// }
//
// pub enum Keyword {
//     Mod,
//     Class,
//     Struct,
//     Interface,
//     Fn,
//     Fns,
//     Trait,
//     Traits,
//     Def,
//     Defs,
// }
//
// pub enum Operator {
//     Dot,
//     Add,
//     Sub,
//     Div,
//     Mul,
//     Eq,
//     Lt,
//     Gt,
// }
//
// pub enum Value {
//     String,
//     Number,
// }
//
// pub enum RawType {
//     String,
//     Int,
//     UInt,
// }
//
// pub enum Level {
//     Colon,
//     Indentation,
//     Paren,
// }
//
// pub enum Next {
//     Comma,
//     NewLine,
// }
//
// fn next_chunk(reader: &BufReader<File>) -> Chunk {
//
// }
//
//
// fn tokenize_line(reader: &BufReader) -> Vec<Token> {
//
// }
