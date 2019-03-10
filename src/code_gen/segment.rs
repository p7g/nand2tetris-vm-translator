#[derive(Debug)]
pub enum Segment {
  Argument,
  Constant,
  Local,
  Pointer,
  Static(String),
  Temp,
  This,
  That,
}

impl std::fmt::Display for Segment {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", match self {
      Segment::Argument => "ARG",
      Segment::Constant => "CONST",
      Segment::Local => "LCL",
      Segment::Pointer => "POINTER",
      Segment::Static(filename) => filename,
      Segment::Temp => "TEMP",
      Segment::This => "THIS",
      Segment::That => "THAT",
    })
  }
}

impl Segment {
  pub fn from_name(name: &str, filename: &str) -> Result<Segment, String> {
    match name {
      "argument" => Ok(Segment::Argument),
      "constant" => Ok(Segment::Constant),
      "local" => Ok(Segment::Local),
      "pointer" => Ok(Segment::Pointer),
      "static" => Ok(Segment::Static(String::from(filename))),
      "temp" => Ok(Segment::Temp),
      "this" => Ok(Segment::This),
      "that" => Ok(Segment::That),
      _ => Err(format!("Invalid segment {}", name)),
    }
  }

  pub fn is_valid_name(name: &str) -> bool {
    match name {
      "argument" => true,
      "constant" => true,
      "local" => true,
      "pointer" => true,
      "static" => true,
      "temp" => true,
      "this" => true,
      "that" => true,
      _ => false,
    }
  }

  pub fn is_writable(&self) -> bool {
    match self {
      Segment::Constant => false,
      _ => true,
    }
  }

  pub fn resolve_address(&self, index: i16) -> String {
    match self {
      Segment::Constant => unreachable!(),
      Segment::Static(name) => format!("@{}.{}\n", name, index),
      Segment::This
      | Segment::That
      | Segment::Local
      | Segment::Argument => {
        if index == 0 {
          format!("
  @{}
  A=M
", self)
        } else {
          format!("\
  @{}
  D=A
  @{}
  A=D+M
", index, self)
        }
      },
      Segment::Pointer
      | Segment::Temp => {
        let base_address = if let Segment::Pointer = self {
          "THIS"
        } else {
          "5"
        };
        if index == 0 {
          format!("@{}\n", base_address)
        } else {
          format!("\
  @{}
  D=A
  @{}
  A=D+A
", base_address, index)
        }
      },
    }
  }
}
