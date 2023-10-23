# Pixel-perfect M4

This respository demonstrates the pixel-perfectness of [M4](http://www.vldb.org/pvldb/vol7/p797-jugel.pdf) for line charts rendered with no anti-aliasing.

## 1. Build

```
cargo build --release
```

## 2. Plot

```
target/release/line-density width height csv_path has_header tqs tqe
```

-   width: the pixel width of the rendered line chart
-   height: the pixel height of the rendered line chart
-   csv_path: the input csv path 
-   has_header: the input csv path has header or not
-   tqs: the start time (x-axis) of the rendered line chart
-   tqe: the end time (x-axis) of the rendered line chart

For example:

```
target/release/line-density 100 100 ts-rawQuery-100.csv true 0 617426057626
```

The command plots the line chart of one time series from `ts-rawQuery-100.csv` on a `100*100` canvas with time range as `[0,617426057626)`. The output png is `ts-rawQuery-100.csv-100.png`.

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

