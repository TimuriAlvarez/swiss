use tracing::{Level, event};

#[derive(derive_more::Display)]
pub enum Program {
  Just,
  Glow,
}

pub struct Process<'a> {
  program: Program,
  com_args: Vec::<String>,
  temp_file: Option::<&'a temp_file::TempFile>,
}

impl<'a> Process<'a> {
  pub fn new(program: Program, args: &[&str], temp_file: Option::<&'a temp_file::TempFile>) -> Self {
    Self {
      program,
      com_args: args.into_iter().map(|arg: &&str| arg.to_string()).collect(),
      temp_file,
    }
  }
  fn build(self, args: &[String]) -> std::process::Command {
    let mut process: std::process::Command = std::process::Command::new(self.program.to_string().to_lowercase());
    self.com_args.into_iter().for_each(|arg: String| { process.arg(arg); });
    if let Some(temp_file) = self.temp_file {
      process.arg(temp_file.path());
    }
    args.into_iter().for_each(|arg: &String| { process.arg(arg); });
    process
  }
  pub fn spawn(self, args: &[String]) -> Result::<(), std::io::Error> {
    let mut command: std::process::Command = self.build(args);
    command.spawn()?.wait().map(|output: std::process::ExitStatus| {
      if !output.success() {
        event!(Level::ERROR, "error code: {:?}", output.code())
      }
      ()
    })
  }
  pub fn output(self, args: &[String]) -> Result::<(bool, Option::<String>), std::io::Error> {
    self.build(args).output().map(|output: std::process::Output| (output.status.success(), Some(String::from_utf8(output.stdout).expect("Failed to unwrap stdout"))) )
  }
}
