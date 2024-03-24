// Copyright 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::{ io, env, fs };
use std::io::BufRead;
use std::convert::From;
use cryptocol::hash::SHA2_512;

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 3
    {
        help();
        return;
    }

    let arg = &args[1][..];
    match arg
    {
        "--text" | "-t" =>  { get_hash_value_from_text(&args[2][..]); },
        "--file" | "-f" =>  { get_hash_value_from_file(&args[2][..]); },
        "--check" | "-c" => { check_files(&args[2][..]) },
        _ =>  { help(); },
    }
}

fn get_hash_value_from_text(txt: &str)
{
    let mut hash = SHA2_512::new();
    hash.digest_str(txt);
    println!("Hash value:\t{}", hash.get_hash_value_in_string());
}

fn get_hash_value_from_file(file: &str)
{
    if let Ok(contents) = fs::read(file)
    {
        let mut hash = SHA2_512::new();
        hash.digest_vec(&contents);
        println!("Hash value:\t{}", hash.get_hash_value_in_string());
    }
    else
    {
        println!("File Error!");
    }
}

fn check_files(file_list: &str)
{
    let mut reader;
    match fs::File::open(file_list)
    {
        Ok(file) => {
                reader = io::BufReader::new(file);
                let mut line = String::new();
                while let Ok(n) = reader.read_line(&mut line)
                {
                    if n == 0
                        { break; }
                    let txt = line.trim();
                    if txt.chars().nth(0).unwrap() == '#'
                    { 
                        line.clear();
                        continue;
                    }
                    let elem: Vec<&str> = txt.split_whitespace().collect();
                    let item = elem[0];
                    let h = String::from(elem[1]).to_uppercase();
                    if let Ok(contents) = fs::read(item)
                    {
                        let mut hash = SHA2_512::new();
                        hash.digest_vec(&contents);
                        if hash.to_string() == h
                            { println!("{} ---> OK", item); }
                        else
                            { println!("{} ---> Corrupted", item); }
                    }
                    line.clear();
                }
            },
        _ => {
                println!("File open error");
                return;
            }
    }
}

fn help()
{
    println!("This is an SHA2_512 hash value extractor from a text or a file, using cryptocol.");
    println!("Usage: sha2_512_app <option> <source>");
    println!("options       description");
    println!("--text, -t    : <source> is a text to get a hash code.");
    println!("                The text should be enclosed by ' or \".");
    println!("--file, -f    : <source> is the name of the file to get a hash code.");
    println!("--check, -c   : <source> is the name of the file that contains pairs");
    println!("                of file and its hash code.");
    println!("--help, -h    : print this help message on screen\n");
    println!("Examples:");
    println!("\tsha2_512_app -t 'How are you doing?'");
    println!("\tsha2_512_app -f linuxmint-21.3-cinnamon-64bit.iso");
    println!("\tsha2_512_app -c CHECKSUM");
}
