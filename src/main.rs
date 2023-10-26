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

type Image = ImageBuffer<Luma<f32>, Vec<f32>>;

fn run_series(series_t: &[f32], series_v: &[f32], width: u32, height: u32) -> Image {
    // initialize new image
    let mut img_data = Image::new(width, height);
    // println!("length:{}", series.len());

    for i in 0..series_t.len()-1 {
    // -1 because draw line connecting two points
    // simulated data t-v and chart data x-y are the same scale, i.e., x in [0,width), y in [0,height]
        draw_line_segment_mut(
            &mut img_data,
            (series_t[i as usize] as f32, series_v[i as usize] as f32),
            (series_t[i as usize + 1] as f32, series_v[i as usize + 1] as f32),
            Luma([1.0]),
        );
    }

    // normalize
    for x in 0..width {
        let mut sum = 0.0;
        for y in 0..height {
            sum += img_data.get_pixel(x,y)[0];
        }
        for y in 0..height {
            let value = img_data.get_pixel(x,y)[0];
            img_data.put_pixel(x,y,Luma([value / sum]));
        }
    }

    img_data
}

/// Reducer that combines counts from two time series.
fn sum_images(image: Image, mut aggregated: Image) -> Image {
    for (x,y,value) in image.enumerate_pixels() {
        let new_value = aggregated.get_pixel(x,y)[0] + value[0];
        aggregated.put_pixel(x,y,Luma([new_value]))
    }

    aggregated
}

fn main() {
    // arguments: width,height,csv_path,has_header
    let mut width:f32 = 400.0;
    let mut height:f32 = 300.0;
    let mut csv_path = String::from("None"); // "ts-{}-{}.csv".format(input,approach,w)
    let mut has_header = true;

    // parse command line argument
    let args: Vec<_> = env::args().collect();
    if args.len() >= 4 {
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
    }
    else {
            println!("error arguments: width,height,csv_path,has_header");
            return;
    }


     // arguments: width,height,csv_path,has_header
    println!("width: {}, height: {}", width, height);
    println!("csv_path: {}", csv_path);
    println!("has_header: {}", has_header);

    println!("=============================================");

    // read csv
    let mut data: Vec<Vec<f32>> = Vec::new(); // the first vector being t, the second vector being v
    let f = &csv_path;
    let mut res_t_new: Vec<f32> = Vec::new(); // t
    let mut res_v_new: Vec<f32> = Vec::new(); // v
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

        // parse string into double value and then value as f32
        let t = row[0].parse::<f32>().unwrap();
        let v = row[1].parse::<f32>().unwrap();
        res_t_new.push(t);
        res_v_new.push(v);
    } // end read

    data.push(res_t_new);
    data.push(res_v_new);

    // --------------------
    // color scale to convert from value to a color
    // binary color here
    let color_scale = Gradient::new(vec![
        Lab::from(LinSrgb::new_u8(0,0,0)),
        Lab::from(LinSrgb::new_u8(0,0,0))
    ]);

    // ------------------------- plot -------------------------
    let mut aggregated = run_series(&data[0], &data[1], width as u32, height as u32);

    let mut img = RgbImage::new(width as u32, height as u32);

    // find the maximum value so that we can scale colors
    let mut max_value = aggregated.pixels().fold(
        0.0,
        |max,pixel| f32::max(max, pixel[0])
    );

    // create output image
    for (x, y, pixel) in aggregated.enumerate_pixels() {
        let value = pixel[0];
        if value == 0.0 {
            img.put_pixel(x,y,image::Rgb([255,255,255]));
        } else {
            let color = LinSrgb::from(color_scale.get(value / max_value));
            let converted_color = image::Rgb([
                (color.red * 255.0).round() as u8,
                (color.green * 255.0).round() as u8,
                (color.blue * 255.0).round() as u8]
            );

            img.put_pixel(x,y,converted_color);
        }
    }

    // ts-lttb-600.csv-600.png
    img.save(format!("{}-{}.png", csv_path, width)).unwrap();
}
