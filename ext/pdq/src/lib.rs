use hex;
use image::ImageReader;
use magnus::{
    Error, ExceptionClass, RArray, RModule, RString, Ruby, function, gc::register_mark_object, prelude::*, value::Lazy
};
use pdqhash::generate_pdq;

static PDQ_ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    let ex = ruby
        .class_object()
        .const_get::<_, RModule>("Pdq")
        .unwrap()
        .const_get("Error")
        .unwrap();
    // ensure `ex` is never garbage collected (e.g. if constant is
    // redefined) and also not moved under compacting GC.
    // ruby.gc_register_mark_object(ex);
    register_mark_object(ex);
    ex
});

fn pdq_hash(ruby: &Ruby, image_path: RString) -> Result<RArray, Error> {
    let path = image_path.to_string().map_err(|e| {
        Error::new(
            ruby.get_inner(&PDQ_ERROR),
            format!("Failed to convert path to string: {}", e),
        )
    })?;

    // Load the image
    let img = ImageReader::open(path)
        .map_err(|e| {
            Error::new(
                ruby.get_inner(&PDQ_ERROR),
                format!("Failed to open image: {}", e),
            )
        })?
        .decode()
        .map_err(|e| {
            Error::new(
                ruby.get_inner(&PDQ_ERROR),
                format!("Failed to decode image: {}", e),
            )
        })?;

    // Generate PDQ hash
    let (hash, quality) = generate_pdq(&img).ok_or_else(|| {
        Error::new(
            ruby.get_inner(&PDQ_ERROR),
            format!("Failed to generate PDQ hash"),
        )
    })?;

    // Create Ruby array with hash and quality
    let result = ruby.ary_new();
    result.push(hex::encode(hash)).unwrap();
    result.push(quality).unwrap();

    Ok(result)
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Pdq")?;
    module.define_singleton_method("pdq_hash", function!(pdq_hash, 1))?;
    Ok(())
}
