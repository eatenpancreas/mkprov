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
mod parsing {
    mod files {
        mod simple;
    }
}

#[cfg(test)]
mod common {
    mod capacity;
}
