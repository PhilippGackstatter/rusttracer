# A Raytracer
*written in Rust*

This project is intended to make myself familiar with Rust *and* Raytracing. It is therefore written in "simple" Rust and the code well commented where necessary.

Here's what it can do at the moment.

![Four raytraced spheres](https://i.imgur.com/J3mXJ3H.png)

## Build & Run

This project uses standard cargo commands.

To write the result to a ppm file pass a filename
```bash
cargo run picture.ppm
```

To write the result to stdout, pass nothing and pipe it to a suitable program, e.g.
```bash
cargo run | display
```

There are some tests as well, run them with
```bash
cargo test
```