# L-System Plant Generator

Generating and rendering 3D fractal plants using L-systems using OpenGL.

## Features
* Loading fractal base geometry from .obj files
* Loading floor model geometry and material from .obj and .mtl files
* Selection of fractal base models
* Selection of predefined L-systems for plant generation
* Custom L-system input via GUI
* Phong, Gouraud or Flat shading selectable via GUI
* Perspective camera with keyboard and mouse controls
* Randomized placement of plants in a defined area
* Interpolating plant color based on height

## Authors
* Mikołaj Garbowski
* Maksym Bieńkowski
* Jędrzej Grabski

## Installation
* Install [Rust](https://rust-lang.org/tools/install/)
* Install [Just](https://github.com/casey/just) for running utility scripts

## Running
Clone, `just run`. Check out `just -l` for recipes.

## Controls
* Move using WASD/HJKL
* Space to move up, z to move down
* Mouse to look around
* Press `Esc` to enter menu


## References
* [L-System - Wikipedia](https://en.wikipedia.org/wiki/L-system)
* The Algorithmic Beauty of Plants, P. Prusinkiewicz, A. Lindenmayer
* Real-Time Rendering 4th Edition, T. Akenine-Möller, E. Haines, N. Hoffman, A. Pesce, M. Iwanicki, S. Hillaire
* [Learn OpenGL](https://learnopengl.com/)
