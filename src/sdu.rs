use std::fs::File;
use anyhow::Result;
use std::path::PathBuf;
use clap::Parser;
use shin::format::rom::IndexEntry;

#[derive(clap::Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    action: SduAction,
}

#[derive(clap::Subcommand, Debug)]
enum SduAction {
    #[clap(subcommand)]
    Rom(RomCommand),
}

#[derive(clap::Subcommand, Debug)]
enum RomCommand {
    List {
        rom_path: PathBuf,
    },
    ExtractOne { // TODO: make a more generalized interface, maybe like tar or 7z
        rom_path: PathBuf,
        rom_filename: String,
        output_path: PathBuf,
    }
}

fn rom_command(command: RomCommand) -> Result<()> {
    match command {
        RomCommand::List { rom_path: path } => {
            let rom = File::open(path)?;
            let reader = shin::format::rom::RomReader::new(rom)?;
            for (name, entry) in reader.traverse() {
                let ty = match entry {
                    IndexEntry::File(_) => "FILE",
                    IndexEntry::Directory(_) => "DIR ",
                };
                println!("{} {}", ty, name);
            }
            Ok(())
        },
        RomCommand::ExtractOne {
            rom_path, rom_filename, output_path
        } => {
            use std::io::Read;
            let rom = File::open(rom_path)?;
            let mut reader = shin::format::rom::RomReader::new(rom)?;
            let file = reader.find_file(&rom_filename)?;
            let mut file = reader.open_file(file)?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            std::fs::write(output_path, buf)?;
            Ok(())
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.action {
        SduAction::Rom(cmd) => rom_command(cmd),
    }
}