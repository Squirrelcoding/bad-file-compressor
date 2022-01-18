extern crate clap;
use clap::{App, ArgMatches, Arg};
mod m_compress;
mod compress;
mod decompress;

fn main() {
    // basic app information
    let app = App::new("bad_compressor")
        .version("1.0")
        .about("A non-binary file compressor which does a poor job at compressing data.")
        .author("Squirrelcoding")
        .subcommand(App::new("compress")
            .about("Compresses a file")
            .arg(
                Arg::new("input")
                .short('i')
                .long("input")
                .takes_value(true)
                .value_name("INPUT_FILE")
                .help("Input file to compress")
            )
            .arg(
                Arg::new("output")
                .short('o')
                .long("output")
                .takes_value(true)
                .value_name("OUTPUT_FILE")
                .help("The name of the output file")
            )
            .arg(
                Arg::new("multithreaded")
                .short('m')
                .long("multithreaded")
                .help("Use multiple threads to compress the file, this is usually only benificial for CPUs with more than 8 threads or for large files.")
            )
        )
            .subcommand(App::new("decompress")
            .about("Decompresses a file")
            .arg(
                Arg::new("input")
                .short('i')
                .long("input")
                .takes_value(true)
                .value_name("INPUT_FILE")
                .help("Input file to decompress")
            )
            .arg(
                Arg::new("output")
                .short('o')
                .long("output")
                .takes_value(true)
                .value_name("OUTPUT_FILE")
                .help("The file where the decompressed data will be written to")
            )
            .arg(
                Arg::new("json-key")
                .short('k')
                .long("json-key")
                .takes_value(true)
                .value_name("JSON_KEY")
                .help("The JSON key used to decompress the file")
            )
           );

    let matches = app.get_matches();
    run(matches);
}

fn run(matches: ArgMatches) {
    // (&str, &ArgMatches)
    match matches.subcommand().unwrap() {
        ("compress", args) => {
            if args.is_present("input") && args.is_present("output") && !args.is_present("multithreaded") {
                let input: Vec<_> = args.values_of("input").unwrap().collect();
                let output: Vec<_> = args.values_of("output").unwrap().collect();
                compress::compress(input[0], output[0]);
            }
            if args.is_present("input") && args.is_present("output") && args.is_present("multithreaded") {
                let input: Vec<_> = args.values_of("input").unwrap().collect();
                let output: Vec<_> = args.values_of("output").unwrap().collect();
                m_compress::compress(input[0], output[0]);
            }
        },
        ("decompress", args) => {
            if args.is_present("input") && args.is_present("output") && args.is_present("json-key") {
                let input: Vec<_> = args.values_of("input").unwrap().collect();
                let output: Vec<_> = args.values_of("output").unwrap().collect();
                let json: Vec<_> = args.values_of("json-key").unwrap().collect();
                decompress::decompress(input[0], output[0], json[0]);
            }
        },
        _ => {
            println!("Hello!");
        }
    }
}

