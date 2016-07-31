extern crate getopts;
use getopts::Options;
use std::env;
use std::io::prelude::*;
use std::io::{Cursor, SeekFrom};
use std::fs::File;
use std::process;
use std::str;

//Rooster header
static ROO_HEADER: &'static str = "!<rooster>\n";
//Rooster header byte length
const ROO_HEAD_LEN: usize = 11;

fn check_header<R: Read + Seek>(mut input: R) -> Result<bool,std::io::Error>{

    //Create a buffer to read in the header
    let mut buffer = [0; ROO_HEAD_LEN];
    //Seek to start of file
    try!(input.seek(SeekFrom::Start(0)));
    //Read in exactly length of header bytes
    try!(input.read_exact(&mut buffer));
    //convert buffer to str
    let header = unsafe { str::from_utf8_unchecked(&buffer[..]) };

    println!("Buffer: {}", header);
    //If the header exists and is correct
    Ok(ROO_HEADER == header)
}

fn archive(file_name: &str) {

    //Create the archive and write the header to the top
    let mut archive_file = File::create(file_name).unwrap();
    let result = archive_file.write(ROO_HEADER.as_bytes());

    //Open and check to make sure the header wrote sucessfully
    match File::open(file_name) {
        Ok(mut archive_file) => {
        	//match check_header(Cursor::new(&archive_file[..])) {
            match check_header(&mut archive_file) {
                Ok(ok) => { 
                    if ok { println!("Header wrote sucessfully"); }
                    else {
                        println!("Writing header failed. Aborting.");
                        process::exit(1);
                    }
                },
                Err(err) => {
                    println!("I/O error while checking header: {}", err);
                    process::exit(1);
                }
            } },
        Err(err) => {
            println!("Error opening test archive \"{}\": {}", file_name, err);
            process::exit(1);
            }
    }
}

/*
fn extract() {

}

fn remove() {

}

fn help() {

}

fn table_of_contents() {

}
*/

fn main() {

    let args: Vec<String> = env::args().collect();
    //let program = args[0].clone();

    let mut opts = Options::new();
    /*Options available
     * -a archive file(s) or folders specified
     * -e extract file(s) or folders from archive specified
     * -d Remove file(s) or folders from archive
     * -o overwrite files when extracting (e must be specified)
     * -h Displays help
     * -t Short table of contents for archive
     * -T Long table of contents for archive
     * -V version info
     * -v verbose mode
    */

    //Opt Flag Options
    opts.optflag("V", "Version", "Version 0.1.0");
    opts.optflag("h", "help", "Prints this help menu");

    //Opt opt Options
    opts.optopt("a", "archive", "Archive the given files", "Usage: rooster -a <archive_name>.roo <files_to_archive>");


    //Block to ensure the option exists
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    //Version output
    if matches.opt_present("V") {
        let brief = format!("Version 0.1.0");
        println!("{}", brief);
        process::exit(1);
    }
    if matches.opt_present("h") {
        let brief = format!("Help info");
        println!("{}", brief);
        process::exit(1);
    }

    let output = matches.opt_str("a");
    if !matches.free.is_empty() {
        archive("test.roo");
    }
    else {
        println!("Usage: rooster -a <archive_name>.roo <files_to_archive>");
        process::exit(1);
    };
}
