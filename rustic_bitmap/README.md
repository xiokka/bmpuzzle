# rustic_bitmap
## Description
A simple Rust library for bitmap manipulation. This library provides tools for reading, modifying, and writing bitmap images.

## Installation
Clone this library.
```bash
git clone https://github.com/xiokka/rustic_bitmap.git
```

Add the dependency to your Cargo.toml, specifying the location where you cloned the library.
```toml
[dependencies]
rustic_bitmap = { path = "modify/this/path/to/library" }
```

In your Rust source file, import the crate.
```rust
use rustic_bitmap::*;
```


## Usage
All operations are performed on vectors of bytes.
```rust
//Create a new empty bitmap using new_bitmap(width, height, bits_per_pixel)
let mut bmp:Vec<u8> = Vec::<u8>::new_bitmap(88, 31, 24);

// Draws a circle with a given radius, center and color
let circle_center = Point {x:87, y:0};
let color_circle = Rgb {r:255, g:0, b:0}; // Red
let radius = 20;
bmp.draw_circle(&circle_center, radius, &color_circle);

// Draws a blue rectangle where position1 and position2 are opposing corners
let position1: Point = Point {x: 0, y: 0};
let position2: Point = Point {x: 87, y:30};
let color = Rgb {r:0, g:0, b:255}; // Blue
bmp.draw_rectangle(&position1, &position2, &color);

// Draws a string
let position: Point = Point {x: 3, y: 20};
let color = Rgb {r:255, g:255, b:255}; // White
bmp.draw_string("Hello!", &position, &color);

// Store vector of bytes in file_path
let file_path = "button.bmp";
let mut file = File::create(file_path).unwrap();
file.write_all(&bmp).unwrap();
```
