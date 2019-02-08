use std::io::{Write, Result};

use super::code_gen::segment::Segment;

#[derive(Debug)]
pub struct AssemblyBuilder {
  buffer: Vec<u8>,
}

impl AssemblyBuilder {
  pub fn new() -> AssemblyBuilder {
    AssemblyBuilder {
      buffer: initial_asm().to_vec(),
    }
  }

  pub fn write(&self, stream: &mut Write) -> Result<()> {
    stream.write_all(&self.buffer[..])
  }

  fn access_segment_at_i(&mut self, segment: Segment, i: usize) {
    write!(
      self.buffer,
      "\
@{}
D=A
@{}
D=D+A
",
      segment, i
    )
    .unwrap();
  }
}

fn initial_asm() -> &'static [u8] {
  b"\
// initialize stack pointer
@256
D=A
@SP
M=D
"
}
