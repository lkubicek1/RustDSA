use plotters::prelude::*;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn create_plot(data: &[(usize, u128)], filename: &str, title: &str, label: &str, x_desc: &str, y_desc: &str) -> Result<(), Box<dyn Error>> {
    let output_dir = Path::new("target/benchmarks");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    let filepath = output_dir.join(filename);

    let root = BitMapBackend::new(filepath.to_str().unwrap(), (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0..data.last().unwrap().0, 0..data.iter().map(|x| x.1).max().unwrap())?;

    chart.configure_mesh()
        .x_desc(x_desc)
        .y_desc(y_desc)
        .draw()?;

    chart.draw_series(
        LineSeries::new(data.iter().map(|(x, y)| (*x, *y)), &RED)
    )?
        .label(label)
        .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &RED));

    chart.configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
