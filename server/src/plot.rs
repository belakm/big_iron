use std::vec;

use crate::database::DATABASE_CONNECTION;
use crate::portfolio::get_account_history_with_snapshots;
use crate::server_error::{ServerError, MAP_TO_404, MAP_TO_500};
use plotters::prelude::*;
use rocket::fs::NamedFile;
use rocket::get;

static PATH: &str = "static/account_balance.svg";

async fn plot_account_balance() -> Result<(), ServerError> {
    let account_balance_history =
        get_account_history_with_snapshots(&DATABASE_CONNECTION.clone().lock().unwrap());

    match account_balance_history {
        Ok(balance) => {
            println!("{:?}", balance);
            let drawing_area = SVGBackend::new(&PATH, (1024, 768)).into_drawing_area();
            // let history_data = vec![balance];
            // drawing_area.fill(&WHITE);

            // Chart metadata
            let mut chart = ChartBuilder::on(&drawing_area)
                .caption("y=x^2", ("Arial", 50).into_font())
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
                .unwrap();

            chart.configure_mesh().draw().unwrap();

            chart
                .draw_series(LineSeries::new(
                    (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                    &RED,
                ))
                .map_err(|e| MAP_TO_500(&e.to_string()))?
                .label("y = x^2")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

            chart
                .configure_series_labels()
                .background_style(&WHITE.mix(0.8))
                .border_style(&BLACK)
                .draw()
                .map_err(|e| MAP_TO_500(&e.to_string()))
        }
        Err(error) => Err(MAP_TO_500(&error.to_string())),
    }
}

#[get("/plot/account_balance_history")]
pub async fn account_balance_history() -> Result<NamedFile, ServerError> {
    let plot = plot_account_balance().await;
    match plot {
        Ok(_) => NamedFile::open(&PATH)
            .await
            .map_err(|e| MAP_TO_404(&e.to_string())),
        Err(err) => Err(err),
    }
}
