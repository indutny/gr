extern crate bwr;

fn main() {
    let size = 64;
    let scale = 16.0;
    let steps = 2000;

    println!("a, mass, error");
    let a_start = 0.1;
    let a_step = 0.1;
    for i in 0..149 {
        let a = a_start + a_step * (i as f64);
        let res = bwr::compute(&bwr::Options {
            a,
            steps,

            size,
            scale,
        });

        println!("{:?}, {:?}, {:?}", a, res.mass, res.error);
    }
}
