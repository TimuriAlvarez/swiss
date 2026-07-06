use crate::Res;

pub const JUST: &str = "just";
pub const GLOW: &str = "glow";

pub fn output(mut command: std::process::Command) -> std::io::Result::<std::process::Output> {
  command.output()
}

pub fn spawn(mut command: std::process::Command) -> std::io::Result::<std::process::Output> {
  let status: std::process::ExitStatus = command.spawn()?.wait()?;
  Ok(std::process::Output {
    status,
    stdout: Vec::new(),
    stderr: Vec::new(),
  })
}

#[allow(unused)]
pub struct Output {
  pub success: bool,
  pub stdout: String,
  pub stderr: String,
}

pub fn run(f: impl Fn(std::process::Command) -> std::io::Result::<std::process::Output>, program: &str, c_args: &[&str], temp_file: Option::<&temp_file::TempFile>, args: &[String]) -> Res::<Output> {
  let mut command: std::process::Command = std::process::Command::new(program);
  c_args.into_iter().for_each(|arg: &&str| { command.arg(arg); });
  if let Some(temp_file) = temp_file {
    command.arg(temp_file.path());
  }
  args.into_iter().for_each(|arg: &String| { command.arg(arg); });
  let output: std::process::Output = f(command)?;
  Ok(Output{
    success: output.status.success(),
    stdout: String::from_utf8(output.stdout).expect("Invalid UTF-8 stdout"),
    stderr: String::from_utf8(output.stderr).expect("Invalid UTF-8 stderr"),
  })
}
