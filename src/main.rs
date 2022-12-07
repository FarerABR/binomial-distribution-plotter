use plotters::prelude::*;
use std::env;

/**
## config for IO arguments

```
param: [String]

returns: miu:i32, vrc:i32, perc:i32, dom:i32
```
*/
fn config(args: &[String]) -> (i32, i32) {
    if args.len() < 2 {
        panic!("not enough arguments");
    }

    (
        args[1].parse::<i32>().unwrap(), // n
        args[2].parse::<i32>().unwrap(), // k
    )
}

/**
## function for finding the comb of two big number
due to hardware limitation we can't calculate comb of 2 big number directly
```
param: n:i128 , k:i128
returns: i128
```
*/
fn big_comb(mut n: i128, k: i128) -> i128 {
    if k > n / 2 {
        return big_comb(n, n - k);
    }
    let mut r = 1;
    for i in 1..=k {
        r *= n;
        r /= i;
        n = n - 1;
    }
    r
}

/**
## bimnomial distribution function
```
param: n:i32 , k:i32 , p:f32
returns: f32
```
 */
fn binomial(n: i32, k: i32, p: f32) -> f32 {
    big_comb(n as i128, k as i128) as f32 * p.powi(k) as f32 * (1.0 - p).powi(n - k) as f32
}
fn draw(n: i32, k: i32) {
    let mut y: Vec<(f32, f32)> = Vec::new();
    let p: f32 = k as f32 / n as f32;
    for i in 0..=n {
        y.push((i as f32, binomial(n, i, p)));
    }

    let drawing_area = BitMapBackend::new("./figure.png", (1920, 1080)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(
            format!("binomial distribution - p({}, {})", n, k),
            ("mono", 40),
        )
        .build_cartesian_2d(0.0..n as f32 + 1.0, 0.0..y[10].1 + 0.01)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(
            y.iter().map(|&point| {
                TriangleMarker::new(point, 5, Into::<ShapeStyle>::into(&RED).filled())
            }),
        )
        .unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (n, k) = config(&args);
    draw(n, k);
}
