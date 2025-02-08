#[cfg(test)]
mod lexing {
    mod files {
        mod game_files;
        mod simple;
    }
    mod display_identical;
    mod numbers;
}

#[cfg(test)]
mod modification {
    mod inserts;
}

#[cfg(test)]
mod parsing {
    mod massive;
    mod simple;
}

#[cfg(test)]
mod common {
    mod capacity;
    mod date_tests;
}
