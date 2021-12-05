# A Bitmapper's Companion - zine/book about bitmap drawing algorithms and math with code examples in Rust

A small zine/book written in LaTeX. In progress. See Building section below for how to build.

## Samples


![bezier interactive demo](./bezier_interactive.gif?raw=true)


<kbd>

![cover_sample](./cover_sample.png?raw=true)

</kbd>

--------

<kbd>

![thumb_sample](./thumb_sample.png?raw=true)

</kbd>

--------

<kbd>

![frontmatter_sample](./frontmatter_sample.png?raw=true)

</kbd>

--------

<kbd>

![frontmatter_sample2](./frontmatter_sample2.png?raw=true)

</kbd>
--------

<kbd>

![page_sample](./page_sample.png?raw=true)

</kbd>


## Building

Run `make`, output will be in the `./build` directory.

To run the rust example binaries, first you can inspect them with `ls ./src/bin/`, for example:

```shell
$ ls ./src/bin
atkinsondither.rs
beams.rs
bezierglyph.rs
bezier.rs
boundingcircle.rs
bresenham.rs
distance_between_two_points.rs
floyddither.rs
fonts.rs
hilbert.rs
introduction.rs
rotation.rs
scale.rs
shearing.rs
smooth_scale.rs
xbmtors.rs
zcurve.rs
```

Then execute one with `cargo run --bin` for example `cargo run --bin atkinsondither`.
