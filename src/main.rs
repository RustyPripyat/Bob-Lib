use robotics_lib::runner::Robot;
use crate::bob::BobMap;

mod bob;

fn main() {
    struct MyRobot(Robot);
    let map = BobMap::init();
}
