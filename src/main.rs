use anyhow::Result;
use ndarray::{Array1, Array2, Ix};
use ndarray_linalg::Solve;
use plotters::style::register_font;

const M: usize = 2;

fn main() -> Result<()> {
    println!("Hello, world!");
    register_font(
        "sans-serif",
        plotters::style::FontStyle::Normal,
        include_bytes!("../ComicMono.ttf"),
    )
    .map_err(|_| "Couldn't register font")
    .unwrap();

    let data = csv::Reader::from_path("data/main.csv")?;
    let records = data.into_records();

    let values: Array1<(f64, f64)> = records
        .flatten()
        .map(|datum| {
            (
                datum.get(0).unwrap().parse::<f64>().unwrap(),
                datum.get(1).unwrap().parse::<f64>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
        .into();

    println!("{:?}", values);

    let lhs: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[Ix; 2]>> =
        make_lhs(M, values.map(|(x, _)| *x));
    println!("{:?}", lhs);

    let rhs = make_rhs(M, values.map(|(x, _)| *x), values.map(|(_, y)| *y));
    println!("{:?}", rhs);

    let result = Solve::solve_into(&lhs, rhs).unwrap();
    println!("{:?}", result);

    Ok(())
}

fn make_lhs(m: usize, x: Array1<f64>) -> Array2<f64> {
    let mut lhs = Array2::from_elem((m + 1, m + 1), 0.0);

    {
        let i = 0;
        for j in 0..m + 1 {
            lhs[(i, j)] = x.map(|x| x.powi((i + j) as i32)).sum();
        }
    }

    for i in 1..m + 1 {
        for j in 0..m + 1 {
            lhs[(i, j)] = x.map(|x| x.powi((i + j) as i32)).sum();
        }
    }

    lhs
}

fn make_rhs(m: usize, x: Array1<f64>, y: Array1<f64>) -> Array1<f64> {
    let mut rhs = Array1::from_elem(m + 1, 0.0);

    for i in 0..m + 1 {
        rhs[i] = y
            .iter()
            .zip(x.map(|x| x.powi(i as i32)))
            .map(|(y, x)| x * y)
            .sum();
    }

    rhs
}
