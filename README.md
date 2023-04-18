# Animated-Julia
A parallel Rust solution for animating Julia sets.  
[Click here for demo:  
![example video](https://img.youtube.com/vi/GRp3HAUCKk8/0.jpg)](https://www.youtube.com/watch?v=GRp3HAUCKk8)

Looking for a live/interactive GLSL demo ? [Click here!](https://www.shadertoy.com/view/slSBDm)
## Usage

```
Usage: julia [MODE]
Modes:
    "-jd <re(c)> <im(c)>": Display a simple Julia fractal
    "-jr <norm(c)>": Render Julia rotations frames
```
This program will output every frame into `./render`.  
You'll need `ffmpeg` to turn this into a GIF/Video.

## Build
Clone the git and just `cargo build --release`
