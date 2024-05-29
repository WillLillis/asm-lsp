// TODO: Move existing tests over to this crate, and then write new
// tests to make sure we can properly deserialize the converted docs
// in the main server
use ::asm_lsp::types::NameToInstructionMap;
use ::asm_lsp::x86_parser::{
    populate_instructions, populate_name_to_instruction_map, populate_name_to_register_map,
    populate_registers,
};
use asm_lsp::NameToRegisterMap;

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, clap::ValueEnum)]
enum DocType {
    Instruction,
    Register,
    Directive,
}

#[derive(Parser, Debug)]
struct SerializeDocs {
    #[arg(long, short, help = "Path to the input file")]
    input_files: Vec<PathBuf>,
    #[arg(long, short, help = "Path to store the output file")]
    output_path: Option<PathBuf>,
    #[arg(
        long,
        short,
        help = "Serialize input as Instructions, Registers, or Directives"
    )]
    doc_type: DocType,
}

#[derive(Subcommand)]
#[command(about = "Parse and serialize assembly-related documentation")]
enum Commands {
    SerializeDocs(SerializeDocs),
}


// Big ol' TODO:
//
// Since we're serializing this, we can't store references to the various
// instructions and registers in the struct. We will need to instead clone
// each struct as we place it into the map, and update all the code that touches
// these maps

fn run(opts: &SerializeDocs) -> Result<()> {
    match opts.doc_type {
        DocType::Instruction => {
            let mut map = NameToInstructionMap::new();

            for input_path in opts.input_files.iter() {
                let path = input_path.canonicalize()?;
                let conts = std::fs::read_to_string(path)?;
                // NOTE: Will have to do some sort of arch detection here
                // once we support multiple (e.g. once ARM or Z80 are supported)
                let instrs = populate_instructions(&conts)?;
                // For now we'll assume all instructions out of a single file share
                // a common architecture
                let arch = if let Some(instr) = instrs.first() {
                    if let Some(arch) = instr.arch {
                        arch
                    } else {
                        return Err(anyhow!(
                            "Failed to determine architecture -- Empty 'arch' field"
                        ));
                    }
                } else {
                    return Err(anyhow!(
                        "Failed to determine architecture -- Zero instructions read in"
                    ));
                };
                populate_name_to_instruction_map(arch, &instrs, &mut map);
            }

            let serialized = serde_json::to_string(&map)?;
            let output_path = if let Some(ref path) = opts.output_path {
                path.canonicalize()?
            } else {
                format!("instructions_ser").into()
            };
            std::fs::write(output_path, serialized)?;
        }
        DocType::Register => {
            let mut map = NameToRegisterMap::new();

            for input_path in opts.input_files.iter() {
                let path = input_path.canonicalize()?;
                let conts = std::fs::read_to_string(path)?;
                // NOTE: Will have to do some sort of arch detection here
                // once we support multiple (e.g. once ARM or Z80 are supported)
                let regs = populate_registers(&conts)?;
                // For now we'll assume all instructions out of a single file share
                // a common architecture
                let arch = if let Some(instr) = regs.first() {
                    if let Some(arch) = instr.arch {
                        arch
                    } else {
                        return Err(anyhow!(
                            "Failed to determine architecture -- Empty 'arch' field"
                        ));
                    }
                } else {
                    return Err(anyhow!(
                        "Failed to determine architecture -- Zero instructions read in"
                    ));
                };
                populate_name_to_register_map(arch, &regs, &mut map);
            }

            let serialized = serde_json::to_string(&map)?;
            let output_path = if let Some(ref path) = opts.output_path {
                path.canonicalize()?
            } else {
                format!("registers_ser").into()
            };
            std::fs::write(output_path, serialized)?;
        }
        DocType::Directive => {
            // TODO: Can implement this once https://github.com/bergercookie/asm-lsp/pull/57 gets
            // merged
            todo!()
        }
    }
    Ok(())
}

fn main() {
    let opts = SerializeDocs::parse();
    if let Err(e) = run(&opts) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
