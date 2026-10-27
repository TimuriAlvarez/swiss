use gprl::types::Res;

#[derive(Debug)]
#[derive(derive_more::Display)]
#[display("{}{stdout}{}{}{stderr}",
  if !stdout.is_empty() { "stdout:\n"} else { "" },
  if !stdout.is_empty() && stderr.is_empty() { "\n" } else { "" },
  if !stderr.is_empty() { "stderr:\n"} else { "" },
)]
pub struct State {
  pub status: std::process::ExitStatus,
  pub stdout: String,
  pub stderr: String,
}

pub fn runner(shell: impl Fn(std::process::Command) -> std::io::Result<std::process::Output>, program: &str, args: &[&str]) -> Res<State> {
  let mut command: std::process::Command = std::process::Command::new(program);
  args.into_iter().for_each(|arg: &&str| { command.arg(arg); });
  let output: std::process::Output = shell(command)?;
  Ok(State{
    status: output.status,
    stdout: String::from_utf8(output.stdout)?,
    stderr: String::from_utf8(output.stderr)?,
  })
}

pub fn output(mut command: std::process::Command) -> std::io::Result<std::process::Output> {
  command.output()
}

pub fn spawn(mut command: std::process::Command) -> std::io::Result<std::process::Output> {
  Ok(std::process::Output {
    status: command.spawn()?.wait()?,
    stdout: Vec::new(),
    stderr: Vec::new(),
  })
}

pub fn propagate(state: State) -> Res {
  if state.status.success() { return Ok(()) }
  let result: String = state.to_string();
  if !result.is_empty() {
    println!("{result}");
  }
  let code: i32 = if let Some(code) = state.status.code() { code } else {
    anyhow::bail!("program was terminated by a signal")
  };
  std::process::exit(code);
}
