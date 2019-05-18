use chrono::{NaiveDate, Utc};
use gnuplot::*;
use serde::Deserialize;

const DOLLAR_URL: &str = "http://infra.datos.gob.ar/catalog/sspm/dataset/168/distribution/168.1/\
                          download/datos-tipo-cambio-usd-futuro-dolar-frecuencia-diaria.csv";

#[derive(Debug, Deserialize)]
struct Record {
    indice_tiempo: NaiveDate,
    tipo_cambio_bna_vendedor: Option<f32>,
    tipo_cambio_a3500: Option<f32>,
}

fn download_usd() -> (Vec<i64>, Vec<f32>) {
    let res = reqwest::Client::new()
        .get(DOLLAR_URL)
        .send()
        .expect("Unable to download csv");
    let past = Utc::today().naive_utc() - chrono::Duration::days(365);
    csv::Reader::from_reader(res)
        .deserialize::<Record>()
        .filter_map(Result::ok)
        .filter(|r| r.indice_tiempo > past)
        .filter_map(|r| {
            r.tipo_cambio_a3500
                .map(|p| (r.indice_tiempo.and_hms(0, 0, 0).timestamp(), p))
        })
        .unzip()
}

fn main() {
    let (xs, ys) = download_usd();

    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("USD/ARS - US Dollar Argentinian Peso", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .set_x_label("Date", &[])
        .set_x_grid(true)
        .set_x_time(true)
        .set_x_ticks(Some((Auto, 1)), &[Format("%b %Y")], &[Rotate(45.0)])
        .set_y_label("Price", &[])
        .set_y_grid(true)
        .lines(&xs, &ys, &[Caption("USD/ARS")]);
    fg.show();
}
