mod field;

use field::*;

pub struct Options {
    pub size: usize,
    pub scale: f64,
}

fn compute_ddq(rho: f64, z: f64) -> f64 {
    let a: f64 = 1.0;
    let r = f64::sqrt(f64::powi(rho, 2) + f64::powi(z, 2));
    let num: f64 = 5.0 * f64::powi(rho, 4) * f64::powi(r, 6) -
        45.0 * f64::powi(rho, 4) * r +
        5.0 * f64::powi(rho, 2) * f64::powi(r, 6) * f64::powi(z, 2) -
        45.0 * f64::powi(rho, 2) * f64::powi(z, 2) * r +
        2.0 * f64::powi(r, 10) +
        4.0 * f64::powi(r, 5) +
        2.0;
    let den: f64 = 3.0 * f64::powi(r, 10) +
        f64::powi(r, 15) +
        3.0 * f64::powi(r, 5) +
        1.0;

    return a * num / den;
}

fn step(options: &Options, past: &Field, future: &mut Field) -> f64 {
    let ds = options.scale / (options.size as f64);
    let ds2 = f64::powi(ds, 2);

    let mut error: f64 = 0.0;
    for x in 0..(options.size - 1) {
        for y in 0..(options.size - 1) {
            for z in 0..(options.size - 1) {
                let coord_rho = (
                    (x as f64).powi(2) +
                    (y as f64).powi(2)).sqrt() * ds;
                let coord_z = (z as f64) * ds;

                let lx = match x { 0 => 1, _ => x - 1 };
                let ly = match y { 0 => 1, _ => y - 1 };
                let lz = match z { 0 => 1, _ => z - 1 };
                let rx = x + 1;
                let ry = y + 1;
                let rz = z + 1;

                let ddq = compute_ddq(coord_rho, coord_z);
                let laplacian = -ddq * past[(x, y, z)] / 8.0;

                let new_value = 1.0 / 6.0 * (
                    past[(lx, y, z)] + past[(rx, y, z)] +
                    past[(x, ly, z)] + past[(x, ry, z)] +
                    past[(x, y, lz)] + past[(x, y, rz)] -
                    ds2 * laplacian);

                error += (past[(x, y, z)] - new_value).abs();

                future[(x, y, z)] = new_value;
            }
        }
    }

    return error / ((options.size - 1) as f64).powi(3);
}

fn compute_mass(options: &Options, field: &Field) -> f64 {
    let ds = options.scale / (options.size as f64);
    let ds3 = f64::powi(ds, 3);

    let boundary = (options.size - 2) as isize;

    fn constrain(coord: isize) -> usize {
        if coord < 0 {
            (-coord) as usize
        } else {
            coord as usize
        }
    }

    let mut m: f64 = 0.0;
    for x in -boundary..=boundary {
        for y in -boundary..=boundary {
            for z in -boundary..=boundary {
                let cx = constrain(x);
                let cy = constrain(y);
                let cz = constrain(z);
                let rx = constrain(x + 1);
                let ry = constrain(y + 1);
                let rz = constrain(z + 1);
                let lx = constrain(x - 1);
                let ly = constrain(y - 1);
                let lz = constrain(z - 1);

                let dpsix = (field[(rx, cy, cz)] - field[(lx, cy, cz)]) /
                    (2.0 * ds);
                let dpsiy = (field[(cx, ry, cz)] - field[(cx, ly, cz)]) /
                    (2.0 * ds);
                let dpsiz = (field[(cx, cy, rz)] - field[(cx, cy, lz)]) /
                    (2.0 * ds);

                let dpsi2 = dpsix.powi(2) + dpsiy.powi(2) + dpsiz.powi(2);

                m += dpsi2 / field[(cx, cy, cz)].powi(2) * ds3;
            }
        }
    }

    return m / (2.0 * std::f64::consts::PI);
}

pub fn compute(options: &Options) {
    let mut field_a = Field::new(options.size);
    let mut field_b = Field::new(options.size);

    for i in 0..options.size {
        field_a[(i, options.size - 1, options.size - 1)] = 1.0;
        field_a[(options.size - 1, i, options.size - 1)] = 1.0;
        field_a[(options.size - 1, options.size - 1, i)] = 1.0;

        field_b[(i, options.size - 1, options.size - 1)] = 1.0;
        field_b[(options.size - 1, i, options.size - 1)] = 1.0;
        field_b[(options.size - 1, options.size - 1, i)] = 1.0;
    }

    for s in 0..10000000 {
        step(options, &field_a, &mut field_b);
        let error = step(options, &field_b, &mut field_a);

        println!("s={:?} error={:?} mass={:?}",
            s, error, compute_mass(options, &field_a));
    }
}

