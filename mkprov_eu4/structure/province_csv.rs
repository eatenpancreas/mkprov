use std::collections::HashMap;

use cli_prompts::{DisplayPrompt, prompts::Selection, style::SelectionStyle};
use itertools::Itertools;
use mkprov_lib::{
    common::Color,
    workspace::{AnyCsv, Workspace, WorkspaceFile},
};
use rust_fuzzy_search::fuzzy_search_sorted;

pub struct ProvinceCsv {
    _file: WorkspaceFile<AnyCsv>,
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

        Self { _file: def_file, data }
    }

    fn index_of_name(&self, name: &str) -> Option<u16> {
        self.data.iter().find_map(|(i, (_, n))| {
            (n == name || normalise_name(n).is_some_and(|n| n == name)).then_some(*i)
        })
    }

    pub fn identify_duplicates(&mut self) {
        let mut counts = HashMap::new();
        self.data
            .iter_mut()
            .for_each(|(i, (_, name))| match counts.get_mut(&**name) {
                Some(_) => {
                    name.push_str(&format!(" ({i})"));
                }
                None => {
                    counts.insert(&**name, ());
                }
            });
    }

    pub fn search_name(&self, search_for: &str) -> Option<u16> {
        let lowercase_search = search_for.to_lowercase();
        let mut names = self.data.values().map(|(_, name)| &**name).collect_vec();
        let duplicated = names.iter().filter_map(|n| normalise_name(n)).collect_vec();
        names.extend(duplicated.iter().map(|s| &**s));
        let search = fuzzy_search_sorted(&lowercase_search, names.as_slice());
        let mut search_iter = search.into_iter();

        if let Some((first_name, first_score)) = search_iter.next() {
            // exact match
            if first_score == 1. {
                return self.index_of_name(&first_name);
            } else {
                let mut similar_scores = search_iter
                    .filter(|(_, score)| (first_score - score).abs() <= 0.2)
                    .collect_vec();

                // first score is very similar and no others come close
                if first_score > 0.5 && similar_scores.len() == 0 {
                    eprintln!(
                        "Assuming {search_for:?} to be {:?}",
                        capitalize_first_letter(first_name)
                    );
                    return self.index_of_name(&first_name);
                } else if first_score > 0.5 {
                    similar_scores.push((first_name, first_score));

                    let chosen = Selection::new(
                        "(ESC to quit) No exact match was found, but similar province names exist: ",
                        similar_scores.iter().map(|(name, _)| capitalize_first_letter(name)),
                    )
                    .style(SelectionStyle::default())
                    .display()
                    .unwrap();

                    return self.index_of_name(&chosen);
                }

                eprintln!("No match found for {search_for:?}");
            }
        }

        None
    }
}

fn capitalize_first_letter(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let (first, rest) = s.split_at(1);
    first.to_uppercase() + rest
}

fn normalise_name(search: &str) -> Option<String> {
    let mut out = String::new();

    for c in search.to_lowercase().chars() {
        match c {
            // Nordic Letters
            'æ' => out.push_str("ae"),
            'ø' => out.push_str("o"),
            'å' => out.push_str("a"),

            // Germanic Letters
            'ß' => out.push_str("ss"),
            'ü' => out.push_str("u"),
            'ö' => out.push_str("o"),
            'ä' => out.push_str("a"),

            // French Letters
            'é' | 'è' | 'ê' | 'ë' => out.push('e'),
            'ç' => out.push('c'),

            // Iberian Letters
            'ñ' => out.push('n'),
            'á' => out.push('a'),
            'ó' => out.push('o'),
            'í' => out.push('i'),
            'ú' => out.push('u'),

            // Other Letters
            'ł' => out.push('l'),
            'ð' => out.push('d'),
            'þ' => out.push_str("th"),
            _ => out.push(c),
        }
    }

    (&out != search).then_some(out)
}
