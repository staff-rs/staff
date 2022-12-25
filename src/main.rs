use clap::Parser as ClapParser;
use staff::{parse::Parser, render::staff::renderer::Renderer};

use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

#[derive(ClapParser)]
#[clap(author, version, about = "Music engraving command-line interface", long_about = None)]
struct App {
    path: PathBuf,
}

type Result = anyhow::Result<()>;

fn main() -> Result {
    let app = App::parse();
    let mut input = String::new();
    let mut file = File::open(app.path)?;
    file.read_to_string(&mut input)?;

    let renderer = Renderer::default();
    let mut parser = Parser::from(input.as_str());
    let staff = parser.staff(&renderer);
    let svg = renderer.render(&staff);

    let mut stdout = io::stdout().lock();
    svg::write(&mut stdout, &svg)?;
    writeln!(&mut stdout)?;

    Ok(())
}
