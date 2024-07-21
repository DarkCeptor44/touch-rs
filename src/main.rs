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

    #[arg(short, long, default_value_t = 0)]
    size: u64,
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

    if let Err(e) = f.set_len(args.size) {
        eprintln!("Error: {}", e);
        exit(1);
    }
}
