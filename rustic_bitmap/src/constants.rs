// Constants for parsing bitmap image headers
// For more information consult the bitmap format header specification
pub const HEADER_SIZE: u32 = 14;
pub const INFOHEADER_SIZE:u32 = 40;
pub const TOTAL_HEADER_SIZE:u32 = HEADER_SIZE + INFOHEADER_SIZE;

// Header offsets
pub const HEADER_FILE_SIZE:usize = 2;
pub const HEADER_PIXEL_ARRAY_OFFSET:usize = 10;

// InfoHeader offsets
pub const HEADER_WIDTH_OFFSET:usize = 18;
pub const HEADER_HEIGHT_OFFSET:usize = 22;
pub const HEADER_BITS_PER_PIXEL:usize = 28;
pub const HEADER_PLANES_OFFSET:usize = 26;
pub const INFOHEADER_SIZE_OFFSET:usize = 14;
pub const HEADER_COMPRESSION_OFFSET:usize = 30;
pub const HEADER_IMAGE_SIZE:usize = 34;
pub const HEADER_COLORS_USED:usize = 46;
pub const HEADER_IMPORTANT_COLORS:usize = 50;
