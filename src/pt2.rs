use anyhow::{anyhow, Result};
use ndarray::{Array1, Array2, Axis};
use plotters::{
    backend::SVGBackend,
    chart::ChartBuilder,
    drawing::IntoDrawingArea,
    element::{Cross, EmptyElement},
    series::LineSeries,
    style::{BLUE, GREEN, RED, WHITE},
};

const N: usize = 2;
const ETA: f64 = 1.1;

pub fn main() -> Result<()> {
    crate::pt1::setup_plotters();

    let root = SVGBackend::new("images/part2.svg", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(20)
        .y_label_area_size(50)
        .build_cartesian_2d(0.0..1000.0, 0.0..50.0)?;

    let (x, y) = parse_data()?;

    // for (i, y) in y.iter().enumerate() {
    //     if *y == 1 {
    //         println!("{}", i)
    //     }
    // }

    chart.draw_series(x.axis_iter(Axis(1)).enumerate().filter(|(i, _)| i % 100 == 0).map(|(i, elem)| {
        match y[i] {
            0 => EmptyElement::at((elem[0], elem[1])) + Cross::new((0, 0), 5, BLUE),
            1 => EmptyElement::at((elem[0], elem[1])) + Cross::new((0, 0), 5, RED),
            _ => panic!("DATA MALFORMED: YOU PUT SOME NON-ZERO AND NON-ONE VALUES AS FLAGS INTO THE CSV"),
        }
    }))?;

    let mut w: Array1<f64> = Array1::from_elem(N, 0.0);
    learn_inner(&mut w, &x, &y);
    println!("\n{}\n", w);
    chart.draw_series(LineSeries::new(
        (0..1000).map(|i| (i as f64, -w[0] / w[1] * i as f64)),
        &RED,
    ))?;

    chart.configure_mesh().draw()?;
    chart.configure_series_labels().draw()?;
    root.present()?;
    Ok(())
}

pub fn parse_data() -> Result<(Array2<f64>, Array1<i32>)> {
    let data = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("data/pt2_main.csv")?;
    let records = data.into_records();

    let (x, y) = records
        .flatten()
        .map(|datum| {
            (
                (
                    datum
                        .get(0)
                        .expect("Coudln't get x")
                        .parse::<f64>()
                        .unwrap(),
                    datum
                        .get(1)
                        .expect("Couldn't get y")
                        .parse::<f64>()
                        .unwrap(),
                ),
                datum
                    .get(2)
                    .expect("Coudln't get y")
                    .parse::<i32>()
                    .unwrap(),
            )
        })
        .collect::<((Vec<_>, Vec<_>), Vec<_>)>();
    let len = y.len();

    let mut xcoords = Array2::from_elem((2, len), 0.0);
    let mut ycoords = Array1::from_elem(len, 0);

    for i in 0..len {
        xcoords[(0, i)] = x.0[i];
        xcoords[(1, i)] = x.1[i];
        ycoords[i] = y[i];
    }

    Ok((xcoords, ycoords))
}

pub fn learn_inner(w: &mut Array1<f64>, x: &Array2<f64>, y: &Array1<i32>) {
    let mut res;

    for i in 0..100 {
        for (x, y) in x.axis_iter(ndarray::Axis(1)).zip(y) {
            let x = x.to_owned() / x[0];
            res = i32::from(w.dot(&x) >= 0.0);

            if res == *y {
                continue;
            }

            if i == 80 {
                println!(
                    "w: {:>.2},\tx: {:>.2},\ty: {:>.4},\tres: {:>.4}",
                    w, x, y, res
                );
            }

            w.scaled_add(ETA * (y - res) as f64, &x);
        }
    }
}
