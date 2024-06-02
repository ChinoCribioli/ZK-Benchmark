use sp1_helper::build_program;

fn main() {
    build_program("../program/components");
    build_program("../program/snafu");
    build_program("../program/beacons");
}
