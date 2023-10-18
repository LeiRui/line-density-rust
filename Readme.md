# Pixel-perfect M4

This respository demonstrates the pixel-perfectness of [M4](http://www.vldb.org/pvldb/vol7/p797-jugel.pdf) for line charts rendered with no anti-aliasing.

## 1. Build

```
cargo build --release
```

## 2. Plot

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

The command plots the line chart of one time series containing `100*400` points on a `400*400` canvas, using raw data points and M4 representation points to output `output-i1-k100-w400-h400-ufalse-dfalse.png` and `output-i1-k100-w400-h400-ufalse-dtrue.png`, respectively.

## 3. Compute DSSIM

We use the definition DSSIM=1-(1-SSIM)/2, the same as used in the experiments by Jugel et al.

```
python3 run-DSSIM.py -f1 [image1 path] -f2 [image2 path]
```

For example:

```
python3 run-DSSIM.py -f1 output-i1-k100-w400-h400-ufalse-dfalse.png -f2 output-i1-k100-w400-h400-ufalse-dtrue.png
```

The command computes the DSSIM of the two output pngs in the previous plot step. 

The result DSSIM equals 1, meaning that M4 can achieve pixel-perfectness when using the [drawing tool that support Bresenham's line drawing algorithm](https://docs.rs/imageproc/latest/imageproc/drawing/fn.draw_line_segment_mut.html).



## References

```
@article{DBLP:journals/pvldb/JugelJM14,
  author       = {Uwe Jugel and
                  Zbigniew Jerzak and
                  Gregor Hackenbroich and
                  Volker Markl},
  title        = {{M4:} {A} Visualization-Oriented Time Series Data Aggregation},
  journal      = {Proc. {VLDB} Endow.},
  volume       = {7},
  number       = {10},
  pages        = {797--808},
  year         = {2014},
  url          = {http://www.vldb.org/pvldb/vol7/p797-jugel.pdf},
  doi          = {10.14778/2732951.2732953},
  timestamp    = {Sat, 25 Apr 2020 13:58:52 +0200},
  biburl       = {https://dblp.org/rec/journals/pvldb/JugelJM14.bib},
  bibsource    = {dblp computer science bibliography, https://dblp.org}
}

@article{DBLP:journals/corr/abs-1808-06019,
  author       = {Dominik Moritz and
                  Danyel Fisher},
  title        = {Visualizing a Million Time Series with the Density Line Chart},
  journal      = {CoRR},
  volume       = {abs/1808.06019},
  year         = {2018},
  url          = {http://arxiv.org/abs/1808.06019},
  eprinttype    = {arXiv},
  eprint       = {1808.06019},
  timestamp    = {Sun, 02 Sep 2018 15:01:53 +0200},
  biburl       = {https://dblp.org/rec/journals/corr/abs-1808-06019.bib},
  bibsource    = {dblp computer science bibliography, https://dblp.org}
}
```

