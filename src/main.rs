use std::error::Error;
use chrono::NaiveDate;
use serde::Deserialize;
use tensorflow::Tensor;

static DOLLAR_URL: &str = "http://infra.datos.gob.ar/catalog/sspm/dataset/168/distribution/168.1/download/datos-tipo-cambio-usd-futuro-dolar-frecuencia-diaria.csv";

#[derive(Debug, Deserialize)]
struct Record {
    indice_tiempo: NaiveDate,
    tipo_cambio_bna_vendedor: Option<f32>,
    tipo_cambio_a3500: Option<f32>,
    tipo_cambio_mae: Option<f32>,
    volumen_mae: Option<f32>,
    tipo_cambio_implicito_en_adrs: Option<f32>,
    futuro_rofex_usd1m: Option<f32>,
    interes_abierto_1m: Option<f32>,
    futuro_rofex_usd2m: Option<f32>,
    interes_abierto_2m: Option<f32>,
    futuro_rofex_usd3m: Option<f32>,
    interes_abierto_3m: Option<f32>,
    futuro_rofex_usd4m: Option<f32>,
    interes_abierto_4m: Option<f32>,
    futuro_rofex_usd5m: Option<f32>,
    interes_abierto_5m: Option<f32>,
    futuro_rofex_usd6m: Option<f32>,
    interes_abierto_6m: Option<f32>,
    futuro_rofex_usd_12m: Option<f32>,
    interes_abierto_12m: Option<f32>,
}

/// split a univariate sequence into samples
fn split_sequence(sequence: Vec<f32>, n_steps: usize) -> (Vec<Vec<f32>>, Vec<f32>){
    let mut x = Vec::new();
    let mut y = Vec::new();

    for i in 0..sequence.len() {
        // find the end of this pattern
        let end_ix = i + n_steps;

        // check if we are beyond the sequence
        if end_ix > sequence.len() -1 {
            break;
        }

        // gather input and output parts of the pattern
        let seq_x = sequence[i..end_ix].to_vec();
        let seq_y = sequence[end_ix];
        x.push(seq_x);
        y.push(seq_y);
    }
    (x, y)
}

fn download_usd() -> Vec<f32> {

    let client = reqwest::Client::new();
    let res = client.get(DOLLAR_URL).send().expect("Unable to download csv");

    csv::Reader::from_reader(res)
        .deserialize::<Record>()
        .map(|r|
            r
                .expect("Unable to deserialize record")
                .tipo_cambio_a3500
                .expect("Field not found"))
        .collect()
}

fn main() {
    // choose a number of time steps
    let n_steps = 4;

    let mut historic_data = download_usd();
    let len = historic_data.len() - 20;
    historic_data.split_off(len);

    // split into samples
    let (x, y) = split_sequence(historic_data, n_steps);
    println!("secuencias x {:?}", x);
}
