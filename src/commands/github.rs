pub mod gh;
mod repository;

use serenity::framework::standard::macros::group;

use self::gh::*;

#[group]
#[commands(github)]
struct GitHub;
