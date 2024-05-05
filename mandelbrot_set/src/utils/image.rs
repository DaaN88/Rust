use std::path::Path;
use image::ExtendedColorType;

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
pub fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize)
) -> Result<(), std::io::Error> {
    let path = Path::new(filename);

    image::save_buffer(path, pixels, bounds.0 as u32, bounds.1 as u32, ExtendedColorType::L8).unwrap();

    Ok(())
}