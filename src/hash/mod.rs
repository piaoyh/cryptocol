// Copyright 2023, 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! various cryptographic hash functions
//! 
//! # Introduction
//! The module that contains a few sub-modules
//! to define cryptographic hash functions
//! 
//! # Background: Cryptographic hash functions
//! What if we can make a kind of finger print of data and the finger print is
//! of small size? Then, we can identify data with that small-sized finger
//! print. And, by means of the finger print of the data, we can also determine
//! whether or not certain data has been changed. The finger print of data with
//! small size must be very useful. The output of the cryptographic hash
//! function is called hash value and can virtually play the role of the finger
//! print of data.
//! 
//! Cryptographic hash functions are hash algorithms that have three properties:
//! - Pre-image resistance;
//! - Second pre-image resistance; and
//! - Collision resistance.
//! 
//! Read [this article](https://en.wikipedia.org/wiki/Cryptographic_hash_function)
//! and/or Watch [this lecture](https://www.youtube.com/watch?v=bDIc3jcLlOE)
//! or [this lecture](https://www.youtube.com/watch?v=tLkHk__-M6Q)
//! to learn cryptographic hash functions more in detail. However, the last
//! video lecture has a synch problem that the video and its audio are not
//! synchronized, though this lecture is so good that I would like to highly
//! encourage you to watch even with suffering from mis-synchronization.
//! 
//! # The algorithms of cryptographic hash functions
//! This module provides several kinds of cryptographic hash algorithms:
//! - MD4 hash algorithms based on 128 bits --- Includes MD4 and its expanded versions. [`MD4_Generic`](struct@MD4_Generic)
//! - MD5 hash algorithms based on 128 bits --- Includes MD5 and its expanded versions. [`MD5_Generic`](struct@MD5_Generic)
//! - SHA-1 hash algorithms based on 160 bits --- Includes SHA-1, SHA-0, and their expanded versions. [`SHA1_Generic`](struct@SHA1_Generic)
//! - SHA-2 hash algorithms based on 256 bits --- Includes SHA-256, SHA-224, and their expanded versions. [`SHA2_Generic_256`](struct@SHA2_256_Generic)
//! - SHA-2 hash algorithms based on 512 bits --- Includes SHA-512, SHA-384, SHA-512/256, and their expanded versions. [`SHA2_512_Generic`](struct@SHA2_512_Generic)
//! - SHA-2 hash algorithms based on 512/t bits --- Includes 512/256, SHA-512/224, and their expanded versions. [`SHA2_512_t_Generic`](struct@SHA2_512_t_Generic)
//! 
//! # QUICK START
//! - For `MD4`, read [here](struct@MD4_Generic#quick-start).
//! - For `MD5`, read [here](struct@MD5_Generic#quick-start).
//! - For `SHA-1`, read [here](struct@SHA1_Generic#quick-start).
//! - For `SHA-256`, read [here](struct@SHA2_256_Generic#quick-start).
//! - For `SHA-512`, read [here](struct@SHA2_512_Generic#quick-start).
//! - For `SHA-512/t`, read [here](struct@SHA2_512_t_Generic#quick-start).
//! 
//! # Simple but Useful Applications using cryptocol
//! - For `MD4`, try [this](struct@MD4_Generic#a-simple-but-useful-application-using-cryptocol).
//! - For `MD5`, try [this](struct@MD5_Generic#a-simple-but-useful-application-using-cryptocol).
//! - For `SHA-1`, try [this](struct@SHA1_Generic#a-simple-but-useful-application-using-cryptocol).
//! - For `SHA-256`, try [this](struct@SHA2_256_Generic#a-simple-but-useful-application-using-cryptocol).
//! - For `SHA-512`, try [this](struct@SHA2_512_Generic#a-simple-but-useful-application-using-cryptocol).
//! - For `SHA-512/t`, try [this](struct@SHA2_512_t_Generic#a-simple-but-useful-application-using-cryptocol).
//! 
//! The following application is made by combining above all applications.
//! 
//! ## A Simple but Useful Integrated Hash Application using cryptocol
//! The following is the source code of the commandline integrated hash value
//! extractor using the above modules. You can get the hash value from a text
//! or a file. The following source code assumes its executable file name will
//! be "hash_app". You can find all the examples including the following source
//! code in the folder "examples" of this crate if you download this crate from
//! [github](https://github.com/piaoyh/cryptocol).
//! ```
//! use std::{ io, env, fs };
//! use std::io::BufRead;
//! use std::convert::From;
//! use std::fmt::{ Debug, Display };
//! use cryptocol::number::SmallUInt;
//! use cryptocol::hash::*;
//! 
//! fn main()
//! {
//!     let args: Vec<String> = env::args().collect();
//!     if args.len() < 4  
//!     {
//!         help();
//!         return;
//!     }
//! 
//!     match &args[1][..]
//!     {
//!         "md4" => { print_hash_value(&args[2][..], MD4::new(), &args[3][..]); },
//!         "md5" => { print_hash_value(&args[2][..], MD5::new(), &args[3][..]); },
//!         "sha1" => { print_hash_value(&args[2][..], SHA1::new(), &args[3][..]); },
//!         "sha2_256" => { print_hash_value(&args[2][..], SHA2_256::new(), &args[3][..]); },
//!         "sha2_224" => { print_hash_value(&args[2][..], SHA2_224::new(), &args[3][..]); },
//!         "sha2_512" => { print_hash_value(&args[2][..], SHA2_512::new(), &args[3][..]); },
//!         "sha2_512_256" => { print_hash_value(&args[2][..], SHA2_512_256::new(), &args[3][..]); },
//!         "sha2_384" => { print_hash_value(&args[2][..], SHA2_384::new(), &args[3][..]); },
//!         "sha2_512_t_224" => { print_hash_value(&args[2][..], SHA2_512_t_224::new(), &args[3][..]); },
//!         _ => { help(); },
//!     }
//! }
//! 
//! fn print_hash_value<H: Hash>(text_or_file: &str, hash: H, src: &str)
//! {
//!     match text_or_file
//!     {
//!         "--text" | "-t" =>  { get_hash_value_from_text(hash, src); },
//!         "--file" | "-f" =>  { get_hash_value_from_file(hash, src); },
//!         "--check" | "-c" => { check_files(hash, src) },
//!         _ =>  { help(); },
//!     }
//! }
//! 
//! fn get_hash_value_from_text<H: Hash>(mut hash: H, txt: &str)
//! {
//!     hash.digest_str(txt);
//!     println!("Hash value:\t{}", hash.get_hash_value_in_string());
//! }
//! 
//! fn get_hash_value_from_file<H: Hash>(mut hash: H, file: &str)
//! {
//!     if let Ok(contents) = fs::read(file)
//!     {
//!         hash.digest_vec(&contents);
//!         println!("Hash value:\t{}", hash.get_hash_value_in_string());
//!     }
//!     else
//!     {
//!         println!("File Error!");
//!     }
//! }
//! 
//! fn check_files<H: Hash>(mut hash: H, file_list: &str)
//! {
//!     let mut reader;
//!     match fs::File::open(file_list)
//!     {
//!         Ok(file) => {
//!                 reader = io::BufReader::new(file);
//!                 let mut line = String::new();
//!                 while let Ok(n) = reader.read_line(&mut line)
//!                 {
//!                     if n == 0
//!                         { break; }
//!                     let txt = line.trim();
//!                     if txt.chars().nth(0).unwrap() == '#'
//!                     { 
//!                         line.clear();
//!                         continue;
//!                     }
//!                     let elem: Vec<&str> = txt.split_whitespace().collect();
//!                     let item = elem[0];
//!                     let h = String::from(elem[1]).to_uppercase();
//!                     if let Ok(contents) = fs::read(item)
//!                     {
//!                         hash.digest_vec(&contents);
//!                         if hash.get_hash_value_in_string() == h
//!                             { println!("{} ---> OK", item); }
//!                         else
//!                             { println!("{} ---> Corrupted", item); }
//!                     }
//!                     line.clear();
//!                 }
//!             },
//!         _ => {
//!                 println!("File open error");
//!                 return;
//!             }
//!     }
//! }
//! 
//! fn help()
//! {
//!     println!("This is a hash value extractor from a text or a file, using cryptocol.");
//!     println!("Usage: hash_app <algorithm> <option> <source>\n");
//!     println!("algorithms        description");
//!     println!("md4               : MD4 algorithm");
//!     println!("md5               : MD5 algorithm");
//!     println!("sha1              : SHA1 algorithm");
//!     println!("sha2_256          : SHA2_256 algorithm");
//!     println!("sha2_224          : SHA2_224 algorithm");
//!     println!("sha2_512          : SHA2_512 algorithm");
//!     println!("sha2_512_256      : SHA2_512_256 algorithm");
//!     println!("sha2_384          : SHA2_384 algorithm");
//!     println!("sha2_512_t_224    : SHA2_512_t_224 algorithm\n");
//!     println!("options           description");
//!     println!("--text, -t        : <source> is a text to get a hash code.");
//!     println!("                    The text should be enclosed by ' or \".");
//!     println!("--file, -f        : <source> is the name of the file to get a hash code.");
//!     println!("--check, -c       : <source> is the name of the file that contains pairs");
//!     println!("                    of file and its hash code.");
//!     println!("--help, -h        : print this help message on screen\n");
//!     println!("Examples:");
//!     println!("\thash_app sha2_256 -t 'How are you doing?'");
//!     println!("\thash_app sha2_384 -f linuxmint-21.3-cinnamon-64bit.iso");
//!     println!("\thash_app sha2_512 -c CHECKSUM");
//! }
//! 
//! trait Hash
//! {
//!     fn digest_str(&mut self, message: &str);
//!     fn digest_vec<T>(&mut self, message: &Vec<T>) where T: SmallUInt + Copy + Clone + Display + Debug + ToString;
//!     fn get_hash_value_in_string(&self) -> String;
//! }
//! 
//! macro_rules! impl_hash_for
//! {
//!     ($f:ty) => {
//!         impl Hash for $f
//!         {
//!             #[inline] fn digest_str(&mut self, message: &str)   { self.digest_str(message); }
//!             #[inline] fn digest_vec<T>(&mut self, message: &Vec<T>) where T: SmallUInt + Copy + Clone + Display + Debug + ToString
//!                         { self.digest_vec(message); }
//!             #[inline] fn get_hash_value_in_string(&self) -> String  { self.get_hash_value_in_string() }
//!         }
//!     };
//! }
//! 
//! impl_hash_for!{MD4}
//! impl_hash_for!{MD5}
//! impl_hash_for!{SHA1}
//! impl_hash_for!{SHA2_224}
//! impl_hash_for!{SHA2_256}
//! impl_hash_for!{SHA2_384}
//! impl_hash_for!{SHA2_512}
//! impl_hash_for!{SHA2_512_256}
//! impl_hash_for!{SHA2_512_t_224}
//! ```

mod md4;
mod md5;
mod sha1;
mod sha2_256;
mod sha2_512;
mod sha2_512_t;

pub use md4::*;
pub use md5::*;
pub use sha1::*;
pub use sha2_256::*;
pub use sha2_512::*;
pub use sha2_512_t::*;
