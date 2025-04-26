use {
    anyhow::{Result, anyhow},
    clap::{Parser, builder::Styles},
    clap_cargo::style::*,
    mtf::*,
    std::path::PathBuf,
};

const STYLES: Styles = Styles::styled()
    .header(HEADER)
    .usage(USAGE)
    .literal(LITERAL)
    .placeholder(PLACEHOLDER)
    .error(ERROR)
    .valid(VALID)
    .invalid(INVALID);

#[derive(Parser)]
#[command(about, version, max_term_width = 80, styles = STYLES)]
struct Args {
    /// Input file(s)
    #[arg(value_name = "PATH")]
    input_files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    for input_file in &args.input_files {
        match std::fs::read_to_string(input_file) {
            Ok(input) => {
                print!("{}", process(&input)?);
            }
            Err(e) => {
                return Err(anyhow!("Could not read `{}` ({e})", input_file.display()));
            }
        }
    }
    Ok(())
}
