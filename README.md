# Conway's game of life

A simple implementation of game of life in Rust. I just want to learn the basic syntax of Rust. 

![example](https://github.com/rzavalet/simple-conway-game-of-life/blob/main/Simulation.gif)

## Dependencies

Please install the `SDL2` library and set the `RUSTFLAGS`, e.g.:

```code
export RUSTFLAGS="-L ${HOME}/opt/sdl2/lib"
```

## Build and run

Build and run as usual:

```code
cargo run
```

## Some key/mouse events

- `q` or `Esc` to exit.
- `SPACE` to pause the event loop.
- `RETURN` to increase the simulation speed.
- `C` to clear the world after pausing the loop.
- `R` to generate a new random initial state after pausing the loop.
- `Click` and `Move` to invert the state of a cell after pausing the loop.
