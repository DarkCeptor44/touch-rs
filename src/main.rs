/**
 * touch: File-creating utility for Windows
 * Copyright (C) 2024 DarkCeptor44
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use clap::Parser;
use std::{fs::File, path::Path, process::exit};

#[derive(Parser)]
#[command(version,about,long_about=None)]
struct Args {
    #[arg(help = "Path to file")]
    file: String,

    #[arg(
        help = "Size in bytes or human-readable format (e.g., 1M, 2G)",
        default_value = ""
    )]
    size: String,
}

fn main() {
    let args = Args::parse();

    if args.file.is_empty() {
        println!("file cannot be empty");
        exit(1);
    }

    let f = match File::create_new(Path::new(&args.file)) {
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
        Ok(f) => f,
    };

    if args.size.is_empty() {
        return;
    }

    let size_bytes = match parse_size(&args.size) {
        Ok(size) => size,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    };

    if let Err(e) = f.set_len(size_bytes) {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn parse_size(size_str: &str) -> Result<u64, String> {
    let size_str = size_str.to_lowercase();

    if let Ok(size) = size_str.parse::<u64>() {
        return Ok(size);
    }

    let (size, suffix) = size_str.split_at(size_str.len() - 1);
    let size = size.parse::<u64>().map_err(|_| "Invalid size")?;

    match suffix {
        "k" | "kb" => Ok(size * 1024),
        "m" | "mb" => Ok(size * 1024 * 1024),
        "g" | "gb" => Ok(size * 1024 * 1024 * 1024),
        _ => Err("Invalid size suffix".to_string()),
    }
}
