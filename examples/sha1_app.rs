// Copyright 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::{ env, fs };
use cryptocol::hash::SHA1;

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
        _ =>  { help(); },
    }
}

fn get_hash_value_from_text(txt: &str)
{
    let mut hash = SHA1::new();
    hash.digest_str(txt);
    println!("Hash value:\t{}", hash.get_hash_value_in_string());
}

fn get_hash_value_from_file(file: &str)
{
    if let Ok(contents) = fs::read(file)
    {
        let mut hash = SHA1::new();
        hash.digest_vec(&contents);
        println!("Hash value:\t{}", hash.get_hash_value_in_string());
    }
    else
    {
        println!("File Error!");
    }
}

fn help()
{
    println!("This is an SHA1 hash value extractor from a text or a file, using cryptocol.");
    println!("Usage: sha1_app <option> <source>");
    println!("options       description");
    println!("--text, -t    : <source> is given in text just after this option");
    println!("                In this case, <source> is a text.");
    println!("                The text should be enclosed by ' or \".");
    println!("--file, -f    : <source> is given from a file the name of which is");
    println!("                given just after this option");
    println!("                In this case, <source> is a file name.");
    println!("                If <source> is a file name without a path, the file");
    println!("                will be found in the current directory.");
    println!("                If <source> is a file name with a path, the file");
    println!("                will be found in the directory of the path.");
    println!("                The path can be either full path or relative path.");
    println!("--help, -h    : print this help message on screen\n");
    println!("Examples:");
    println!("\tsha1_app -t 'How are you doing?'");
    println!("\tsha1_app -f linuxmint-21.3-cinnamon-64bit.iso");
}
