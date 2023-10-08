extern crate image;
extern crate imageproc;
extern crate rand;
extern crate palette;
extern crate rayon;

use image::{Luma, ImageBuffer, RgbImage};
use imageproc::drawing::{draw_line_segment_mut};
use rand::distributions::{IndependentSample, Normal};
use palette::{Lab, LinSrgb, Gradient};
use rayon::prelude::*;
use std::time::Instant;
use std::env;

type Image = ImageBuffer<Luma<f32>, Vec<f32>>;

fn run_series(series: &[u32], width: u32, height: u32, k: u32, downsampling: bool) -> Image {
    // initialize new image
    let mut data = Image::new(width, height);
    // println!("length:{}", series.len());

    // draw the time series as a line
    if downsampling {
      // first, last, small, large
      for i in 0..width*4-1 { // M4 downsampling
      // -1 because draw line connecting two points
      // simulated data t-v and chart data x-y are the same scale, i.e., x in [0,width), y in [0,height]
          if i % 4 == 3 {
          // the last point in a column, need align, because 3/4 != 9/10,
          // but first point 4/4=10/10, and TP&BP's t do not matter as long as they are inside the same column,
          // so only last point needs alignment
              let j = (i / 4 + 1 ) * k - 1; // e.g., k=10, i=3, j=9
              let x = j as f32 / k as f32; // e.g., x=9/10 rather than 3/4
              draw_line_segment_mut(
                 &mut data,
                 (x, series[i as usize] as f32),
                 ((i as f32 +1.0)/4.0, series[i as usize + 1]  as f32),
                 Luma([1.0]),
              );
              // println!("({},{}),({},{}),",x,series[i as usize],(i as f32 +1.0)/4.0,series[i as usize + 1]);
              // https://docs.rs/imageproc/latest/imageproc/drawing/fn.draw_line_segment_mut.html
              // Uses Bresenham’s line drawing algorithm.
          }
          else {
          // first point 4/4=10/10, and TP&BP's t do not matter as long as they are inside the same column
              draw_line_segment_mut(
                  &mut data,
                  (i as f32 / 4.0, series[i as usize] as f32),
                  ((i as f32 +1.0)/4.0, series[i as usize + 1]  as f32),
                  Luma([1.0]),
              );
              // println!("({},{}),({},{}),",i as f32 / 4.0,series[i as usize],(i as f32 +1.0)/4.0,series[i as usize + 1]);
          }
      }
    }
    else {
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

fn main() {
    // arguments: iterations,k,width,height,use_external_data,csv_dir_path
    // 100,10,400,300,false
    // 2,10,400,300,true,"/home/data"
    let mut iterations = 100; // number of time series
    let mut k = 4; // regular point count = width*k
    let mut width = 400;
    let height; // if not set, default = width
    let mut use_external_data = false;
    let mut csv_dir_path = String::from("None");

    // parse command line argument
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
            iterations = match args[1].parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    println!("error: argument not an integer");
                    return;
                },
            };
    }
    if args.len() > 2 {
        k = match args[2].parse() {
            Ok(n) => {
                n
            },
            Err(_) => {
                println!("error: argument not an integer");
                return;
            },
        };
    }
    if args.len() > 3 {
        width = match args[3].parse() {
            Ok(n) => {
                n
            },
            Err(_) => {
                println!("error: argument not an integer");
                return;
            },
        };
    }
    if args.len() > 4 {
        height = match args[4].parse() {
            Ok(n) => {
                n
            },
            Err(_) => {
                println!("error: argument not an integer");
                return;
            },
        };
    }
    else {
        height = width;
    }
    if args.len() > 5 {
        use_external_data = match args[5].parse() {
            Ok(n) => {
                n
            },
            Err(_) => {
                println!("error: argument not a bool");
                return;
            },
        };
    }
    if use_external_data {
        if args.len() > 6 {
            csv_dir_path = match args[6].parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    println!("error: argument not a string");
                    return;
                },
            };
        }
        else {
            println!("error: missing csv_dir_path");
            return;
        }
    }

    // arguments: iterations,k,width,height,use_external_data,csv_dir_path
    println!("number of time series: {}", iterations);
    println!("number of points in a time series: {}", width*k);
    println!("width: {}, height: {}", width, height);
    println!("use_external_data: {}", use_external_data);
    println!("csv_dir_path: {}", csv_dir_path);
    println!("=============================================", csv_dir_path);

    let data;
    if !use_external_data {
        // create sine wave as a model
        let model: Vec<f32> = (0..width*k).map(|x| { // note that x is regular
            let heightf = height as f32;
            let xf = x as f32 / k as f32;
            let y = heightf/4.0 * (xf/20.0).sin() + heightf/2.0;
            y
        }).collect();

        data: Vec<Vec<u32>> = (0..iterations).map(|_| {
            // add some noise
            let normal = Normal::new(0.0, 12.0); // mean 0, standard deviation 12
            let mut rng = rand::thread_rng();
            // Each thread has an initialized generator.
            // Integers are uniformly distributed over the range of the type,
            // and floating point numbers are uniformly distributed from 0 up to but not including 1.

            model.iter().map(|v| {
                let value = v + normal.ind_sample(& mut rng) as f32;
                if value < 0.0 {
                    0u32
                } else if value > height as f32 {
                    height
                } else {
                    value as u32 // 这里又还原回整数了？
                }
            }).collect()
        }).collect();
    }
    else {
        // TODO read iterations csv files from csv_dir_path, for each csv read the first width*k points

    }

    // M4 downsampling
    // data -> downsampled_data
    let w = width;
    // let w:u32 = 2; // the number of pixel columns should = width
    let downsamples: Vec<Vec<u32>> = data.iter().map(|series| { // for one series
         let mut res: Vec<u32> = Vec::new();
         for i in 0..w {
         // (0..w).map(|i| {
             // println!("{}", x as f32/k as f32);
             let start = series.len()/w as usize * i as usize;
             let end = series.len()/w as usize * (i+1) as usize;
             let mut large: u32 = 0; // note value range [0,height]
             let mut small: u32 = height+1; // note value range [0,height]
             for j in start..end {
                  if large < series[j] {
                      large = series[j]
                  }
                  if small > series[j] {
                      small = series[j]
                  }
                  // print!("{},",series[j]);
             }
             // println!("");
             let first = series[start];
             let last = series[end-1];
             // println!("first={},last={},small={},large={}",first,last,small,large);
             res.push(first);
             res.push(small);
             res.push(large);
             res.push(last); // note this is the last
         }
         //}).collect()
         res
    }).collect();
    //for row in tmp.iter() {
        //for pixel in row.iter() {
            //println!("{:#?}", pixel);
        //}
    //}

    // color scale to convert from value to a color
    let color_scale = Gradient::new(vec![
        Lab::from(LinSrgb::new_u8(247, 252, 241)),
        Lab::from(LinSrgb::new_u8(14, 66, 127))
    ]);

    // ------------------------- test original -------------------------
    let mut downsampling = false;
    let mut input_data: Vec<Vec<u32>> = data;
    let mut now = Instant::now();
    let mut aggregated = input_data
        .par_iter()
        .map(|series| {
            run_series(&series, width as u32, height as u32, k as u32, downsampling)
        })
        .reduce(|| Image::new(width as u32, height as u32), sum_images);

    println!("Downsampling:{}, Computing line density took {}ms", downsampling, now.elapsed().as_millis());

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

    img.save(format!("output-{}-{}-{}-{}-{}.png", iterations, k, width, height, downsampling)).unwrap();

    // ------------------------- test downsampled -------------------------
    downsampling = true;
    input_data = downsamples;
    now = Instant::now();
    aggregated = input_data
        .par_iter()
        .map(|series| {
            run_series(&series, width as u32, height as u32, k as u32, downsampling)
        })
        .reduce(|| Image::new(width as u32, height as u32), sum_images);

    println!("Downsampling:{}, Computing line density took {}ms", downsampling, now.elapsed().as_millis());

    img = RgbImage::new(width as u32, height as u32);

    // find the maximum value so that we can scale colors
    max_value = aggregated.pixels().fold(
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

    img.save(format!("output-{}-{}-{}-{}-{}.png", iterations, k, width, height, downsampling)).unwrap();
}
