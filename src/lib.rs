// Copyright 2023 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! Cryptocol crate provides libraries for cryptography.
//! 
//! This crate is optimized for Little-endian CPUs
//! because Little-Endian CPUs are far more popular than Big-endian CPUs.
//! And, this crate is so experimental for Big-endian CPUs
//! that you are highy discouraged to use this crate for Big-endian CPUs
//! for serious purpose. So, use this crate for Big-endian CPUs with
//! your own full responsibility.
//! 

// #![doc(
//     html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
//     html_favicon_url = "https://www.rust-lang.org/favicon.ico",
//     html_root_url = "https://rust-random.github.io/rand/"
// )]

pub mod number;

//use number::*;
//pub use cryptocol_errors::*;



