extern crate bwr;

fn main() {
    bwr::compute(&bwr::Options {
        size: 100,
        scale: 10.0,
    });
}
