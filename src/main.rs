mod engine;
use crate::engine::app::run;

fn main() {
    pollster::block_on(run());
}
