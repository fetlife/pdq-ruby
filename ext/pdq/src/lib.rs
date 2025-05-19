use magnus::{function, prelude::*, Error, Ruby, RString, RArray};
use image::ImageReader;
use pdqhash::generate_pdq;
use hex;

fn pdq_hash(image_path: RString) -> RArray {
    let path = image_path.to_string()
        .expect("Failed to convert path to string");
    
    // Load the image
    let img = ImageReader::open(path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");
    
    // Generate PDQ hash
    let (hash, quality) = generate_pdq(&img)
        .expect("Failed to generate PDQ hash");
    
    // Create Ruby array with hash and quality
    let result = RArray::new();
    result.push(hex::encode(hash)).unwrap();
    result.push(quality).unwrap();

    result
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Pdq")?;
    module.define_singleton_method("pdq_hash", function!(pdq_hash, 1))?;
    Ok(())
}