fn main() {
    // get args from command line
    let args: Vec<String> = std::env::args().collect();
    // get the first argument
    let arg = &args[1];
    if arg == "hello" {
        println!("hello world");
    } else if arg == "run" {
        println!("run the program");
    }
}
