use anyhow::{ensure, Result};
use ndarray::{Array1, Array2};
use ndarray_linalg::Solve;
use plotters::{
    backend::SVGBackend,
    chart::ChartBuilder,
    drawing::IntoDrawingArea,
    series::LineSeries,
    style::{register_font, GREEN, RED, WHITE},
};

const M: usize = 3;

pub fn learn() -> Result<()> {
    setup_plotters();

    let root = SVGBackend::new("images/part1.svg", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(20)
        .y_label_area_size(50)
        .build_cartesian_2d(0.0..1000.0, 0.0..50.0)?;

    let values = parse_csv()?;

    let lhs = make_lhs(M, values.get_x());
    let rhs = make_rhs(M, values.get_x(), values.get_y());

    let result = Solve::solve_into(&lhs, rhs).unwrap();
    println!("{:?}", result);

    chart.draw_series(LineSeries::new(
        (0..1000).map(|x: i32| {
            (
                x as f64,
                result[0]
                    + result[1] * x as f64
                    + result[2] * x.pow(2) as f64
                    + result[3] * x.pow(3) as f64,
            )
        }),
        &GREEN,
    ))?;

    chart.draw_series(LineSeries::new(values, &RED))?;

    chart.configure_mesh().draw()?;
    chart.configure_series_labels().draw()?;

    root.present()?;

    let mut writer = csv::Writer::from_path("data/pt1_results.csv")?;
    writer.write_record(result.iter().map(ToString::to_string))?;

    Ok(())
}

pub fn predict(x: f64) -> Result<()> {
    let reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("data/pt1_results.csv")?;

    let results = reader
        .into_records()
        .flat_map(|record| {
            record.into_iter().flat_map(|val| {
                val.into_iter()
                    .flat_map(|val| val.parse::<f64>())
                    .collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>();

    ensure!(!results.is_empty(), "Results not found or CSV file is bad");

    println!(
        "{}",
        results
            .into_iter()
            .enumerate()
            .map(|(i, result)| x.powi(i as i32) * result)
            .sum::<f64>()
    );

    Ok(())
}

pub fn setup_plotters() {
    register_font(
        "sans-serif",
        plotters::style::FontStyle::Normal,
        include_bytes!("../ComicMono.ttf"),
    )
    .map_err(|_| "Couldn't register font")
    .unwrap();
}

pub fn make_lhs(m: usize, x: Array1<f64>) -> Array2<f64> {
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

pub fn make_rhs(m: usize, x: Array1<f64>, y: Array1<f64>) -> Array1<f64> {
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

pub fn parse_csv() -> Result<Array1<(f64, f64)>> {
    let data = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("data/pt1_main.csv")?;
    let records = data.into_records();

    Ok(records
        .flatten()
        .map(|datum| {
            (
                datum
                    .get(0)
                    .expect("Coudln't get x")
                    .parse::<f64>()
                    .unwrap(),
                datum
                    .get(1)
                    .expect("Coudln't get y")
                    .parse::<f64>()
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>()
        .into())
}

pub trait GetYCoords {
    fn get_y(&self) -> Array1<f64>;
}

impl GetYCoords for Array1<(f64, f64)> {
    fn get_y(&self) -> Array1<f64> {
        self.map(|(_, y)| *y)
    }
}

pub trait GetXCoords {
    fn get_x(&self) -> Array1<f64>;
}

impl GetXCoords for Array1<(f64, f64)> {
    fn get_x(&self) -> Array1<f64> {
        self.map(|(x, _)| *x)
    }
}
