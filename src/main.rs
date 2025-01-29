use std::io::Write;
use rustic_bitmap::*;
use std::io::Read;
use std::fs::File;
use std::io;
use std::env;
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 3 {
        eprintln!("Invalid arguments. Usage: bmpuzzle [-i|-e] [bmp_file] [hidden_data]");
        return Ok(());
    }
    let operation = &args[1];
    let bmp_file_path = &args[2];
    let hidden_data_file_path = &args[3];

    if operation != "-i" && operation != "-e" {
        eprintln!("Invalid arguments. Usage: bmpuzzle [-i|-e] [bmp_file] [hidden_data]");
        return Ok(());
    }

    let mut bmp_file = File::open(bmp_file_path)?;
    let mut bmp = Vec::new();
    bmp_file.read_to_end(&mut bmp)?;

    if !bmp.has_file_signature() {
	println!("File is not a bitmap.");
	return Ok(());
    }

    println!("Bitmap file size: {} bytes.", bmp.len());
    println!("Bitmap padding size: {} bytes.", bmp.get_padding_size());
    println!("Padding per line: {} bytes.", bmp.get_padding_per_line());

    let paddingless_width = bmp.get_width() * (bmp.get_bits_per_pixel() / 8) as u32;
    let padding_per_line = bmp.get_padding_per_line();

    if operation == "-i" { // Insert hidden data into bitmap
        let mut hidden_data_file = File::open(hidden_data_file_path)?;
        let mut hidden_data = Vec::new();
        hidden_data_file.read_to_end(&mut hidden_data)?;
        println!("Hidden file size: {} bytes.", hidden_data.len());
	if (hidden_data.len() as u64) > (bmp.get_padding_size() as u64) {
	    println!("Not enough padding. Aborting.");
	    return Ok(());
	}
	
	let chunks = hidden_data.chunks(padding_per_line.into())
	    .map(|chunk| chunk.to_vec())
	    .collect::<Vec<Vec<u8>>>();

	let mut current_position = bmp.get_pixel_array_offset() - 1;
	for chunk in chunks {
	    // Adjust current_position based on paddingless_width
	    current_position += paddingless_width as usize;
	
	    // Write the chunk to the BMP
	    for byte in chunk.iter() {
                current_position += 1;
	        bmp[current_position] = *byte;
	    }
	}
	let mut bmp_file = File::create(bmp_file_path)?;
	bmp_file.write_all(&bmp)?;
	println!("Data written to {}", bmp_file_path);

    }

    if operation == "-e" { // Extract data out of bitmap
        let mut hidden_data: Vec<u8> = vec![];
        let mut current_position = bmp.get_pixel_array_offset() - 1;

        while current_position + (paddingless_width as usize) < bmp.len() {
            current_position += paddingless_width as usize;

            for i in 0..padding_per_line {
                current_position += 1;
                if current_position < bmp.len() {
                    hidden_data.push(bmp[current_position]);
                } else {
                    break;
                }
            }

            if current_position >= bmp.len() {
                break;
            }
        }

	let mut hidden_data_file = File::create(hidden_data_file_path)?;
	hidden_data_file.write_all(&hidden_data)?;
	println!("Data extracted to {}", hidden_data_file_path);
    }

    return Ok(());
}
