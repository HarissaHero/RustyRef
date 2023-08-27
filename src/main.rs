use ui::run;

pub mod reference;
pub mod renderer;
pub mod ui;

fn main() {
    env_logger::init();

    pollster::block_on(run());
}
