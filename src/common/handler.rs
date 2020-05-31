use serenity::{
    model::gateway::Ready,
    prelude::{Context, EventHandler},
};

#[derive(Default)]
pub struct ScrabsHandler;

impl EventHandler for ScrabsHandler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
