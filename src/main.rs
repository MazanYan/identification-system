mod code;
mod tests;

fn main() {
    println!("bar code: {:?}", code::bar_code(348165).unwrap());
}
