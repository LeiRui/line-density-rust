# Pixel-perfect M4

This repository renders line charts without anti-aliasing for DSSIM experiments.

## 1. Build

```
cargo build --release
```

## 2. Plot

```
target/release/line-density width height csv_path has_header
```

-   width: the pixel width of the rendered line chart
-   height: the pixel height of the rendered line chart
-   csv_path: the input csv which contains one time series
    -   we assume that the input time series in the csv has already been scaled onto the width*height canvas pixel space, which is realized by M4_VISUALIZATION_EXP/tools/parse.py

-   has_header: the input csv path has header or not

For example:

```
target/release/line-density 100 100 ts-rawQuery-100.csv true
```

The command plots the line chart of time series in `ts-rawQuery-100.csv` on a `100*100` canvas. The output png is `ts-rawQuery-100.csv-100.png`.

