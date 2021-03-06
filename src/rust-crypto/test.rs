// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(asm)]
#![feature(macro_rules)]
#![feature(simd)]

extern crate serialize;
extern crate test;

pub mod aes;
pub mod aessafe;
pub mod bcrypt;
pub mod bcrypt_pbkdf;
pub mod blockmodes;
pub mod blowfish;
pub mod buffer;
pub mod chacha20;
mod cryptoutil;
pub mod digest;
pub mod ghash;
pub mod hmac;
pub mod mac;
pub mod md5;
pub mod pbkdf2;
pub mod poly1305;
pub mod ripemd160;
pub mod rc4;
pub mod salsa20;
pub mod scrypt;
pub mod sha1;
pub mod sha2;
pub mod symmetriccipher;
pub mod util;

#[cfg(target_arch = "x86")]
#[cfg(target_arch = "x86_64")]
pub mod aesni;
