#[cfg(test)]
mod test;
mod core;
use clap::clap_app;

#[cfg(not(test))] 
fn main() {

    // parse cli args
    let matches = clap_app!(SLICK2 =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_REPOSITORY"))

        (@subcommand enc =>
            (about: "Encrypt a file with key or password")
            (@arg KEYFILE: -k --key +takes_value "Provide key file instead of prompting for password")
            (@arg INPUT: +required "Sets the file to encrypt")
            (@arg OUTPUT: +required "Sets the location of the output file")
        )

        (@subcommand dec =>
            (about: "Decrypt a file with key or password")
            (@arg KEY: -k --key +takes_value "Provide key file instead of prompting for password")
            (@arg INPUT: +required "Sets the file to decrypt")
            (@arg OUTPUT: +required "Sets the location of the output file")
        )

        (@subcommand key =>
            (about: "Generate a random or seeded key")
            (@arg AUTOMATIC: -a --automatic "Do not prompt for password and use random seed")
            (@arg OUTPUT: +required "Sets the output file")
        )
    ).get_matches();

    // ex: generate random key
    if let Some(matches) = matches.subcommand_matches("key") {
        println!("{:?}", core::key::generate(None));        
    }
}
