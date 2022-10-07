#[warn(clippy::pedantic)]

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const _SCREEN_WIDTH: i32 = 80;
    pub const _SCREEN_HEIGHT: i32 = 50;
    pub const _MAPSIZE: i32 = _SCREEN_HEIGHT * _SCREEN_WIDTH;
}

use prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello Nihn");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Feathers and Glass")
        .build()?;
    let gs = State {};
    main_loop(context, gs)
}
