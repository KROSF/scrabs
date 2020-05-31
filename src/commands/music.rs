mod deafen;
mod join;
mod leave;
mod mute;
mod play;
mod undeafen;
mod unmute;

use serenity::framework::standard::macros::group;

use self::deafen::*;
use self::join::*;
use self::leave::*;
use self::mute::*;
use self::play::*;
use self::undeafen::*;
use self::unmute::*;
#[group]
#[commands(deafen, join, leave, mute, play, undeafen, unmute)]
struct Music;
