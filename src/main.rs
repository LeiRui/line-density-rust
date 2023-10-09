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
    // arguments: iterations,k,width,height,use_external_data,csv_dir_path,has_header
    // synthetic: 100,10,400,300
    // real: 2,10,400,300,true,"/home/data",true
    let mut iterations = 100; // number of time series
    let mut k = 4; // regular point count = width*k
    let mut width = 400;
    let height; // if not set, default = width
    let mut use_external_data = false;
    let mut csv_dir_path = String::from("None");
    let mut has_header = false;

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
        if args.len() > 7 {
            csv_dir_path = match args[6].parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    println!("error: argument not a string");
                    return;
                },
            };
            has_header = match args[7].parse() {
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
            println!("error: missing csv_dir_path, has_header");
            return;
        }
    }

    // arguments: iterations,k,width,height,use_external_data,csv_dir_path,has_header
    println!("number of time series: {}", iterations);
    println!("number of points in a time series: {}", width*k);
    println!("width: {}, height: {}", width, height);
    println!("use_external_data: {}", use_external_data);
    println!("csv_dir_path: {}", csv_dir_path);
    println!("has_header: {}", has_header);
    println!("=============================================");

    let mut data:Vec<Vec<u32>>;
    if !use_external_data {
        // create sine wave as a model
        let model: Vec<f32> = (0..width*k).map(|x| { // note that x is regular
            let heightf = height as f32;
            let xf = x as f32 / k as f32;
            let y = heightf/4.0 * (xf/20.0).sin() + heightf/2.0;
            y
        }).collect();

        data = (0..iterations).map(|_| {
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
                    value as u32 // note
                }
            }).collect()
        }).collect();
    }
    else {
        // read iterations csv files from csv_dir_path,
        // for each csv read the first width*k points,
        // only read the second column as values, as timestamps are regular (0..width*k)/k
        let files:Vec<String> = match get_files_in_directory(&csv_dir_path) {
            Ok(files) => {
                files
            },
            Err(_) => {
                println!("error: get_files_in_directory");
                return;
            },
        };
        println!("{:?}", files);
        if files.len() < iterations {
            println!("error: there are no iterations number of files in csv_dir_path");
            return;
        }

        let mut data_tmp: Vec<Vec<f32>> = Vec::new();
        let mut global_min: f32 = f32::MAX; // for scale value to [0,height]. (v-global_min)/(global_max-global_min)*height
        let mut global_max: f32 = f32::MIN; // for scale value to [0,height]. (v-global_min)/(global_max-global_min)*height
        for i in 0..iterations {
            let f = format!("{}/{}",csv_dir_path,files[i]); // the i-th files, containing the i-th time series
            let mut res: Vec<f32> = Vec::new();
            let mut point_cnt = 0; // width*k points

            let reader_result = ReaderBuilder::new().has_headers(has_header).from_path(f);
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

                // parse string into double value and then value as u32
                let v = row[1].parse::<f32>().unwrap();
                res.push(v); // assume the second column is value field

                if v > global_max {
                    global_max = v;
                }
                if v < global_min {
                    global_min = v;
                }

                point_cnt += 1;
                if point_cnt >= width*k { // only needs width*k points in each file f
                    break;
                }
            } // end for loop
            if point_cnt < width*k { // the file f has less than width*k points
                println!("error: the file f has less than width*k points");
                return;
            }
            data_tmp.push(res); // finish one time series
        } // end for, finish iteration numbers of time series

        // scale v: (v-global_min)/(global_max-global_min)*height
        data = Vec::new();
        for i in 0..iterations {
            let mut res: Vec<u32> = Vec::new();
            for j in 0..width*k {
                let v: f32 = (data_tmp[i as usize][j as usize]-global_min)/(global_max-global_min)* height as f32;
                res.push(v as u32);
            }
            data.push(res);
        } // end for

    }// end else


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
         res
    }).collect();

    //for row in data.iter() {
        //println!("{:?}", row);
    //}

    //for row in downsamples.iter() {
        //println!("{:?}", row);
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

    img.save(format!("output-i{}-k{}-w{}-h{}-u{}-d{}.png", iterations, k, width, height, use_external_data, downsampling)).unwrap();

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
    img.save(format!("output-i{}-k{}-w{}-h{}-u{}-d{}.png", iterations, k, width, height, use_external_data, downsampling)).unwrap();
}
