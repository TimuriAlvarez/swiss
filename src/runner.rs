use tracing::{event, Level};

#[derive(derive_more::Display)]
#[display(rename_all = "lowercase")]
pub enum Program {
  Just,
  Glow,
}

pub fn spawn(command: &mut std::process::Command) -> Result::<(std::process::ExitStatus, Option::<String>), std::io::Error>{
  command.spawn()?.wait().map(|status: std::process::ExitStatus| (status, None))
}

fn reduce(program: Program, c: Result::<(std::process::ExitStatus, Option::<String>), std::io::Error>) -> Result::<Option::<String>, std::io::Error> {
  c.map(|c: (std::process::ExitStatus, Option<String>)| {
    if !c.0.success() {
      event!(Level::ERROR, "{} failed with {}", program, c.0);
    }
    c.1
  })
}

pub fn run(program: Program, prefix: &[&str], temp_file: Option::<String>, args: &[String], f: impl Fn(&mut std::process::Command) -> Result<(std::process::ExitStatus, Option<String>), std::io::Error>) -> Result::<Option<String>, std::io::Error> {
  let tf: temp_file::TempFile = temp_file::with_contents(&temp_file.clone().unwrap_or_default().into_bytes());
  let mut command: std::process::Command = std::process::Command::new(program.to_string());
  prefix.into_iter().for_each(|arg: &&str| { command.arg(arg); });
  temp_file.is_some().then(|| { command.arg(tf.path()); });
  args.into_iter().for_each(|arg| { command.arg(arg); });
  reduce(program, f(&mut command))
}
