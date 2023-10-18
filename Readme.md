# Pixel-perfect M4

This respository is aimed to use reproduce the pixel-perfect result of M4 in paper "U. Jugel, Z. Jerzak, G. Hackenbroich, and V. Markl. M4: A visualization-oriented time series data aggregation. Proc. VLDB Endow., 7(10):797–808, 2014". 

## Build

```
cargo build --release
```

## Plot

```
target/release/line-density 1 100 400
```

This command plots the line chart of one time series containing `100*400` points on a `400*400` canvas, using raw data points and M4 representation points to output `output-i1-k100-w400-h400-ufalse-dfalse.png` and `output-i1-k100-w400-h400-ufalse-dtrue.png`, respectively.

## Compute DSSIM

We use the definition DSSIM=1-(1-SSIM)/2, the same as used in the experiments by Jugel et al.

```
python3 run-DSSIM.py
```

This command computes the DSSIM of the two output pngs in the previous plot step. 

The result DSSIM equals 1, meaning that M4 can achieve pixel-perfectness when using the [drawing tool that support Bresenham's line drawing algorithm](https://docs.rs/imageproc/latest/imageproc/drawing/fn.draw_line_segment_mut.html).
