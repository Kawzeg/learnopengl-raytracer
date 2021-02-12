use miniquad::*;

struct Stage {
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
        Stage {}
    }
}

impl EventHandler for Stage {
    fn update(&mut self, ctx: &mut Context) {
    }

    fn draw(&mut self, ctx: &mut Context) {
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        UserData::owning(Stage::new(&mut ctx), ctx)
    });
}
