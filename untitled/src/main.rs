use std::{env, error::Error, ffi::OsString, fs::File, process};
use serde::Deserialize;

use ndarray_glm::{array, Linear, ModelBuilder};
use ndarray::{Array2, ArrayBase, OwnedRepr, Ix1, Ix2, Axis, ViewRepr};
use ndarray::Array1;
use ndarray_glm::utility::standardize;

#[derive(Debug, Deserialize)]
struct Record {
    number: i32,
    year: f64,
    catch: f64,
    effort: f64,
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut data_yr:ArrayBase<OwnedRepr<f64>, Ix1> = Default::default();
    let mut data_xr:ArrayBase<OwnedRepr<f64>, Ix2> = Default::default();
    let mut rdr = csv::Reader::from_path(file_path)?;
    {
        let headers = rdr.headers()?;
        println!("{:?}", headers);
    }
    for result in rdr.deserialize() {
        let record:Record = result?;
        // Append year as yx
        let mut data_y = array![record.year];
        let mut data_x = array![record.catch, record.effort];
        data_yr = data_yr.push(Axis(0),data_y);

        println!("{:?}", record);
    }


    let data_x = standardize(data_xr);
    let model = ModelBuilder::<Linear>::data(&data_yr, &data_x).build().unwrap();
    let fit = model.fit_options().l2_reg(1e-5).fit().unwrap();
    println!("Fit result : {}", fit.result);
    Ok(())
}

fn linear_regression() {
    let x = array![[1., 1.], [2., 2.], [3., 3.], [4., 4.], [5., 5.]];
    let y = array![2., 4., 6., 8., 10.];
    let model = ModelBuilder::linear().build().unwrap();
    let model = model.fit(&x, &y).unwrap();
    let y_hat = model.predict(&x).unwrap();
    println!("{:?}", y_hat);
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}