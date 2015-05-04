extern crate openssl;

use std::io::prelude::*;
use std::io::Result;
use openssl::crypto::hash::{ Hasher, Type };

pub struct HashReader<R> {
  inner: R,
  hasher: Hasher,
}

impl<R: Read> HashReader<R> {
  pub fn new(r: R, hasher: Hasher) -> HashReader<R> {
    HashReader { inner: r, hasher: hasher }
  }

  pub fn of(r: R, typ: Type) -> HashReader<R> {
    HashReader::new(r, Hasher::new(typ))
  }

  pub fn finish(mut self) -> Vec<u8> { self.hasher.finish() }
}

impl<R: Read> Read for HashReader<R> {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
    let amt = try!(self.inner.read(buf));
    let _ = self.hasher.write_all(&buf[..amt]);
    return Ok(amt)
  }
}


#[test]
fn it_works() {
}
