use magnus::{function, prelude::*, Error, Ruby, RString};

/*let image = image::load_from_memory(bytes).unwrap();

c.bench_function("load_bridge", |b| {
    b.iter(|| dwn_pdq::generate_pdq_full_size(&image))
});*/

fn pdq_hash(image_path: RString) -> String {
    let _path = image_path.to_string();
    // Load the image, run PDQ, return the hash as a hex string
    // Placeholder for actual logic
    "fake_pdq_hash".to_string()
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Pdq")?;
    module.define_singleton_method("pdq_hash", function!(pdq_hash, 1))?;
    Ok(())
}