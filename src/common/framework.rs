use serenity::{framework::StandardFramework, model::id::UserId};
use std::collections::HashSet;

use crate::commands::{self, meta};

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
    }
}
