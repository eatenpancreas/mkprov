use std::collections::HashMap;
use std::io;
use thiserror::Error;
use crate::{Config, RequireError};
use crate::files::LocalFile;

#[derive(Debug)]
pub struct LocalisationFile {
  pub title: String,
  pub key_vals: HashMap<u16, LocalisationValue>,
  file: LocalFile,
}

#[derive(Debug)]
pub struct LocalisationValue {
  prio: u8,
  name: String
}

#[derive(Error, Debug)]
pub enum LocalisationFileError {
  #[error("Something went wrong with getting files")]
  IoError(#[from] io::Error),
  #[error("Unexpected YML format")]
  UnexpectedFormat,
  #[error(transparent)]
  RequireError(#[from] RequireError)
}

impl LocalisationFile {
  pub fn load_localisation(cfg: &Config) -> Result<Self, LocalisationFileError> {
    Self::load(cfg, "localisation", "vap_prov_names_l_english.yml")
  }
  
  pub fn save(&self) -> bool {
    let mut x = self.title.to_string();
    x.push_str(":\n");
    let kvs = sort_hashmap(&self.key_vals);

    for (key, value) in kvs {
      x.push_str(format!(" PROV{}:{} \"{}\"\n", key, value.prio, value.name).as_str())
    }
    
    self.file.write_contents(&x)
  }
  
  pub fn replace_or_add_key_name(&mut self, key: u16, name: String, prio: Option<u8>) {
    if let Some(v) = self.key_vals.get_mut(&key) {
      v.name = name;
      v.prio = prio.unwrap_or(v.prio)
    } else {
      self.key_vals.insert(key, LocalisationValue {
        prio: prio.unwrap_or(0),
        name,
      });
    }
  }
  
  fn load(cfg: &Config, sub_directory: &str, filename: &str) -> Result<Self, LocalisationFileError> {
    let file = LocalFile::get_file(cfg.require_mod_directory()?, sub_directory, &filename)?;
    let contents_str = file.get_contents()?;
    let mut contents = contents_str.split('\n');
    let mut title = contents.next().ok_or(LocalisationFileError::UnexpectedFormat)?.to_string();
    let mut key_vals = HashMap::new();
    
    if !title.pop().is_some_and(|char| char == ':') { 
      return Err(LocalisationFileError::UnexpectedFormat)
    }
    
    for line in contents {
      // line example: PROV3004:0 "Gulf of Boothia"
      let line: Vec<&str> = line.split('"').collect();
      let before = match line.get(0) { Some(start) => *start, None => break };
      let value = match line.get(1) { Some(start) => *start, None => break };

      let before: Vec<&str> = before.split(|c: char| c.is_whitespace() || c == ':')
        .filter(|x| x.len() > 0).collect();

      let key = match before.get(0) { Some(start) => *start, None => break };
      let (_, key) = key.split_at(4);
      
      let key_u8 = key.parse();
      if key_u8.is_err() { continue }
      
      let prio = before.get(1).and_then(|str| str.parse().ok());
      if prio.is_none() { continue }
      
      key_vals.insert(key_u8.unwrap(), LocalisationValue {
        prio: prio.unwrap(),
        name: value.to_string(),
      });
    }
    
    Ok(LocalisationFile {
      title,
      key_vals,
      file,
    })
  }
}

pub(crate) fn sort_hashmap<T>(hm: &HashMap<u16, T>) -> Vec<(&u16, &T)> {
  let mut kvs: Vec<(&u16, &T)> = hm.iter().collect();
  kvs.sort_by(|(a, _),(b, _)| a.partial_cmp(b).unwrap());
  kvs
}