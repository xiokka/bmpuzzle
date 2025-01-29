mod get;
pub use get::*;

mod constants;
pub use crate::constants::*;

mod font;
use crate::font::*;

pub struct Point {
	pub x: u32,
	pub y: u32
}

pub struct Rgb {
	pub r: u8,
	pub g: u8,
	pub b: u8
}

pub trait Bitmap {
	fn new_bitmap(width: u32, height: u32, bpp: u16) -> Vec<u8>;
	fn point_exists(&self, point: &Point) -> bool;
	fn draw_point(&mut self, start:&Point, color: &Rgb);
	fn draw_circle(&mut self, center: &Point, radius: u32, color: &Rgb);
	fn draw_line(&mut self, start: &Point, end: &Point, color: &Rgb);
	fn draw_rectangle(&mut self, point1: &Point, point2: &Point, color: &Rgb);
	fn draw_char(&mut self, char_index: usize, position: &Point, color:&Rgb);
	fn draw_string(&mut self, string: &str, position: &Point, color: &Rgb);
	fn draw_polygon(&mut self, points: &[Point], color: &Rgb);
	fn has_file_signature(&self) -> bool;
}

impl Bitmap for Vec<u8> {
	fn new_bitmap(width: u32, height: u32, bpp: u16) -> Vec<u8> {
		// Each scan line is zero padded to the nearest 4-byte boundary. If the image has a width that is not divisible by four, say, 21 bytes, there would be 3 bytes of padding at the end of every scan line.
		let mut padded_width:u32 = width * (bpp as u32 / 8);
		// Compute the remainder when divided by 4
		let remainder = padded_width % 4;
		// If there's no remainder, the value is already a multiple of 4
		if remainder != 0 {
			padded_width = padded_width + (4 - remainder); // Add padding
		}
		let len = (padded_width * height) + TOTAL_HEADER_SIZE as u32; // Total size of bitmap, with headers and padded pixel array

		let mut new_bitmap:Vec<u8> = vec![0; len as usize];
		// Insert bitmap header signature
		new_bitmap[0] = b'B';
		new_bitmap[1] = b'M';

		// Insert offset of pixel array in header
		let offset_pixel_array_as_u8 = TOTAL_HEADER_SIZE.to_le_bytes();
		for i in 0..4 {
                        new_bitmap[HEADER_PIXEL_ARRAY_OFFSET + i] = offset_pixel_array_as_u8[i]; // Convert u32 value into array of u8 (Little Endian)
                }

		// Insert image width in header
		let width_as_u8 = width.to_le_bytes(); // Convert u32 value into array of u8 (Little Endian)
		for i in 0..4 {
			new_bitmap[HEADER_WIDTH_OFFSET + i] = width_as_u8[i];
		}

		// Insert image height in header
		let height_as_u8 = height.to_le_bytes(); // Convert u32 value into array of u8 (Little Endian)
		for i in 0..4 { 
                        new_bitmap[HEADER_HEIGHT_OFFSET + i] = height_as_u8[i];
                }

		// Insert bits per pixel in header
		let bpp_as_u8 = bpp.to_le_bytes(); // Convert u32 value into array of u8 (Little Endian)
		for i in 0..2 {
                        new_bitmap[HEADER_BITS_PER_PIXEL + i] = bpp_as_u8[i];
                }

		// Insert file size in header
		let file_size = new_bitmap.len() as u32;
		let file_size_as_u8 = file_size.to_le_bytes(); // Convert u32 value into array of u8 (Little Endian)
                for i in 0..4 {
                        new_bitmap[HEADER_FILE_SIZE + i] = file_size_as_u8[i];
                }

		// Insert number of planes in header
		new_bitmap[HEADER_PLANES_OFFSET] = 1;

		// Insert size of InfoHeader in header;
		let infoheader_size_as_u8 = INFOHEADER_SIZE.to_le_bytes(); // Convert u32 value into array of u8 (Little Endian)
                for i in 0..4 {
                        new_bitmap[INFOHEADER_SIZE_OFFSET + i] = infoheader_size_as_u8[i];
                }

		return new_bitmap;
	}

	fn draw_point(&mut self, start: &Point, color: &Rgb) {
		// Ensure the point is within the bounds of the bitmap
		if !self.point_exists(start) {
			return;
		}
	
		// Convert dimensions to usize for consistent indexing
		let width = self.get_width() as usize;
		let bytes_per_pixel = (self.get_bits_per_pixel() as usize) / 8;

		// Calculate the row width in bytes, including padding
		let row_width = width * bytes_per_pixel;
		let padding = (4 - (row_width % 4)) % 4; // Calculate padding to make row width a multiple of 4
		let padded_row_width = row_width + padding;

		// Calculate the base index for the pixel location
		let base_index = (start.y as usize * padded_row_width + start.x as usize * bytes_per_pixel) + self.get_pixel_array_offset() as usize;
	
		// Ensure the base index is within bounds before accessing the array
		if base_index + 2 < self.len() {
			self[base_index] = color.b;   // Blue
			self[base_index + 1] = color.g; // Green
			self[base_index + 2] = color.r; // Red
		} else {
			eprintln!("Index out of bounds: {}", base_index);
		}
	}

