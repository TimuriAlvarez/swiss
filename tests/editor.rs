use gprl::types::Res;

fn test(input: &str, id: &str, extract: bool, pat: &str, rep: &str, lit: &[&str]) -> Res {
  let haystack: String = std::fs::read_to_string(format!("./tests/data/{input}.txt"))?;
  let pattern: String = pat.to_string();
  let replacement: String = rep.to_string();
  let literals: Vec::<String> = lit.into_iter().map(|lit: &&str| lit.to_string()).collect();
  let result: String = swiss::editor::editor(extract, &haystack, &pattern, &replacement, &literals)?;
  let expected: String = std::fs::read_to_string(format!("./tests/data/{input}/{id}.txt"))?;
  assert_eq!(result, expected, "{id}");
  Ok(())
}

#[test]
fn full_match() -> Res {
  test("matches", "a1", false, r"baa aaa", r"ccc ccc", &[])?;
  test("matches", "a2", false, r"abb aba", r"ccc ccc", &[])?;
  test("matches", "a3", false, r"aab bab", r"ccc ccc", &[])?;
  test("matches", "a4", false, r"aab bab\n", r"ccc ccc", &[])?;
  test("matches", "b1", false, r"baa aaa", r"ccc\nccc", &[])?;
  test("matches", "b2", false, r"abb aba", r"ccc\nccc", &[])?;
  test("matches", "b3", false, r"aab bab", r"ccc\nccc", &[])?;
  test("matches", "b4", false, r"aab bab\n", r"ccc\nccc", &[])?;
  test("matches", "c1", false, r"baa aaa\nabb aba", r"ccc\nccc", &[])?;
  test("matches", "c2", false, r"abb aba\naaa bbb", r"ccc\nccc", &[])?;
  test("matches", "c3", false, r"bbb abb\naab bab", r"ccc\nccc", &[])?;
  test("matches", "c4", false, r"bbb abb\naab bab\n", r"ccc\nccc", &[])?;
  Ok(())
}

#[test]
fn partial_match() -> Res {
  test("matches", "d1", false, r".*aaa.*", r"ccc ccc", &[])?;
  test("matches", "d2", false, r".*aba.*", r"ccc ccc", &[])?;
  test("matches", "d3", false, r".*bab.*", r"ccc ccc", &[])?;
  test("matches", "d4", false, r".*bab.*\n.*", r"ccc ccc", &[])?;
  test("matches", "e1", false, r"(.*)baa(.*)", r"&1ccc&2", &[])?;
  test("matches", "e2", false, r"(.*) (.*)", r"&1_&2", &[])?;
  test("matches", "e3", false, r"(.*)bab(.*)", r"&1ccc&2", &[])?;
  test("matches", "e4", false, r"(.*)bab(.*)\n(.*)", r"&1ccc&2&3", &[])?;
  Ok(())
}

#[test]
fn literals() -> Res {
  test("literals", "a1", false, r"&1", r"ccc.ccc", &["abc.abc"])?;
  test("literals", "a2", false, r"\<&1\>", r"dict", &["val"])?;
  test("literals", "a3", false, r"(&1)", r"&1\n&1", &["abc.abc"])?;
  test("literals", "a4", false, r"&1.&1", r"&0\n&0", &["abc"])?;
  test("literals", "a5", false, r"(&1 =).*", r"&1 255 # swiss &0", &["val.a"])?;
  test("literals", "a6", false, r".* # swiss (&1.*)", r"&1", &["val.b"])?;
  Ok(())
}

#[test]
fn extract() -> Res {
  test("extract", "a1", true, r"&1", r"&0\n", &["efg"])?;
  test("extract", "a2", true, r"&1", r"&0\n", &["abcd"])?;
  test("extract", "a3", true, r"&1(.*)", r"&1\n", &["1234"])?;
  Ok(())
}
