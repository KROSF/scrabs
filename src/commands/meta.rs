mod ping;

use serenity::framework::standard::macros::group;

use self::ping::*;

#[group]
#[commands(ping)]
struct Meta;
