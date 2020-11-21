use clap::arg_enum;
use mozlz4::{compress, decompress};
use std::io::{Write};
use std::path::PathBuf;
use structopt::StructOpt;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APPNAME: &'static str = env!("CARGO_PKG_NAME");

arg_enum! {
    #[derive(Debug, Clone, Copy)]
    enum ColorOutput {
        Never,
        Always,
        Ansi,
        Auto
    }
}

/// Decompresses and compresses mozlz4 files found in Firefox
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = APPNAME, version=VERSION)]
struct Opt {
    /// Colorize output
    #[structopt(possible_values = &ColorOutput::variants(), case_insensitive = true, long )]
    color: Option<ColorOutput>,

    /// Compress instead of decompress the input (true/false)
    #[structopt(short, long, parse(try_from_str))]
    compress: Option<bool>,

    /// The file to convert
    #[structopt(name = "INPUT", parse(from_os_str))]
    input: PathBuf,
    /// The desired output file
    #[structopt(name = "OUPUT", parse(from_os_str))]
    output: PathBuf,
}

impl From<ColorOutput> for ColorChoice {
    fn from(colorcfg: ColorOutput) -> Self {
        match colorcfg {
            ColorOutput::Always => ColorChoice::Always,
            ColorOutput::Ansi => ColorChoice::AlwaysAnsi,
            ColorOutput::Auto => ColorChoice::Auto,
            ColorOutput::Never => ColorChoice::Never,
        }
    }
}

fn main() -> std::io::Result<()> {
    let opt: Opt = Opt::from_args();

    let colormode = if opt.color.is_some() {
        match opt.color.unwrap() {
            ColorOutput::Auto => {
                if atty::is(atty::Stream::Stdout) {
                    ColorOutput::Auto
                } else {
                    ColorOutput::Never
                }
            }

            c => c,
        }
    } else {
        ColorOutput::Auto
    };

    let mut stdout = StandardStream::stdout(colormode.into());
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(122, 218, 204))))?;

    let input_disp = opt.input.clone().to_owned();
    let output_disp = opt.output.clone().to_owned();

    writeln!(
        &mut stdout,
        "Decrypting molz4 file: {:?} to {:?}",
        &input_disp, &output_disp
    )?;

    writeln!(&mut stdout, "{} v{}", APPNAME, VERSION)?;

    let input_path: PathBuf = match opt.input.canonicalize() {
        Ok(r) => r,
        Err(ie) => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            writeln!(&mut stdout, "Failed to canonicalize input. {}", ie)?;
            panic!("Convertion failed")
        }
    };

    let output_path: PathBuf = opt.output;
    let input_data = match std::fs::read(input_path.clone()) {
        Ok(r) => r,
        Err(e) => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            writeln!(&mut stdout, "Failed to read input. {}", e)?;
            panic!("Convertion failed")
        }
    };

    if opt.compress.is_some() && opt.compress.unwrap() == true {
        let compressed = match compress(input_data) {
            Ok(r) => r,
            Err(e) => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                writeln!(&mut stdout, "Failed to compress input. {}", e)?;
                panic!("Convertion failed")
            }
        };
        std::fs::write(output_path.clone(), compressed)?;
    } else {
        let decompressed = match decompress(input_data) {
            Ok(r) => r,
            Err(e) => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                writeln!(&mut stdout, "Failed to decompress input. {}", e)?;
                panic!("Convertion failed")
            }
        };

        std::fs::write(output_path.clone(), decompressed)?;
    }
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(
        &mut stdout,
        "{:?} successfully converted to {:?}",
        &input_disp, &output_disp
    )?;
    Ok(())
}
