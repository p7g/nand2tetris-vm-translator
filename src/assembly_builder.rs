use std::io::{Write, Result};

#[derive(Debug)]
pub struct AssemblyBuilder {
  pub buffer: Vec<u8>,
  label_count: i32,
}

impl AssemblyBuilder {
  pub fn new() -> AssemblyBuilder {
    AssemblyBuilder {
      buffer: initial_asm().to_vec(),
      label_count: 0,
    }
  }

  pub fn write(&self, stream: &mut Write) -> Result<()> {
    stream.write_all(&self.buffer[..])
  }

  pub fn next_label_count(&mut self) -> i32 {
    self.label_count += 1;
    self.label_count
  }
}

macro_rules! new_label {
  ( $x:expr ) => {{
    let count = ($x).next_label_count();
    format!("__VM_GENERATED_{}", count)
  }};
}

macro_rules! label {
  ( $x:expr, $label:expr ) => {{
    write!(($x).buffer, "({})\n", $label).unwrap();
  }};
}

macro_rules! goto {
  ( $x:expr, $label:expr ) => {{
    write!(($x).buffer, "
  // GOTO {label}
  @{label}
  0;JMP
", label = $label).unwrap();
  }};
}

macro_rules! if_goto {
  ( $x:expr, $label:expr ) => {{
    pop_D!($x);
    write!(($x).buffer, "
  @{label}
  D;JNE
", label = $label).unwrap();
  }}
}

macro_rules! push {
  ( $a:expr, $segment:expr, $index:expr ) => {{
    write!(($a).buffer, "
  // PUSHING FROM {segment} {index}
  {address}
  D=M
  @SP
  A=M
  M=D
  @SP
  M=M+1
", address = ($segment).resolve_address($index)
 , segment = $segment
 , index = $index).unwrap();
  }}
}

macro_rules! push_constant {
  ( $a:expr, $value:expr ) => {{
    write!(($a).buffer, "
  // PUSHING CONSTANT {value}
  @{value}
  D=A
  @SP
  A=M
  M=D
  @SP
  M=M+1
", value = $value).unwrap();
  }}
}

macro_rules! pop {
  ( $a:expr, $segment:expr, $index:expr ) => {{
    write!(($a).buffer, "
  // POPPING INTO {segment} {index}
  {address}
  D=A
  @R13
  M=D
  @SP
  AM=M-1 // dereference and decrement SP at the same time
  D=M
  @R13
  A=M
  M=D
", address = ($segment).resolve_address($index)
 , segment = $segment
 , index = $index).unwrap();
  }}
}

macro_rules! pop_D {
  ( $a:expr ) => {{
    write!(($a).buffer, "
  // POPPING INTO D REGISTER
  @SP
  AM=M-1 // dereference and decrement SP at the same time
  D=M
").unwrap();
  }}
}

macro_rules! add {
  ( $a:expr ) => {{
    pop_D!($a); // pop top of stack into D register
    write!(($a).buffer, "
  // ADD
  @SP
  A=M-1
  M=D+M
").unwrap();
  }}
}

macro_rules! neg {
  ( $a:expr ) => {{
    write!(($a).buffer, "
  // NEG
  @SP
  A=M-1
  M=-M
").unwrap();
  }}
}

macro_rules! sub {
  ( $a:expr ) => {{
    pop_D!($a); // pop top of stack into D register
    write!(($a).buffer, "
  // SUB
  @SP
  A=M-1
  M=D-M
  M=-M // negate the result since it was subtracted backwards
").unwrap();
  }}
}

macro_rules! eq {
  ( $a:expr ) => {{
    let label = new_label!($a);
    pop_D!($a);
    write!(($a).buffer, "
  // EQ
  @SP
  A=M-1
  D=D-M
  @EQ_CALLBACK_{label}
  D;JNE // not the same, return false
  @SP
  A=M-1
  M=-1
  @EQ_END_{label}
  0;JMP
(EQ_CALLBACK_{label})
  @SP
  A=M-1
  M=0
(EQ_END_{label})
", label = label).unwrap();
  }}
}

macro_rules! gt {
  ( $a:expr ) => {{
    let label = new_label!($a);
    pop_D!($a);
    write!(($a).buffer, "
  // GT
  @SP
  A=M-1
  D=D-M
  @GT_CALLBACK_{label}
  D;JLT // not greater, return false
  @SP
  A=M-1
  M=0
  @GT_END_{label}
  0;JMP
(GT_CALLBACK_{label})
  @SP
  A=M-1
  M=-1
(GT_END_{label})
", label = label).unwrap();
  }}
}

macro_rules! lt {
  ( $a:expr ) => {{
    let label = new_label!($a);
    pop_D!($a);
    write!(($a).buffer, "
  // LT
  @SP
  A=M-1
  D=D-M
  @LT_CALLBACK_{label}
  D;JGT // not greater, return false
  @SP
  A=M-1
  M=0
  @LT_END_{label}
  0;JMP
(LT_CALLBACK_{label})
  @SP
  A=M-1
  M=-1
(LT_END_{label})
", label = label).unwrap();
  }}
}

macro_rules! and {
  ( $a:expr ) => {{
    pop_D!($a);
    write!(($a).buffer, "
  // AND
  @SP
  A=M-1
  M=D&M
").unwrap();
  }}
}

macro_rules! or {
  ( $a:expr ) => {{
    pop_D!($a);
    write!(($a).buffer, "
  // OR
  @SP
  A=M-1
  M=D|M
").unwrap();
  }}
}

macro_rules! not {
  ( $a:expr ) => {{
    write!(($a).buffer, "
  // NOT
  @SP
  A=M-1
  M=!M
").unwrap();
  }}
}

fn initial_asm() -> &'static [u8] {
  b"\
  // initialize stack pointer to 256
  @256
  D=A
  @SP
  M=D

  // PROGRAM START
"
}
