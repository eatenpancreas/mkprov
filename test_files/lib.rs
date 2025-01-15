pub mod simple {
    pub const COMMENT: &str = include_str!("simple/comment.txt");
    pub const COMMENTS_OBJECT: &str = include_str!("simple/comments_object.txt");
    pub const DATE_OBJECT: &str = include_str!("simple/date_object.txt");
    pub const KEYVAL: &str = include_str!("simple/keyval.txt");
    pub const WONKY_OBJECT: &str = include_str!("simple/wonky_object.txt");
}

pub mod game_files {
    pub const ADAL: &str = include_str!("game_files/adal.txt");
    pub const DAGOBAH: &str = include_str!("game_files/dagobah.txt");
}

pub mod massive {
    pub const ACHIEVEMENTS: &str = include_str!("massive/achievements.txt");
    pub const AGES: &str = include_str!("massive/ages.txt");
}
