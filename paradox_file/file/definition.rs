use std::collections::HashMap;
use crate::{Config, LocalisationFileError};
use crate::color::Color;
use crate::files::{LocalFile, sort_hashmap};

pub struct DefinitionCsv {
    title: String,
    rows: HashMap<u16, DefinitionRow>,
    file: LocalFile
}

pub struct DefinitionRow {
    color: Color,
    name: String
}

impl DefinitionCsv {
    pub fn save(&self) -> bool {
        let mut x = self.title.to_string();
        x.push('\n');
        let kvs = sort_hashmap(&self.rows);

        for (key, value) in kvs {
            x.push_str(format!("{};{};{};{};{};x\n", 
                key, value.color.r(), value.color.g(), value.color.b(), value.name).as_str())
        }

        self.file.write_contents(&x)
    }

    pub fn max_id(&self) -> u16 {
        self.rows.keys().map(|k| *k).max().unwrap_or(1)
    }

    pub fn push(&mut self, id: u16, color: Color, name: String) {
        self.rows.insert(id, DefinitionRow {
            color,
            name,
        });
    }

    pub fn rename(&mut self, key: u16, to: String) {
        if let Some(v) = self.rows.get_mut(&key) {
            v.name = to;
        }
    }

    pub fn load(cfg: &Config) -> Result<DefinitionCsv, LocalisationFileError> {
        let (lines, title, file) = 
          load_csv(cfg, "map", "definition.csv")?;
        let mut rows = HashMap::new();

        for line in lines {
            if let Some((index, row)) = Self::row(line) {
                rows.insert(index, row);
            }
        }

        Ok(DefinitionCsv {
            title,
            file,
            rows
        })
    }

    fn row(mut line: Vec<String>) -> Option<(u16, DefinitionRow)> {
        Some((
            line.get(0)?.parse().ok()?,
            DefinitionRow {
                color: Color::new_rgb([
                    line.get(1)?.parse().ok()?,
                    line.get(2)?.parse().ok()?,
                    line.get(3)?.parse().ok()?,
                ]),
                name: line.remove(4),
            }
        ))
    }
}

fn load_csv(
    cfg: &Config, sub_directory: &str, filename: &str
) -> Result<(Vec<Vec<String>>, String, LocalFile), LocalisationFileError> {
    let file = LocalFile::get_file(
        cfg.require_mod_directory()?, sub_directory, &filename).unwrap();
    let contents_str = file.get_contents()?;
    let mut contents = contents_str.split('\n');
    let title = contents.next().ok_or(LocalisationFileError::UnexpectedFormat)?.to_string();
    let mut lines = vec![];

    for line in contents {
        lines.push(line.split(';').map(|x| x.to_string()).collect())
    }

    Ok((lines, title, file))
}