extern crate getopts;
use getopts::Options;
use std::env;
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    /*Options available
     * -a archive file(s) specified
     * -e extract file(s) from archive specified
     * -d Remove files from archive
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


}