	fn point_exists(&self, point: &Point) -> bool {
                if !self.has_file_signature() {
                        eprintln!("Invalid file signature. Not a Bitmap.");
                        return false;
                }
		return (self.get_height() > point.y) && (self.get_width() > point.x);
	}

	fn draw_circle(&mut self, center: &Point, radius: u32, color: &Rgb) {
	    // Calculate the bounding box for the circle
	    let min_x = center.x.saturating_sub(radius);
	    let max_x = center.x.saturating_add(radius);
	    let min_y = center.y.saturating_sub(radius);
	    let max_y = center.y.saturating_add(radius);
	
	    // Iterate over the bounding box around the circle
	    for y in min_y..=max_y {
	        for x in min_x..=max_x {
	            // Calculate the distance from the center to the current point (x, y)
	            let dx = x as i32 - center.x as i32;
	            let dy = y as i32 - center.y as i32;
	            if dx * dx + dy * dy <= (radius as i32) * (radius as i32) {
	                let point = Point { x, y };
	                self.draw_point(&point, color);
	            }
	        }
	    }
	}

	fn draw_line(&mut self, start: &Point, end: &Point, color: &Rgb) {
		let mut x0 = start.x as i32;
		let mut y0 = start.y as i32;
		let x1 = end.x as i32;
		let y1 = end.y as i32;
		let dx = (x1 - x0).abs();
		let dy = (y1 - y0).abs();
		let sx = if x0 < x1 { 1 } else { -1 };
		let sy = if y0 < y1 { 1 } else { -1 };
		let mut err = dx - dy;
		loop {
			self.draw_point(&Point { x: x0 as u32, y: y0 as u32 }, color);
			if x0 == x1 && y0 == y1 {
				break;
			}
			let e2 = 2 * err;
			if e2 > -dy {
				err -= dy;
				x0 += sx;
			}
			if e2 < dx {
				err += dx;
				y0 += sy;
			}
		}
	}

	// Input two opposite corners of the rectangle
	fn draw_rectangle(&mut self, point1: &Point, point2: &Point, color: &Rgb) {
		let point3 = Point {x:point1.x , y:point2.y};
		let point4 = Point {x: point2.x, y: point1.y};
		self.draw_line(&point1, &point3, &color);
		self.draw_line(&point3, &point2, &color);
		self.draw_line(&point2, &point4, &color);
		self.draw_line(&point4, &point1, &color);
	}

	fn draw_char(&mut self, char_index: usize, position: &Point, color: &Rgb) {
		// Ensure char_index is within bounds of FONT_BITMAP
		if char_index >= FONT_BITMAP.len() {
			return; // or handle the error as appropriate
		}

		// Get the bitmap for the character
		let bitmap = &FONT_BITMAP[char_index];

		// Iterate over rows of the character's bitmap from bottom to top
		for row in (0..FONT_HEIGHT).rev() {
			// Iterate over columns of the character's bitmap from left to right
			for col in 0..FONT_WIDTH {
				// Calculate the index into the bitmap
				let bit_index = (FONT_HEIGHT - 1 - row) * FONT_WIDTH + col;
				let byte_index = bit_index / 8;
				let bit_position = 7 - (bit_index % 8);
	
				// Ensure the byte index is within bounds of the bitmap array
				if byte_index < bitmap.len() {
					// Create a mask for the bit position
					let mask = 1 << bit_position;
	
					// Check if the specific bit is set
					if (bitmap[byte_index] & mask) != 0 {
						// Calculate the correct x and y coordinates for the point
						let x = position.x as u32 + (FONT_WIDTH - 1 - col) as u32;
						let y = position.y as u32 + row as u32;
	
						// Draw the point
						let point = Point { x, y };
						self.draw_point(&point, color);
					}
				}
			}
		}
	}


	fn draw_string(&mut self, string: &str, position: &Point, color: &Rgb) {
		let mut x_offset = position.x; // Start at the initial x position
		// Iterate over each character in the string
		for char in string.chars() {
			// Find the index of the character in the font bitmap
			let char_index = char as usize - 32;
			// Draw the character at the current position
			self.draw_char(char_index, &Point { x: x_offset, y: position.y }, color);
			// Move the x offset by the width of the character plus any spacing
			x_offset += FONT_WIDTH as u32;
		}
	}

