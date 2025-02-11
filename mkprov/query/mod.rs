use std::{collections::HashMap, fmt::Display};

use ItemKind::*;
use clap::{Parser, ValueEnum};

use itertools::Itertools;
use mkprov_lib::workspace::Workspace;
use rust_fuzzy_search::fuzzy_search_sorted;

/// The query part of Mkprov.
/// Use mkprov --action to display help for the action part of Mkprov
///
/// Combine the output with the action part using the pipe operator |
#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
#[command(name = "mkprov (query)")]
#[command(bin_name = "mkprov")]
#[command(version, about)]
pub struct QueryArgs {
    /// The kind of item selected
    #[arg(required_unless_present = "action")]
    pub kind: Option<ItemKind>,
    /// The item names or ids
    pub items: Vec<String>,
    #[arg(long, help = "Show help for the second part of mkprov")]
    pub action: bool,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum ItemKind {
    /// Select one or more provinces.
    /// Aliases = p, prov
    #[clap(aliases(["p", "prov"]))]
    Province,
    /// Select one or more areas.
    /// Aliases = a
    #[clap(alias("a"))]
    Area,
}

#[derive(Debug)]
pub struct QueryOutput {
    unsure: Option<String>,
    kind: ItemKind,
    name: String,
}

impl QueryOutput {
    pub fn is_sure(&self) -> bool { self.unsure.is_none() }
    pub fn unsure(&self) -> Option<&str> { self.unsure.as_ref().map(|t| t.as_str()) }
    fn new(unsure: Option<String>, kind: ItemKind, name: impl ToString) -> QueryOutput {
        QueryOutput { unsure, kind, name: name.to_string() }
    }
    pub fn parse(string: &str) -> QueryOutput {
        let mut spl = string.split("?");
        let left = spl.next().unwrap();

        let (unsure, item) = match spl.next() {
            Some(n) => (Some(left), n),
            None => (None, left),
        };

        let mut spl2 = item.split(":");
        let kind = spl2.next().unwrap();
        let name = spl2.next().unwrap();

        return QueryOutput {
            unsure: unsure.map(|t| t.to_owned()),
            kind: match kind {
                "province" => Province,
                "area" => Area,
                k => panic!("query kind {k} not allowed"),
            },
            name: name.to_string(),
        };
    }
}

impl Display for QueryOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let question = match &self.unsure {
            Some(n) => &format!("{n}?"),
            None => "",
        };

        let kind = match self.kind {
            Province => "province",
            Area => "area",
        };
        let name = &self.name;

        write!(f, "{question}{kind}:{name}")
    }
}

impl QueryArgs {
    pub fn main(kind: ItemKind, items: Vec<String>, workspace: Workspace) -> Vec<QueryOutput> {
        items
            .into_iter()
            .filter_map(|item| match kind {
                Province => get_province(&workspace, item),
                Area => None,
            })
            .collect()
    }
}

fn get_province(workspace: &Workspace, prov: String) -> Option<QueryOutput> {
    // Check if input is number
    if let Ok(p) = prov.parse::<u16>() {
        return Some(QueryOutput::new(None, Province, p));
    }

    // Look at definition for name
    let def = workspace.get_any_csv_file("map/definition.csv");
    let def = def.load_either().unwrap();
    let def = def
        .iter()
        .flat_map(|row| Some((row.get(4)?, row.get(0)?)))
        .collect::<HashMap<_, _>>();

    let names = def.keys().map(|t| *t).collect_vec();

    let mut search = fuzzy_search_sorted(&prov, names.as_slice()).into_iter();

    // eprintln!("{:?}", search);

    if let Some((first_name, first_score)) = search.next() {
        if first_score == 1. {
            return def
                .get(first_name)
                .and_then(|p| Some(QueryOutput::new(None, Province, p)));
        } else if first_score >= 0.5
            && search
                .next()
                .is_some_and(|(_, second_score)| first_score >= second_score + 0.2)
        {
            return def.get(first_name).and_then(|p| {
                eprintln!("Assuming {prov:?} to be {first_name:?}");
                Some(QueryOutput::new(None, Province, p))
            });
        } else if first_score >= 0.1 {
            return def
                .get(first_name)
                .and_then(|p| Some(QueryOutput::new(Some(prov), Province, p)));
        }
    }

    None
}
