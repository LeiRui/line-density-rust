# DenseLines

This respository plots DenseLines.

## 1. Build

```
cargo build --release
```

## 2. Plot using Synthetic Data

```
target/release/line-density [n] [k] [w] [h]
```

-   n: the number of time series
-   k: control the number of points in a time series as `k*w`
-   w: the pixel width of the rendered line chart
-   h: the pixel height of the rendered line chart

For example:

```
target/release/line-density 1 100 400 400
```

The command plots the line chart of one time series containing `100*400` points on a `400*400` canvas, using raw data points and M4 representation points to render `output-i1-k100-w400-h400-ufalse-dfalse.png` and `output-i1-k100-w400-h400-ufalse-dtrue.png`, respectively.

## 3. Plot using External Data

```
target/release/line-density [n] [k] [w] [h] [use_external_data] [csv_dir_path] [has_header]
```

-   n: the number of time series
-   k: control the number of points in a time series as `k*w`
-   w: the pixel width of the rendered line chart
-   h: the pixel height of the rendered line chart
-   use_external_data: set as true
-   csv_dir_path: the path of the directory that contains csv files
-   has_header: whether the csv has header

For example:

```
target/release/line-density 45 10 160 100 true /root/csvDir true
```

The command plots the line chart of 45 time series using the first 1600 points on a `160*100` canvas, using raw data points and M4 representation points to render `output-i45-k10-w160-h100-utrue-dfalse.png` and `output-i45-k10-w160-h100-utrue-dtrue.png`, respectively.

## References

-   Moritz, Dominik, and Danyel Fisher. "Visualizing a million time series with the density line chart." *arXiv preprint arXiv:1808.06019* (2018).
-   Jugel, Uwe, et al. "M4: a visualization-oriented time series data aggregation." *Proceedings of the VLDB Endowment* 7.10 (2014): 797-808.

