extern crate image;
extern crate imageproc;
extern crate rand;
extern crate palette;
extern crate rayon;
extern crate csv;

use image::{Luma, ImageBuffer, RgbImage};
use imageproc::drawing::{draw_line_segment_mut};
use rand::distributions::{IndependentSample, Normal};
use palette::{Lab, LinSrgb, Gradient};
use rayon::prelude::*;
use std::time::Instant;
use std::env;
use csv::ReaderBuilder;
use std::{fs, io};

type Image = ImageBuffer<Luma<f64>, Vec<f64>>;

fn run_series(series: &[u32], width: u32, height: u32, k: u32) -> Image {
    // initialize new image
    let mut data = Image::new(width, height);
    // println!("length:{}", series.len());

    for x in 0..width*k-1 {
    // -1 because draw line connecting two points
    // simulated data t-v and chart data x-y are the same scale, i.e., x in [0,width), y in [0,height]
        draw_line_segment_mut(
            &mut data,
            (x as f32 / k as f32, series[x as usize] as f32),
            ((x as f32 + 1.0) / k as f32, series[x as usize + 1] as f32),
            Luma([1.0]),
        );
        // println!("({},{}),({},{}),",x as f32 / k as f32,series[x as usize],(x as f32 + 1.0) / k as f32,series[x as usize + 1]);
    }

    // normalize
    for x in 0..width {
        let mut sum = 0.0;
        for y in 0..height {
            sum += data.get_pixel(x,y)[0];
        }
        for y in 0..height {
            let value = data.get_pixel(x,y)[0];
            data.put_pixel(x,y,Luma([value / sum]));
        }
    }

    data
}

/// Reducer that combines counts from two time series.
fn sum_images(image: Image, mut aggregated: Image) -> Image {
    for (x,y,value) in image.enumerate_pixels() {
        let new_value = aggregated.get_pixel(x,y)[0] + value[0];
        aggregated.put_pixel(x,y,Luma([new_value]))
    }

    aggregated
}

fn get_files_in_directory(path: &str) -> io::Result<Vec<String>> {
    // Get a list of all entries in the folder
    let entries = fs::read_dir(path)?;

    // Extract the filenames from the directory entries and store them in a vector
    let file_names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();

    Ok(file_names)
}

fn main() {
    // arguments: width,height,csv_path,has_header,tqs,tqe
    let mut width:f64 = 400.0;
    let mut height:f64 = 300.0;
    let mut csv_path = String::from("None"); // "ts-{}-{}.csv".format(input,approach,w)
    let mut has_header = true;
    let mut tqe:f64 = 0.0;
    let mut tqs:f64 = 4259092178974.0; // adapt based on width later

    // parse command line argument
    let args: Vec<_> = env::args().collect();
    if args.len() >= 6 {
            width = match args[1].parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    println!("error: argument width");
                    return;
                },
            };
            height = match args[2].parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    println!("error: argument height");
                    return;
                },
            };
            csv_path = match args[3].parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    println!("error: argument csv_path");
                    return;
                },
            };
            has_header = match args[4].parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    println!("error: argument has_header");
                    return;
                },
            };
            tqs = match args[5].parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    println!("error: tqs");
                    return;
                },
            };
            tqe = match args[6].parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    println!("error: tqe");
                    return;
                },
            };
    }
    else {
            println!("error arguments: width,height,csv_path,has_header,tqs,tqe");
            return;
    }


     // arguments: width,height,csv_path,has_header,tqs,tqe
    println!("width: {}, height: {}", width, height);
    println!("csv_path: {}", csv_path);
    println!("has_header: {}", has_header);
    println!("tqs: {}", has_header);
    println!("tqe: {}", has_header);

    // t_max=math.ceil((t_max_temp-t_min)/(2*width))*2*width+t_min
    let f = (tqe-tqs)/(2.0*width);
    tqe = f.ceil()*2.0*width+tqs;
    println!("adapted tqe: {}", tqe);

    println!("=============================================");

    // read csv
    let mut data: Vec<Vec<f64>> = Vec::new(); // the first vector being t, the second vector being v
    let mut global_min: f64 = f64::MAX; // for scale value to [0,height]. (v-global_min)/(global_max-global_min)*height
    let mut global_max: f64 = f64::MIN; // for scale value to [0,height]. (v-global_min)/(global_max-global_min)*height
    let f = csv_path;
    let mut res_t: Vec<f64> = Vec::new(); // t
    let mut res_v: Vec<f64> = Vec::new(); // v
    let reader_result = ReaderBuilder::new().has_headers(has_header).from_path(&f);
    let reader = match reader_result {
        Ok(reader) => reader,
        Err(_) => { println!("error match reader_result"); return; },
    };
    for record in reader.into_records() {
        let record = match record {
            Ok(record) => record,
            Err(_) => { println!("error match record"); return; },
        };

        let row: Vec<String> = record
                .iter()
                .map(|field| field.trim().to_string())
                .collect();

        if row.len()<2 {
                println!("error: the file f has less than 2 columns");
                return;
        }
        // println!("{:?}", row);

        // parse string into double value and then value as f64
        let t = row[0].parse::<f64>().unwrap();
        let v = row[1].parse::<f64>().unwrap();
        res_t.push(t);
        res_v.push(v);

        if v > global_max {
            global_max = v;
        }
        if v < global_min {
            global_min = v;
        }
    } // end read

    // scale v: (v-global_min)/(global_max-global_min)*height
    let mut res_t_new: Vec<f64> = Vec::new();
    let mut res_v_new: Vec<f64> = Vec::new();
    for j in 0..res_v.len() {
        let t: f64 = (res_t[j as usize]-tqs)/(tqe-tqs)* width;
        res_t_new.push(t as f64);

        let v: f64 = (res_v[j as usize]-global_min)/(global_max-global_min)* height;
        res_v_new.push(v as f64);
    }
    data.push(res_t_new);
    data.push(res_v_new);

}
