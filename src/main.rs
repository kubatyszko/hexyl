#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::{self, prelude::*, StdoutLock};

use clap::{App, AppSettings, Arg};

const BUFFER_SIZE: usize = 64;

struct Printer<'a> {
    idx: usize,
    line: Vec<u8>,
    stdout: StdoutLock<'a>,
}

impl<'a> Printer<'a> {
    fn new(stdout: StdoutLock) -> Printer {
        Printer {
            idx: 1,
            line: vec![],
            stdout,
        }
    }

    fn print_byte(&mut self, b: u8) -> io::Result<()> {
        write!(self.stdout, "{:02x} ", b)?;
        self.line.push(b);

        match self.idx % 16 {
            8 => write!(self.stdout, " ")?,
            0 => {
                self.print_textline()?;
            }
            _ => {}
        }

        self.idx += 1;

        Ok(())
    }

    fn print_textline(&mut self) -> io::Result<()> {
        write!(self.stdout, "")?;
        writeln!(self.stdout);

        Ok(())
    }
}

fn run() -> io::Result<()> {
    let app = App::new(crate_name!())
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .version(crate_version!())
        .arg(Arg::with_name("file").help("to do").required(true));

    let matches = app.get_matches();

    let filename = matches.value_of("file").unwrap();

    let mut buffer = [0; BUFFER_SIZE];
    let mut file = File::open(filename)?;

    let stdout = io::stdout();
    let mut printer = Printer::new(stdout.lock());
    loop {
        let size = file.read(&mut buffer)?;
        if size == 0 {
            break;
        }

        for b in &buffer[..size] {
            let res = printer.print_byte(*b);

            if res.is_err() {
                // Broken pipe
                break;
            }
        }
    }

    // Finish last line
    printer.print_textline().ok();

    Ok(())
}

fn main() {
    let result = run();
    match result {
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
        Ok(()) => {}
    }
}
