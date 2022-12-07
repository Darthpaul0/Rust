mod args;

use args::Args;
use image::{
    imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    println!("{:?}", args);

    let (image_1, image_1_format) = find_image_from_path(args.image_1);
    let (image_2, image_2_format) = find_image_from_path(args.image_2);

    // handling errors with result
    if image_1_format != image_2_format {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_1, image_2) = standardise_size(image_1, image_2);
    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

    let combined_data = combine_images(image_1, image_2);

    output.set_data(combined_data)?;

    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        image_1_format,
    )
    .unwrap();
    Ok(())
}

#[derive(Debug)]
enum ImageDataErrors {
    BufferTooSmall,
    DifferentImageFormats,
}

/**
 * struct named FloatingImage to hold the width, height, and data
 * of the image, as well as the name of the output file
 */
struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = 3_655_744;
        let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
        FloatingImage {
            width,
            height,
            data: buffer,
            name,
        }
    }
    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }
}

// function named find_image_from_path to open the image file from a path argument
fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader = Reader::open(path).unwrap();
    let image_format = image_reader.format().unwrap();
    let image = image_reader.decode().unwrap();
    (image, image_format)
}
/**
 * If image_2 is the smallest image, then image_1 needs to be resized to
 * match the smallest dimensions. Otherwise, image_2 needs to be resized
 */
fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
    println!("width: {}, height: {}\n", width, height);
    if image_2.dimensions() == (width, height) {
        // resize_exact
        // borrows the image, and, using the width, height,
        // and FilterType arguments, resizes the image
        (image_1.resize_exact(width, height, Triangle), image_2)
    } else {
        (image_1, image_2.resize_exact(width, height, Triangle))
    }
}

/**
 * returns the width and height of the image as a tuple.
 * These tuples can be compared, and the smallest returned
 */
fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}
/**
 * implements the to_rgba8 method, which returns an ImageBuffer containing
 * a Vec<u8>, and the ImageBuffer implements the into_vec method,
 * which returns the Vec<u8>:
 */
fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2)
}
/**
 * vec_1 and vec_2 variables are passed to the alternate_pixels function
 * which returns the combined image data by alternating the
 * RGBA pixel sets of the two images
 */
fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_1.len()];

    let mut i = 0;
    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
        }
        i += 4;
    }

    combined_data
}
/**
 * takes a reference to a Vec<u8>, and returns the RGBA pixel set for
 * that Vec<u8> starting and ending at a given index
 */
fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let val = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index out of bounds"),
        };
        rgba.push(val);
    }
    rgba
}
