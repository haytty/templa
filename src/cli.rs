use clap::{Parser};
use crate::error::Error;
use crate::key_value::KeyValue;
use crate::replacer::Replacer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    template: String,

    #[arg(short, long)]
    config: Option<String>,

    #[arg()]
    key_value_args: Option<Vec<String>>,
}

fn get_key_value(args: &Args) -> Result<KeyValue, Error> {
    let mut key_value_from_config = match args.config.as_ref() {
        Some(config_path) => {
            KeyValue::from_config(config_path)?
        }
        None => {
            KeyValue::empty()
        }
    };

    let key_value_from_args = match args.key_value_args.as_ref() {
        Some(key_value_args) => {
            KeyValue::from_vec_string(key_value_args)
        }
        None => {
            KeyValue::empty()
        }
    };

    key_value_from_config.merge(&key_value_from_args);
    let key_value = key_value_from_config;
    println!("{:?}", key_value);

    match key_value.is_empty() {
        true => Err(Error::EmptyKeyValueError),
        _ => Ok(key_value)
    }
}

pub fn start() -> Result<(), Error> {
    let args = Args::parse();

    let key_value = get_key_value(&args)?;

    let replacer = Replacer::from_file(&args.template, key_value)?;

    let replaced_content = replacer.replace();

    println!("{}", replaced_content);

    Ok(())
}