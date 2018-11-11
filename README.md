# A Raytracer
*written in Rust*

This project is intended to make myself familiar with Rust *and* Raytracing. It is therefore written in "simple" Rust and the code well commented where necessary.

Here's what it can do at the moment.

![Four raytraced spheres](https://i.imgur.com/J3mXJ3H.png)

## Build & Run

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