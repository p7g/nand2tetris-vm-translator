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
  ( $x:expr, $function_name:expr, $label:expr ) => {{
    write!(($x).buffer, "({}${})\n", $function_name, $label).unwrap();
  }};
}

macro_rules! vm_goto {
  ( $a:expr, $label:expr ) => {{
    write!(($a).buffer, "
  // GOTO FUNCTION {label}
  @{label}
  0;JMP
", label = $label).unwrap();
  }}
}

macro_rules! goto {
  ( $a:expr, $function_name:expr, $label:expr ) => {{
    vm_goto!($a, format!("{}${}", $function_name, $label));
  }};
}

macro_rules! if_goto {
  ( $x:expr, $function_name:expr, $label:expr ) => {{
    pop_D!($x);
    write!(($x).buffer, "
  // IF-GOTO {function_name}${label}
  @{function_name}${label}
  D;JNE
", label = $label
 , function_name = $function_name).unwrap();
  }}
}

macro_rules! push {
  ( $a:expr, $segment:expr, $index:expr ) => {{
    write!(($a).buffer, "
  // PUSHING FROM {segment} {index}
  {address}
  D=M
", address = ($segment).resolve_address($index)
 , segment = $segment
 , index = $index).unwrap();
    push_D!($a);
  }}
}

macro_rules! push_D {
  ( $a:expr ) => {{
    write!(($a).buffer, "
  // PUSHING FROM D REGISTER
  @SP
  A=M
  M=D
  @SP
  M=M+1
").unwrap();
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

macro_rules! push_address {
  ( $a:expr, $segment:expr ) => {{
    write!(($a).buffer, "
  @{segment}
  D=A
", segment = $segment).unwrap();
    push_D!($a);
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

macro_rules! vm_label {
  ( $a:expr, $function_name:expr ) => {{
    write!(($a).buffer, "({})\n", $function_name).unwrap();
  }}
}

macro_rules! function {
  ( $a:expr, $name:expr, $num_vars:expr ) => {{
    vm_label!($a, $name);
    for _ in 0..($num_vars) {
      push_constant!($a, 0);
    }
  }}
}

macro_rules! call {
  ( $a:expr, $function_name:expr, $num_args:expr ) => {{
    let label = new_label!($a);
    write!(($a).buffer, "
  @{}
  D=A
", label).unwrap();
    push_D!($a);
    // store addresses of caller's segments
    push_address!($a, Segment::Local);
    push_address!($a, Segment::Argument);
    push_address!($a, Segment::This);
    push_address!($a, Segment::That);
    // reposition arg segment
    push_address!($a, Segment::Argument);
    push_constant!($a, ($num_args) + 5);
    sub!($a);
    pop_D!($a);
    write!(($a).buffer, "
  @{}
  M=D
", Segment::Argument).unwrap();
    // reposition local pointer
    write!(($a).buffer, "
  @SP
  D=A
  @{local}
  M=D
", local = Segment::Local).unwrap();
    vm_goto!($a, $function_name);
    vm_label!($a, label);
  }}
}

macro_rules! return_ {
  ( $a:expr ) => {{
    unimplemented!();
  }}
}

fn initial_asm() -> &'static [u8] {
  b"\
  // initialize stack pointer to 256
  @256
  D=A
  @SP
  M=D

  // PROGRAM START"
}
