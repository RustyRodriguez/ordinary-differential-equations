
fn eulers_method<F>(f: F, x_0: f64, y_0: f64, h: f64, n: u32) -> Vec<(f64, f64)> 
where 
    F: Fn(f64, f64) -> f64, 
{
    let mut points:Vec<(f64, f64)> = Vec::new();

    let mut x = x_0;
    let mut y = y_0;

    points.push((x, y));

    for _ in 0..n {
        y += h * f(x, y);
        x += h;
        points.push((x, y));
    }

    points

}

fn main() {
    // Create a Closure for the ODE y' = -2y
    let differential_equation = |x: f64, y: f64| -2.0 * y;

    // Setup intial conidtions, setp size, and number of iterations
    let x_0: f64 = 0.0;
    let y_0: f64 = 1.0;
    let h: f64 = 0.1;
    let n: u32 = 20;

    // Run euler's method
    let solution: Vec<(f64, f64)> = eulers_method(differential_equation, x_0, y_0, h, n);

    // Loop through the list of points and print them out
    for (x, y) in solution {
        println!("x = {:.2}, y = {:.5}", x, y)
    }
}
