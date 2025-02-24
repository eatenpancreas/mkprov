use crate::{
    common::{
        CountryTag,
        ItemKind::{self},
    },
    structure::{ProvinceCsv, ProvinceHistories},
};

use itertools::Itertools;
use mkprov_lib::workspace::Workspace;

#[derive(clap::Args, Clone, Debug)]
pub struct QueryArgs {
    /// The kind of items selected
    pub kind: ItemKind,
    /// The item names or ids
    pub items: Vec<String>,
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum QueryError {
    #[error("Failed to provide items")]
    ItemsEmpty,
}

impl QueryArgs {
    pub fn query_province_ids(self, workspace: &Workspace) -> Result<Vec<u16>, QueryError> {
        if self.items.is_empty() {
            return Err(QueryError::ItemsEmpty);
        }

        Ok(match self.kind {
            ItemKind::Province => self
                .items
                .iter()
                .filter_map(|item| {
                    let csv = ProvinceCsv::load(&workspace);
                    if let Ok(i) = item.parse() {
                        if let Some(_) = csv.get(&i) {
                            return Some(i);
                        }
                    }

                    csv.search_name(&item)
                })
                .collect_vec(),
            ItemKind::Area => vec![],
            ItemKind::Country => vec![],
        })
    }
}

/// Actions
#[derive(clap::Args, Clone, Debug)]
pub struct QueryActions {
    /// [action] Sets the owner of the queried items
    #[arg(short, long)]
    pub(crate) owner: Option<String>,
    /// [action] Sets the continent of the queried items
    #[arg(long)]
    pub(crate) continent: Option<String>,
    /// [action] Sets the tradenode of the queried items
    #[arg(long)]
    pub(crate) tradenode: Option<String>,
    /// [action] Sets the climate of the queried items
    #[arg(long)]
    pub(crate) climate: Option<ClimateType>,
    /// [action] Sets the monsoon of the queried items
    #[arg(long)]
    pub(crate) monsoon: Option<Severity>,
    /// [action] Sets the winter of the queried items
    #[arg(long)]
    pub(crate) winter: Option<Severity>,
    /// [action] Sets the culture of the queried items
    #[arg(long)]
    pub(crate) culture: Option<String>,
    /// [action] Sets the religion of the queried items
    #[arg(long)]
    pub(crate) religion: Option<String>,
    /// [action] Sets the trade goods of the queried items
    #[arg(long)]
    pub(crate) goods: Option<String>,
    /// [action] Sets the trade wind of the queried items
    #[arg(long)]
    pub(crate) trade_wind: Option<u16>,
    /// [action] Deletes the trade wind of the queried items
    #[arg(long)]
    pub(crate) delete_trade_wind: bool,
    /// [action] Sets the queried items to be a sea
    #[arg(long)]
    pub(crate) sea_start: Option<bool>,
    /// [action] Sets the queried items to be used for random new world
    #[arg(long)]
    pub(crate) rnw: Option<bool>,
    /// [action] Sets the queried items to be a lake
    #[arg(long)]
    pub(crate) lake: Option<bool>,
    /// [action] Sets the queried items to be forced coastal
    #[arg(long)]
    pub(crate) force_coastal: Option<bool>,
    /// [action] Deletes the queried items
    #[arg(long)]
    pub(crate) delete: bool,
    /// [action] Prints the ids of the provinces in the queried items
    #[arg(long)]
    pub(crate) print: bool,
    /// Auto-confirms any potentially destructive action
    #[arg(short, long)]
    pub(crate) yes_to_all: bool,
}

use clap::ValueEnum;
use thiserror::Error;

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum ClimateType {
    Tropical,
    Arid,
    Arctic,
    #[default]
    Normal,
}

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum Severity {
    Normal,
    Mild,
    Severe,
    #[default]
    None,
}

#[derive(Error, Debug, Clone, Copy)]
#[error("No action given for queried items")]
pub struct NoActionGiven;

impl QueryActions {
    pub fn execute(self, provinces: Vec<u16>, workspace: &Workspace) -> Result<(), NoActionGiven> {
        match self {
            QueryActions {
                owner: None,
                continent: None,
                tradenode: None,
                climate: None,
                monsoon: None,
                winter: None,
                delete: false,
                print: false,
                yes_to_all: _, // not an action
                trade_wind: None,
                delete_trade_wind: false,
                sea_start: None,
                rnw: None,
                lake: None,
                force_coastal: None,
                culture: None,
                religion: None,
                goods: None,
            } => return Err(NoActionGiven),
            _ => {
                if let Some(owner) = self.owner {
                    for id in provinces {
                        match ProvinceHistories::load(&workspace).find_id(id, &workspace) {
                            Some(mut hist) => {
                                hist.set_owner(CountryTag::new(owner.chars()).unwrap());
                                hist.save(&workspace, !self.yes_to_all).unwrap();
                            }
                            None => println!("could not find {id} in province histories!"),
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
