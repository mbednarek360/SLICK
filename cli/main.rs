use std::env;
mod core;


// ----------------------------------------------------------------
// takes in flags and executes given core
fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1] as &str {
        "-e" => core::file_crypt(&args[2], &args[3], true),
        "-d" => core::file_crypt(&args[2], &args[3], false),
        "-k" => core::gen_key(&args[2], &args[3]),
        "-p" => core::permute(&args[2]),
        "-t" => core::test(&args[2], &args[3]),
        "-h" => core::help(),
        _ => core::error(),
    }
}
