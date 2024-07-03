mod cli;
mod key_value;
mod error;
mod replacer;

fn main() {
    match cli::start() {
        Ok(_) => {
            ()
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