	fn draw_polygon(&mut self, points: &[Point], color: &Rgb) {
		for i in 0..points.len()-1 {
			self.draw_line(&points[i], &points[i+1], color);
		}
		self.draw_line(&points[0], &points[points.len()-1], color); 
	}

	// Checks if a vector of bytes has the BMP file signature
	fn has_file_signature(&self) -> bool {
		return self[0] == b'B' && self[1] == b'M';
	}

}


// Example usage
//#[test]
//fn test_new_bitmap() {
//	let file_path = "example.bmp";
//	let mut file = File::create(file_path).unwrap();
//	let mut bmp:Vec<u8> = Vec::<u8>::new_bitmap(40, 40, 24);
//	assert_eq!(bmp.get_height(), 40);
//	assert_eq!(bmp.get_width(), 40);
//	assert_eq!(bmp.get_bits_per_pixel(), 24);
//	assert_eq!(bmp.get_file_size(), bmp.len() as u32);
//	assert_eq!(bmp.get_size_of_info_header(), 40);
//	assert_eq!(bmp.get_planes(), 1);
//	assert_eq!(bmp.get_compression(), 0);
//	let red:Rgb = Rgb {r: 255, g: 0, b: 0};
//	let position: Point = Point {x: 20, y: 20};
//	bmp.draw_point(&position, &red);
//        let position: Point = Point {x: 0, y: 0};
//        bmp.draw_point(&position, &red);
//        let position: Point = Point {x: 39, y: 39};
//        bmp.draw_point(&position, &red);
//        let position: Point = Point {x: 0, y: 39};
//        bmp.draw_point(&position, &red);
//        let position: Point = Point {x: 39, y: 0};
//        bmp.draw_point(&position, &red);
//	let position1 = Point {x: 38, y: 38};
//	let position2 = Point {x: 1, y: 1};
//	let color = Rgb {r:0, g:255, b:0};
//	bmp.draw_rectangle(&position1, &position2, &color);
//        let position1 = Point {x: 20, y: 20};
//	let radius = 10;
//        let color = Rgb {r:0, g:0, b:255};
//	bmp.draw_circle(&position1, radius, &color);
//        let red:Rgb = Rgb {r: 255, g: 0, b: 0};
//        let position: Point = Point {x: 20, y: 20};
//        bmp.draw_point(&position, &red);
//	bmp.draw_string("!", &position, &red);
//
//        let color = Rgb {r:255, g:255, b:255};
//	let point1 = Point{x: 10, y: 30};
//	let point2 = Point{x: 26, y: 33};
//	let point3 = Point{x: 35, y: 20};
//	let point4 = Point{x: 27, y: 6};
//	let point5 = Point{x: 12, y: 5};
//	let point6 = Point{x: 5, y: 14};
//	let points = [point1, point2, point3, point4, point5, point6];
//	bmp.draw_polygon(&points, &color);
//	file.write_all(&bmp).unwrap();
//}

// Example usage
//#[test]
//fn test_new_bitmap_website() {
//1
//        let file_path = "test.bmp";
//        let mut file = File::create(file_path).unwrap();
//        let mut bmp:Vec<u8> = Vec::<u8>::new_bitmap(100, 80, 24);
//
//2
//        let color= Rgb {r: 255, g: 0, b: 0};
//        let position: Point = Point {x: 50, y: 40};
//        bmp.draw_point(&position, &color);
//3
//        let position1 = Point {x: 10, y: 22};
//        let position2 = Point {x: 110, y: 102};
//        let color = Rgb {r:0, g:255, b:0};
//	bmp.draw_line(&position1, &position2, &color);
//        let position1 = Point {x: 70, y: 22};
//        let position2 = Point {x: 170, y: 102};
//        let color = Rgb {r:0, g:255, b:0};
//        bmp.draw_line(&position1, &position2, &color);
//        let position1 = Point {x: 70, y: 3};
//        let position2 = Point {x: 170, y: 83};
//        let color = Rgb {r:0, g:255, b:0};
//        bmp.draw_line(&position1, &position2, &color);
//4
//        let position1 = Point {x: 10, y: 3};
//        let position2 = Point {x: 70, y: 22};
//        let color = Rgb {r:0, g:255, b:0};
//        bmp.draw_rectangle(&position1, &position2, &color);
//
//5
//        let position1 = Point {x: 14, y: 60};
//        let radius = 10;
//        let color = Rgb {r:0, g:0, b:255};
//        bmp.draw_circle(&position1, radius, &color);

//	let position: Point = Point {x: 14, y: 7};
//	let color = Rgb {r:255, g:255, b:255};
//        bmp.draw_string("Hello!", &position, &color);
//        file.write_all(&bmp).unwrap();
//}


