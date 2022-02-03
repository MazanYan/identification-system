mod file;
mod bar_code;
mod tests;

fn main() {
    let code = bar_code::bar_code(145992).unwrap();
    let image = file::build_image(&code);

    println!("bar code: {:?}", code);

    match image.save("bar-code.png") {
        Err(msg) => println!("error on saving image {}", msg),
        _ => println!("image saved successfully"),
    }
}
