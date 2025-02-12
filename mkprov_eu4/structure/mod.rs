use std::collections::HashMap;

use cli_prompts::{
    DisplayPrompt,
    prompts::{Prompt, Selection},
    style::SelectionStyle,
};
use itertools::Itertools;
use mkprov_lib::{
    common::Color,
    workspace::{AnyCsv, Workspace, WorkspaceFile},
};
use rust_fuzzy_search::fuzzy_search_sorted;

pub struct ProvinceCsv {
    file: WorkspaceFile<AnyCsv>,
    data: HashMap<u16, (Color, String)>,
}

impl ProvinceCsv {
    pub fn load(workspace: &Workspace) -> Self {
        let def_file = workspace.get_any_csv_file("map/definition.csv");
        let csv = def_file.load_either(workspace).unwrap();
        let data = csv
            .iter()
            .flat_map(|row| {
                Some((
                    row.get(0)?.parse().unwrap(),
                    (
                        Color::new([
                            row.get(1)?.parse().unwrap(),
                            row.get(2)?.parse().unwrap(),
                            row.get(3)?.parse().unwrap(),
                        ]),
                        row.get(4)?.to_owned(),
                    ),
                ))
            })
            .collect::<HashMap<_, _>>();

        Self { file: def_file, data }
    }

    fn index_of_name(&self, name: &str) -> Option<u16> {
        self.data
            .iter()
            .find_map(|(i, (_, n))| (n == name).then_some(*i))
    }

    pub fn search_name(&self, search_for: &str) -> Option<u16> {
        let names = self.data.values().map(|(_, name)| &**name).collect_vec();
        let search = fuzzy_search_sorted(search_for, names.as_slice());
        let mut search_iter = search.into_iter();

        if let Some((first_name, first_score)) = search_iter.next() {
            // exact match
            if first_score == 1. {
                return self.index_of_name(first_name);
            } else {
                let mut similar_scores = search_iter
                    .filter(|(_, score)| (first_score - score).abs() <= 0.2)
                    .collect_vec();

                // first score is very similar and no others come close
                if first_score > 0.5 && similar_scores.len() == 0 {
                    eprintln!("Assuming {search_for:?} to be {first_name:?}");
                    return self.index_of_name(first_name);
                } else if first_score > 0.5 {
                    similar_scores.push((first_name, first_score));

                    let chosen = Selection::new(
                        "(ESC to quit) No exact match was found, but similar province names exist: ",
                        similar_scores.iter().map(|(name, _)| *name),
                    )
                    .style(SelectionStyle::default())
                    .display()
                    .unwrap();

                    return self.index_of_name(chosen);
                }

                eprintln!("No match found for {search_for:?}");
            }
        }

        None
    }
}

// fn get_province(workspace: &Workspace, prov: String) -> Option<Item> {
//     // Check if input is number
//     if let Ok(p) = prov.parse::<u16>() {
//         return Some(QueryOutput::new(None, Province, p));
//     }

//     // Look at definition for name

//     let names = def.keys().map(|t| *t).collect_vec();

//     let mut search = fuzzy_search_sorted(&prov, names.as_slice()).into_iter();

//     // eprintln!("{:?}", search);

//     if let Some((first_name, first_score)) = search.next() {
//         if first_score == 1. {
//             return def
//                 .get(first_name)
//                 .and_then(|p| Some(QueryOutput::new(None, Province, p)));
//         } else if first_score >= 0.5
//             && search
//                 .next()
//                 .is_some_and(|(_, second_score)| first_score >= second_score + 0.2)
//         {
//             return def.get(first_name).and_then(|p| {
//                 eprintln!("Assuming {prov:?} to be {first_name:?}");
//                 Some(QueryOutput::new(None, Province, p))
//             });
//         } else if first_score >= 0.1 {
//             return def
//                 .get(first_name)
//                 .and_then(|p| Some(QueryOutput::new(Some(prov), Province, p)));
//         }
//     }

//     None
// }
