//qas::qas!("main.c");

fn main() {
    //println!("{}", add(2, 4))
    let path = "main.c";
    let format = qas::transform::Format::detect(path);
    let mut code = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{:?}: {}", e, path));
    qas::transform::uncomment(&mut code, format);
    println!("{}", qas::transform::start(qas::token::parse(code.chars()), format));
}
