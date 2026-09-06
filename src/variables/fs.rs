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
