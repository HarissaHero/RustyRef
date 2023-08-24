use ui::run;

pub mod ui;
pub mod renderer;

fn main() {
    pollster::block_on(run());
}

