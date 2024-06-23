mod parsing;

struct ParadoxFile {
    fields: Vec<Field>
}

struct Field {
    location: Location,
    ft: FieldType
}

enum FieldType {
    Variable(Variable),
    Literal(Literal),
    Object(Object)
}

enum Literal {
    U8(u8),
    U16(u16),
    F32(f32),
    String(String),
    Date(Date),
}

struct Location {
    row: u32,
    col: u32
}

struct Object {
    fields: Vec<Field>
}

struct Date {
    year: u8,
    month: u8,
    day: u8
}

struct Variable {
    key: String,
    value: Literal
}

