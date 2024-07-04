// Talking_heads
// Author: Neil Crago
// Date: 04/07/2024
//
// Two routines to determine the length of an S-Curve for comparison purposes
//
// 1.) The approximate length using Simpson's rule.
// 2.) The exact length using the ARC method.
//

fn s_curve_function(x: f64) -> f64 {
    1.0 / (1.0 + (x - 2.0).powf(2.0))
}

fn s_curve_derivative(x: f64) -> f64 {
    // Replace this with the derivative of your S-curve function
    -4.0 * (x - 2.0) / ((1.0 + (x - 2.0).powf(2.0)).powf(2.0))
}

fn simpson_integrate(f: fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / (n as f64);
    let mut sum = f(a) + f(b);
    for i in 1..n {
        let x = a + h * (i as f64);
        if i % 2 == 0 {
            sum += 2.0 * f(x);
        } else {
            sum += 4.0 * f(x);
        }
    }
    (h / 3.0) * sum
}

fn arc_length(f: fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let mut integral = 0.0;
    let y = (a * 1000f64) as i32;
    let z = (b * 1000f64) as i32;

    for x in y..z {
        let dx = 1; // Adjust step size as needed
        let dy = f((x + dx) as f64)/ 1000f64 - f((x as f64) / 1000f64);
        integral += (dy * dy + s_curve_derivative((x as f64) + (dx as f64 / 1000f64) / 2.0).powf(2.0)).sqrt();
    }
    integral
}

fn main() {
    // Define S-curve interval and number of segments
    let a = 0.0;
    let b = 4.0;
    let segments = 100;

    // Calculate approximate length with Simpson's rule
    let sim_length = simpson_integrate(s_curve_function, a, b, segments);
    let arc_length = arc_length(s_curve_function, a, b) / 1000f64;

    println!("Approximate length of S-curve using Simpson: \t{}", sim_length);
    println!("Exact length of S-curve using the ARC method: \t{}", arc_length);

    println!();
}
