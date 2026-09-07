use gprl::types::Res;

const USIZE_BYTES: usize = std::mem::size_of::<usize>();
const LENGTH_FIELD: usize = 1;
const EOF_OFFSET: usize = 1;

macro_rules! write {
  ($mmap:expr, $start:expr, $byte_size:expr, $value:expr) => {
    {
      $mmap[$start..$start + $byte_size].copy_from_slice($value);
      $byte_size
    }
  };
}

pub fn write_to_path<P: AsRef<std::path::Path>>(path: P, values: &[String]) -> Res {
  let length: usize = values.len();
  let mut file_size: usize = (LENGTH_FIELD + length + EOF_OFFSET) * USIZE_BYTES;
  let mut offsets: Vec<usize> = vec![0; length + EOF_OFFSET];
  (0..length + EOF_OFFSET).for_each(|i: usize| {
    offsets[i] = if i == 0 { file_size } else {
      let string_length: usize = values[i-1].as_bytes().len();
      file_size += string_length;
      offsets[i-1] + string_length
    }
  });
  // create or truncate the file
  let file: std::fs::File = std::fs::OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)?;
  // set file's length and map it
  file.set_len(file_size as u64)?;
  let mut mmap: memmap2::MmapMut = unsafe { memmap2::MmapMut::map_mut(&file)? };
  // write `length` to the file
  let mut cursor: usize = write!(mmap, 0, USIZE_BYTES, &length.to_ne_bytes());
  (0..length + EOF_OFFSET).for_each(|i: usize| {
    cursor += write!(mmap, cursor, USIZE_BYTES, &offsets[i].to_ne_bytes());
  });
  // write `values` to the file
  values.into_iter().for_each(|value: &String| {
    let value: &[u8] = value.as_bytes();
    let string_length: usize = value.len();
    cursor += write!(mmap, cursor, string_length, value);
  });
  Ok(())
}

fn file2mmap<P: AsRef<std::path::Path>>(path: P) -> Res<memmap2::Mmap> {
  // open the file
  let file: std::fs::File = std::fs::OpenOptions::new()
    .read(true)
    .write(false)
    .open(path)?;
  // map the file
  let mmap: memmap2::Mmap = unsafe { memmap2::Mmap::map(&file)? };
  Ok(mmap)
}

pub fn read_length<P: AsRef<std::path::Path>>(path: P) -> Res<usize> {
  let mmap: memmap2::Mmap = file2mmap(path)?;
  // read `length` from the file
  let length: usize = usize::from_ne_bytes(mmap[0..USIZE_BYTES].try_into()?);
  Ok(length)
}

pub fn read_value<P: AsRef<std::path::Path>>(path: P, index: usize) -> Res<String> {
  let mmap: memmap2::Mmap = file2mmap(path)?;
  // read `length` from the file
  let length: usize = usize::from_ne_bytes(mmap[0..USIZE_BYTES].try_into()?);
  // validate `index`
  if index >= length {
    anyhow::bail!("index is out of bounds")
  }
  // read `offsets` from the file
  let start: usize = (LENGTH_FIELD + index) * USIZE_BYTES;
  let (first, second): (&[u8], &[u8]) = mmap[start..start + 2 * USIZE_BYTES].split_at(USIZE_BYTES);
  let first: usize = usize::from_ne_bytes(first.try_into()?);
  let second: usize = usize::from_ne_bytes(second.try_into()?);
  // read `value` from the file
  Ok(String::from_utf8(mmap[first..second].to_vec())?)
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
      let length: usize = rand::random_range(..TEST_LENGTH);
      let values: Vec<String> = (0..length).map(|_: usize| {
        let size: usize = rand::random_range(..TEST_LENGTH);
        (&mut rng).sample_iter(distr).take(size).map(char::from).collect()
      }).collect();
      roundtrip(&values)?;
    }
    Ok(())
  }
}
