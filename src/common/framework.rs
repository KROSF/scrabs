use serenity::{framework::StandardFramework, model::id::UserId};
use std::collections::HashSet;

use crate::commands::{self, github, meta, music};

pub struct ScrabsFramework;

impl ScrabsFramework {
    pub fn with_owners(owners: HashSet<UserId>) -> StandardFramework {
        StandardFramework::new()
            .configure(|c| {
                c.with_whitespace(true)
                    .prefix("$")
                    .delimiters(vec![", ", ","])
                    .owners(owners)
            })
            .help(&commands::SCRABS_HELP)
            .group(&meta::META_GROUP)
            .group(&music::MUSIC_GROUP)
            .group(&github::GITHUB_GROUP)
    }
}
