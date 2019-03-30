use std::env;

mod tasks;

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1] as &str {
        //"-e" => tasks::file_crypt(&args[2], args[3], true),
        //"-d" => tasks::file_crypt(&args[2], args[3], false),
        "-k" => tasks::gen_key(&args[2]),
        //"-h" => tasks::help(),
        _ => tasks::error(),
    }
}
