use gprl::types::Res;
use std::io::{Read, Seek, BufWriter, Write};

const USIZE: usize = std::mem::size_of::<usize>();

pub fn write_to_path<P: AsRef<std::path::Path>>(path: P, values: &[String]) -> Res {
  let mut file: BufWriter<std::fs::File> = BufWriter::new(std::fs::File::create(path)?);
  let length: usize = values.len();
  let mut offsets: Vec<usize> = vec![0; length];
  for i in 0..length {
    offsets[i] = if i == 0 {
      (1 + length) * USIZE
    } else {
      let value: &[u8] = values[i-1].as_bytes();
      offsets[i-1] + USIZE + value.len()
    };
  }
  file.write_all(&length.to_ne_bytes())?;
  for i in 0..length {
    file.write_all(&offsets[i].to_ne_bytes())?;
  }
  for i in 0..length {
    let value: &[u8] = values[i].as_bytes();
    file.write_all(&value.len().to_ne_bytes())?;
    file.write_all(value)?;
  }
  file.flush()?;
  Ok(())
}

pub fn read_length<P: AsRef<std::path::Path>>(path: P) -> Res<usize> {
  let mut file: std::fs::File = std::fs::File::open(path)?;
  let mut buf: [u8; USIZE] = [0u8; USIZE];
  file.read_exact(&mut buf)?;
  Ok(usize::from_ne_bytes(buf))
}

pub fn read_value<P: AsRef<std::path::Path>>(path: P, index: usize) -> Res<String> {
  let mut file: std::fs::File = std::fs::File::open(path)?;
  let mut buf: [u8; USIZE] = [0; USIZE];
  file.read_exact(&mut buf)?;
  let length: usize = usize::from_ne_bytes(buf);
  if index >= length {
    anyhow::bail!("index is out of bounds")
  }
  if index != 0 {
    file.seek(std::io::SeekFrom::Start(((1 + index) * USIZE) as u64))?;
  }
  file.read_exact(&mut buf)?;
  let offset: usize = usize::from_ne_bytes(buf);
  file.seek(std::io::SeekFrom::Start(offset as u64))?;
  file.read_exact(&mut buf)?;
  let size: usize = usize::from_ne_bytes(buf);
  let mut buf: Vec<u8> = vec![0; size];
  file.read_exact(&mut buf)?;
  Ok(String::from_utf8(buf)?)
}

#[cfg(test)]
mod tests {
  use rand::RngExt;

use super::*;
  const TEST_LENGTH: usize = 1000;

  fn assert_roundtrip(path: &std::path::Path, values: &[String]) -> Res {
    assert_eq!(read_length(path)?, values.len(), "length mismatch");
    for (i, expected) in values.iter().enumerate() {
      assert_eq!(&read_value(path, i)?, expected, "index {i} mismatch");
    }
    Ok(())
  }

  fn roundtrip(values: &[String]) -> Res {
    let temp_file: temp_file::TempFile = temp_file::empty();
    write_to_path(temp_file.path(), values)?;
    assert_roundtrip(temp_file.path(), values)
  }

  #[test]
  fn empty_list() -> Res {
    roundtrip(&[])
  }

  #[test]
  fn empty_strings() -> Res {
    roundtrip(&[String::new()])?;
    roundtrip(&[const { String::new() }; TEST_LENGTH])
  }

  #[test]
  fn large_values() -> Res {
    roundtrip(&[
      "x".repeat(TEST_LENGTH),
      "y".repeat(TEST_LENGTH),
      String::new(),
      "z".repeat(TEST_LENGTH),
    ])
  }

  #[test]
  fn random_roundtrip() -> Res {
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let distr: rand::distr::Alphanumeric = rand::distr::Alphanumeric;
    for _ in 0..TEST_LENGTH {
      let length: usize = rand::random_range(0..TEST_LENGTH);
      let values: Vec<String> = (0..length).map(|_: usize| {
        let size: usize = rand::random_range(0..TEST_LENGTH);
        (&mut rng).sample_iter(distr).take(size).map(char::from).collect()
      }).collect();
      roundtrip(&values)?;
    }
    Ok(())
  }
}
