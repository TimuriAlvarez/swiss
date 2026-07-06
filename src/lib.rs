pub type Res<T=()> = anyhow::Result::<T>;

#[cfg(feature = "editor")]
pub mod editor;

#[cfg(feature = "runner")]
pub mod runner;

#[cfg(feature = "variable")]
pub mod variable;
