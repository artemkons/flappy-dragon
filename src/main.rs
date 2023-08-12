use bracket_lib::prelude::*;

mod game;
use game::State;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy dragon")
        .build()?;

    main_loop(context, State::new())
}