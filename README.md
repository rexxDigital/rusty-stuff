# Rusty stuff

Just making some rusty stuff for future development and some private projects.

## WIP

Currently working on HSB conversion to RGB for my color module, will work something like this ->

```rust
mod color;

fn main() {
    let blue = color::Color::fromHSB(100, 65, 53)
}
```