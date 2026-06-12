use gprl::types::Res;
use tracing::{event, Level};

#[derive(derive_more::Display)]
#[display(rename_all = "lowercase")]
pub enum Program {
  Just,
  Glow,
  More,
}

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

pub fn run (f: impl Fn(std::process::Command) -> std::io::Result::<std::process::Output>, program: Program, c_args: &[&str], temp_file: Option::<String>, args: &[String]) -> Res::<String> {
  let mut command = std::process::Command::new(program.to_string());
  c_args.into_iter().for_each(|arg: &&str| { command.arg(arg); });
  let file: temp_file::TempFile = temp_file::with_contents(&temp_file.clone().unwrap_or_default().into_bytes());
  if temp_file.is_some() {
    command.arg(file.path());
  }
  args.into_iter().for_each(|arg: &String| { command.arg(arg); });
  let output: std::process::Output = f(command)?;
  let status: std::process::ExitStatus = output.status;
  let stdout: String = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
  let stderr: String = String::from_utf8(output.stderr).expect("Invalid UTF-8 output");
  if !status.success() || !stderr.is_empty() {
    let stderr: String = if stderr.is_empty() { stderr } else { format!(", stderr:\n{stderr}") };
    event!(Level::ERROR, "{program} executable {status}{stderr}");
  }
  Ok(stdout)
}
