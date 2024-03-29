# A Raytracer

_written in Rust_

Here's what it can do at the moment.

![Four raytraced spheres](https://i.imgur.com/hOc5FW9.png)

# Build & Run

The binary accepts a few command line arguments.

```bash
$ target/debug/rusttracer -h
Usage:
  target/debug/rusttracer [OPTIONS]

A raytracer

Optional arguments:
  -h,--help             Show this help message and exit
  -f,--fov FOV          The field of view.
  -w,--write-file WRITE_FILE
                        Write output to a file.
  -s,--write-stdout     Write output to stdout.
```

For example, to build, run with a fov of 90, write to stdout and pipe to display

```bash
cargo run -- -f 90 -s | display
```

There are some tests as well, run them with

```bash
cargo test
```

# Resources

## Code

[A nice walkthrough for a raytracer with the simplified rendering equation](https://tmcw.github.io/literate-raytracer/)

## Theory

[The free online version of _Physically Based Rendering_](http://www.pbr-book.org/3ed-2018/Introduction/Photorealistic_Rendering_and_the_Ray-Tracing_Algorithm.html)

[The amazing TU Wien rendering course](https://www.youtube.com/playlist?list=PLujxSBD-JXgnGmsn7gEyN28P1DnRZG7qi)

### Vector Math

[The explanation about projection and how it relates to vector reflection is top notch](https://www.youtube.com/watch?v=NOBhfEHOYZs)

### Camera

[Well explained w/ schematics](https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays)

[TU Wien camera lecture](https://youtu.be/ZhN5-o397QI)
