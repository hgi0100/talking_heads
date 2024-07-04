// Talking_heads
// Author: Neil Crago
// Date: 04/07/2024
//
// A routine to determine the length of an S-Curve using the ARC method.
//

fn s_curve(t: f64, a: f64, b: f64) -> f64 {
    t * t * (3.0 - 2.0 * t) * (a + b * t)
    }

fn do_s_curve(i: usize) -> Vec<(f64, f64, f64)> {
        let mut scvec: Vec<(f64, f64, f64)> = Vec::new();

        for t in 0..=i {
            let s = i as f64;
            let q = t as f64 / s;

            // Example S-curve
            let a = s_curve(q, 1.0, 0.0);

            // Back-loaded S-curve (more emphasis at the end)
            let b = s_curve(q, 0.0, 1.0); // 0.0, 1.0

            // Front-loaded S-curve (more emphasis at the beginning)
            let c = s_curve(q, 2.0, -1.0);

            scvec.push((a, b, c));
        }
        scvec
    }

fn s_curve_derivative(x: f64) -> f64 {
    -4.0 * (x - 2.0) / ((1.0 + (x - 2.0).powf(2.0)).powf(2.0))
}

fn arc_length(svec: Vec<(f64, f64, f64)>, a: f64, b: f64) -> f64 {
    let mut integral = 0.0;
    let k = 100 / b as i32;
    let y = a as i32;
    let z = b as i32 * k;

    //dbg!(z);

    for x in y..z {
        let f = svec[x as usize].0;
        let dx = 1; // Adjust step size as needed
        let dy = (f + dx as f64) / 1000f64 - f / 1000f64;
        integral += (dy * dy + s_curve_derivative((x as f64) + (dx as f64 / 1000f64) / 2.0).powf(2.0)).sqrt();
    }
    integral
}

fn main() {
    // Define S-curve interval and number of segments
    let a = 0.0;
    let b = 4.0;

    let segments = 100;
    let svec = do_s_curve(segments);

    println!();

    let arc_length = arc_length(svec.clone(), a, b);
    println!("Exact length of S-curve using the ARC method: \t{}", arc_length);

    println!();
}
