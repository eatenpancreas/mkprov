mod parser;
#[cfg(test)] mod tests;
mod lexer;

#[derive(Debug)]
pub struct ParadoxFile {
    pub(crate) raw: String,
}

impl From<String> for ParadoxFile {
    fn from(raw: String) -> Self {
        ParadoxFile { raw: raw.to_string() }
    }
}

#[derive(Debug)]
pub struct Field<'f> {
    location: Location,
    ft: FieldType<'f>,
}

#[derive(Debug)]
pub enum FieldType<'f> {
    KeyVal(KeyVal<'f>),
    Literal(Literal<'f>),
    Object(Object<'f>),
}

#[derive(Debug)]
pub enum Literal<'f> {
    U8(u8),
    U16(u16),
    F32(f32),
    String(&'f str),
    Date(Date),
}

#[derive(Debug, PartialEq)]
pub struct Location(usize);

#[derive(Debug)]
pub struct Object<'f> {
    fields: Vec<Field<'f>>,
}

#[derive(Debug)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Debug)]
pub struct KeyVal<'f> {
    key: Literal<'f>,
    value: Literal<'f>,
    value_location: Location,
}
