use crane::image_metadata;

fn main() {
    println!("{:#?}", image_metadata("nginx", None).unwrap());
}
