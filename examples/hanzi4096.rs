extern crate clap;
extern crate hanzi4096;

use std::io::{ self, Read, Write };
use std::fs::File;
use clap::{ Arg, App, ArgMatches };


fn app() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("input").value_name("INPUT"))
        .arg(Arg::with_name("output").short("o").long("output").value_name("OUTPUT").help("write to file"))
        .arg(Arg::with_name("decode").short("d").long("decode").help("decode mode").display_order(0))
        .arg(Arg::with_name("ignore").short("i").long("ignore").help("when decoding, ignore invalid char").display_order(1))
}

fn start(matches: ArgMatches<'static>) -> io::Result<()> {
    let mut input = Vec::new();
    if let Some(path) = matches.value_of("input") {
        File::open(path)?.read_to_end(&mut input)?;
    } else {
        io::stdin().read_to_end(&mut input)?;
    };

    let mut output = if let Some(path) = matches.value_of("output") {
        Box::new(File::create(path)?) as Box<Write>
    } else {
        Box::new(io::stdout()) as Box<Write>
    };

    if matches.occurrences_of("decode") == 0 {
        output.write_all(hanzi4096::encode(&input).as_bytes())?;
    } else if matches.occurrences_of("ignore") == 0 {
        let input = String::from_utf8(input)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
        output.write_all(&hanzi4096::decode(input.as_str())?)?;
    } else {
        let input = String::from_utf8_lossy(&input);
        output.write_all(&hanzi4096::decode_ignore(&input))?;
    }

    output.flush()
}

fn main() {
    start(app().get_matches()).unwrap();
}
