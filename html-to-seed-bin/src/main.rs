use html_to_seed_lib::html_to_seed;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "html-to-seed")]
struct Opt {
    /// Convert stringly Tailwind classes to typed classes.
    #[structopt(short, long)]
    typed_classes: bool,

    /// Output file
    #[structopt(short = "o", long, parse(from_os_str))]
    output: Option<PathBuf>,

    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let mut input_file = std::fs::File::open(opt.file).unwrap();
    let mut html_buffer = String::new();
    match input_file.read_to_string(&mut html_buffer) {
        Ok(_) => {
            let seed_str = html_to_seed(&html_buffer, opt.typed_classes);

            if let Some(output_path) = opt.output {
                let mut output_file =
                    File::create(output_path).expect("create failed");
                output_file
                    .write_all(seed_str.as_bytes())
                    .expect("write failed");
            } else {
                print!("{}", &seed_str);
            };
        },
        Err(err) => {
            println!("{:#}", err);
        },
    }
}
