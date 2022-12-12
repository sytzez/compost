#[derive(Clone, Eq, PartialEq, Debug)]
pub enum RawValue {
    Int(i64),
    String(String),
    Bool(bool),
}
