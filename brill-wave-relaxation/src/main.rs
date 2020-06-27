extern crate bwr;

fn main() {
    bwr::compute(&bwr::Options {
        size: 64,
        scale: 8.0,
    });
}
