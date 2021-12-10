# A Bitmapper's Companion - zine/book about bitmap drawing algorithms and math with code examples in Rust

A small zine/book written in LaTeX. In progress. See Building section below for how to build.

[View current PDF build here](./build/bitgeom.pdf?raw=true)


<details>
<summary>Click to show planned contents</summary>


1. **Introduction**
  - Data representation
  - Displaying pixels to your screen
  - Bits to byte pixels
  - Loading graphics files in Rust
  - Including xbm files in Rust
2. **Points And Lines**
  - Distance between two points
  - Equations of a line
    - *Line through a point 𝑃 = (𝑥𝑝, 𝑦𝑝) and a slope 𝑚*
    - *Line through two points*
  - Distance from a point to a line
    - *Using the implicit equation form*
    - *Using an 𝐿 defined by two points 𝑃1, 𝑃2*
    - *Using an 𝐿 defined by a point 𝑃𝑙 and angle ̂𝜃*
    - *Find perpendicular to line that passes through given point*
  - Angle between two lines
    - *Intersection of two lines*
    - *Line equidistant from two points*
    - *Normal to a line through a point*
3. **Points And Line Segments**
  - Drawing a line segment from its two endpoints
  - Drawing line segments with width
  - Intersection of two line segments
    - *Fast intersection of two line segments*
  - Points, Lines and Circles
  - Equations of a circle
  - Bounding circle
4. **Curves other than circles**
  - Parametric elliptical arcs
  - Bézier curves
5. **Points, Lines and Shapes**
  - Union, intersection and difference of polygons
  - Centroid of polygon
  - Polygon clipping
  - Triangle filling
  - Flood filling
6. **Vectors, matrices and transformations**
  - Rotation of a bitmap
    - *Fast 2D Rotation*
  - 90° Rotation of a bitmap by parallel recursive subdivision
  - Magnification/Scaling
    - *Smoothing enlarged bitmaps*
    - *Stretching lines of bitmaps*
  - Mirroring
  - Shearing
    - *The relationship between shearing factor and angle*
  - Projections
7. **Addendum**
  - Faster Drawing a line segment from its two endpoints using Sym-
metry
  - Joining the ends of two wide line segments together
  - Composing monochrome bitmaps with separate alpha channel data
  - Orthogonal connection of two points
  - Join segments with round corners
  - Faster line clipping
  - Space-filling Curves
    - *Hilbert curve*
    - *Sierpiński curve*
    - *Peano curve*
    - *Z-order curve*
    - *flowsnake curve*
  - Dithering
    - *Floyd-Steinberg*
    - *Atkinson dithering*
  - Marching squares

</details>

## Samples

<table>
<tr>
<td>
<kbd>

![cover_sample](./samples/cover_sample.png?raw=true)

</kbd>
</td>
<td>
<kbd>

![thumb_sample](./samples/thumb_sample.png?raw=true)

</kbd>
</td>
</tr>
<tr>
<th>Cover</th>
<th>

[Thumb index](https://en.wikipedia.org/wiki/Thumb_index) overview

</th>
</tr>

<tr>
<td>
<kbd>

![frontmatter_sample](./samples/frontmatter_sample.png?raw=true)

</kbd>
</td>
<td>
<kbd>

![frontmatter_sample2](./samples/frontmatter_sample2.png?raw=true)

</kbd>
</td>
</tr>
<tr>
<th> Frontmatter</th><th>contents</th>
<tr/>
<tr>
<td>

<kbd>

![page_sample](./samples/page_sample.png?raw=true)

</kbd>

</td>
<td>


![bezier interactive demo](./samples/bezier_interactive.gif?raw=true)

</td>
</tr>
<tr><th>Page spread</th><th>Bezier interactive demo</th></tr>
</table>


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
