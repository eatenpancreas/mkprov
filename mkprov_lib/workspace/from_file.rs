mod sealed {
    pub trait SealedFromFile {}
}

use csv::{IntoInnerError, ReaderBuilder, StringRecord, Writer, WriterBuilder};
use derived_deref::{Deref, DerefMut};
use pdxsyn::{Document, Lexer, PdxError, syntax::RootObject};
use sealed::SealedFromFile;
use serde::{Serialize, de::DeserializeOwned};
use std::{borrow::Cow, string::FromUtf8Error};
use thiserror::Error;
use yaml_rust2::{EmitError, ScanError, Yaml, YamlEmitter, YamlLoader};

pub trait FromFile: Sized + SealedFromFile {
    type FromFileError;
    type IntoFileError;
    fn into_file(self) -> Result<String, Self::IntoFileError>;
    fn from_file(str: Cow<str>) -> Result<Self, Self::FromFileError>;
}

#[derive(Deref, DerefMut, Debug)]
pub struct AnyYaml(Yaml);

#[derive(Error, Debug)]
pub enum YamlFromFileError {
    #[error(transparent)]
    ScanError(#[from] ScanError),
    #[error("No yaml found")]
    NoYaml,
}

impl SealedFromFile for AnyYaml {}
impl FromFile for AnyYaml {
    type FromFileError = YamlFromFileError;
    type IntoFileError = EmitError;
    fn into_file(self) -> Result<String, Self::IntoFileError> {
        let mut out_str = String::new();
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&self)?;
        Ok(out_str)
    }
    fn from_file(str: Cow<str>) -> Result<Self, Self::FromFileError> {
        YamlLoader::load_from_str(&str)?
            .pop()
            .ok_or(YamlFromFileError::NoYaml)
            .map(|o| Self(o))
    }
}

impl SealedFromFile for String {}
impl FromFile for String {
    type FromFileError = ();
    type IntoFileError = ();
    fn into_file(self) -> Result<String, Self::IntoFileError> { Ok(self) }
    fn from_file(str: Cow<str>) -> Result<Self, Self::FromFileError> { Ok(str.to_string()) }
}

impl SealedFromFile for Document {}
impl FromFile for Document {
    type FromFileError = PdxError;
    type IntoFileError = ();
    fn into_file(self) -> Result<String, Self::IntoFileError> { Ok(self.into_string()) }
    fn from_file(str: Cow<str>) -> Result<Self, Self::FromFileError> {
        let lex: Result<Vec<_>, _> = Lexer::new(&str).collect();
        Ok(Document::create(lex?))
    }
}

#[derive(Deref, DerefMut, Debug)]
pub struct AnyCsv {
    headers: StringRecord,
    #[target]
    records: Vec<StringRecord>,
}

impl AnyCsv {
    #[inline]
    pub fn into_records(self) -> Vec<StringRecord> { self.records }
    #[inline]
    pub fn new<'a>(records: Vec<StringRecord>, headers: impl Iterator<Item = &'a str>) -> Self {
        let headers = headers.fold(StringRecord::new(), |mut acc, field| {
            acc.push_field(field);
            acc
        });
        Self { records, headers }
    }
}

impl SealedFromFile for AnyCsv {}
impl FromFile for AnyCsv {
    type FromFileError = csv::Error;
    type IntoFileError = CsvIntoFileError;
    fn into_file(self) -> Result<String, Self::IntoFileError> {
        let mut writer = WriterBuilder::new().delimiter(b';').from_writer(vec![]);
        writer.write_record(&self.headers)?;
        for i in self.into_records() {
            writer.write_record(i.iter())?;
        }
        Ok(String::from_utf8(writer.into_inner()?)?)
    }

    fn from_file(str: Cow<str>) -> Result<Self, Self::FromFileError> {
        let mut reader = ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(str.as_bytes());

        let headers = reader.headers()?.clone();
        let records = reader.records().collect::<Result<_, _>>()?;
        Ok(AnyCsv { records, headers })
    }
}

#[derive(Deref, DerefMut, Debug)]
pub struct Csv<T: Serialize + DeserializeOwned> {
    headers: StringRecord,
    #[target]
    records: Vec<T>,
}

impl<T: Serialize + DeserializeOwned> Csv<T> {
    #[inline]
    pub fn into_records(self) -> Vec<T> { self.records }
    #[inline]
    pub fn new<'a>(records: Vec<T>, headers: impl Iterator<Item = &'a str>) -> Self {
        let headers = headers.fold(StringRecord::new(), |mut acc, field| {
            acc.push_field(field);
            acc
        });
        Self { records, headers }
    }
}

#[derive(Error, Debug)]
pub enum CsvIntoFileError {
    #[error(transparent)]
    IntoInnerError(#[from] IntoInnerError<Writer<Vec<u8>>>),
    #[error(transparent)]
    Utf8Error(#[from] FromUtf8Error),
    #[error(transparent)]
    CsvError(#[from] csv::Error),
}

impl<T: Serialize + DeserializeOwned> SealedFromFile for Csv<T> {}
impl<T: Serialize + DeserializeOwned> FromFile for Csv<T> {
    type FromFileError = csv::Error;
    type IntoFileError = CsvIntoFileError;
    fn into_file(self) -> Result<String, Self::IntoFileError> {
        let mut writer = WriterBuilder::new().delimiter(b';').from_writer(vec![]);
        writer.write_record(&self.headers)?;
        for i in self.into_records() {
            writer.serialize(i)?;
        }
        Ok(String::from_utf8(writer.into_inner()?)?)
    }
    fn from_file(str: Cow<str>) -> Result<Self, Self::FromFileError> {
        let mut reader = ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(str.as_bytes());

        let headers = reader.headers()?.clone();
        let records: Result<Vec<_>, _> = reader.deserialize().collect();
        Ok(Csv { records: records?, headers })
    }
}

impl SealedFromFile for (Document, RootObject) {}
impl FromFile for (Document, RootObject) {
    type FromFileError = PdxError;
    type IntoFileError = ();
    fn into_file(self) -> Result<String, Self::IntoFileError> { self.0.into_file() }
    fn from_file(str: Cow<str>) -> Result<Self, Self::FromFileError> {
        let doc = Document::from_file(str)?;
        let parsed = doc.parse()?;
        Ok((doc, parsed))
    }
}
