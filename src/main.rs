fn main() {
    println!("{}", crane::get_buildinfo());
    println!("{:#?}", crane::image_metadata("nginx", None).unwrap());
}
