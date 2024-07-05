// Talking_heads
// Author: Neil Crago
// Date: 04/07/2024
//
// A routine to determine the length of an S-Curve using the ARC method.
//
const SEGMENTS: usize = 100;

fn s_curve(t: f64, a: f64, b: f64) -> f64 {
    t * t * (3.0 - 2.0 * t) * (a + b * t)
    }

fn do_s_curve(i: usize) -> Vec<f64> {
        let mut svec: Vec<f64> = Vec::new();

        for t in 0..=i {
            let s = i as f64;
            let q = t as f64 / s;

            // Normal S-curve
            let a = s_curve(q, 1.0, 0.0);

            svec.push(a);
        }
        svec
    }

fn s_curve_derivative(t: f64, a: f64, b: f64) -> f64 {
        6.0 * a * t * (1.0 - t) + 2.0 * b * t.powi(2) * (3.0 - 2.0 * t) - 4.0 * b * t.powi(3) * (1.0 - t)
    }


fn arc_length(svec: Vec<f64>, a: f64, b: f64) -> f64 {
    let mut integral = 0.0;
    let num_segments = svec.len() - 1; // Total number of segments

    for x in 0..num_segments {
        let f = svec[x];
        let dx = 0.001; // Smaller step size
        let dy = (svec[x + 1] - f) / dx;
        integral += (dy * dy + s_curve_derivative(x as f64 / num_segments as f64, a, b).powf(2.0)).sqrt() * dx;
    }

    integral
}

fn main() {
    let segments = SEGMENTS;

    let svec = do_s_curve(segments);

    println!();

    let arc_length0 = arc_length(svec.clone(), 1.0, 0.01);

    println!("Exact length of normal S-curve using the ARC method: \t{}", arc_length0);
}
