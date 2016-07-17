extern crate getopts;
use getopts::Options;
use std::env;
use std::process;

fn archive() {

}

fn extract() {

}

fn remove() {

}

fn help() {

}

fn table_of_contents() {

}


fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

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
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    }
    else {
        println!("Usage: rooster -a <archive_name>.roo <files_to_archive>");
        process::exit(1);
    };
}
