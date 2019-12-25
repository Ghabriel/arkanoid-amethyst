# arkanoid-amethyst
A simple implementation of Arkanoid using Amethyst.

## How to run

To run the game, first you'll need to have Rust and Vulkan installed. Then, run:

```
cargo run
```

## Using a joystick

If you prefer to use a joystick instead of a keyboard, change the value of `use_joystick_keybindings` to `true` in `config/game.ron`:

```
use_joystick_keybindings: true,
```
