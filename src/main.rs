mod engine;
use crate::engine::window::run;

fn main() {
    pollster::block_on(run());
}
