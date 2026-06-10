use gprl::types::Res;

fn test(mode: &str, input: &str, id: &str, pat: &str, rep: &str, lit: &[&str]) -> Res {
  let haystack: String = std::fs::read_to_string(format!("./tests/data/{input}.txt"))?;
  let pattern: String = pat.to_string();
  let replacement: String = rep.to_string();
  let literals: Vec::<String> = lit.into_iter().map(|lit: &&str| lit.to_string()).collect();
  let result: String = swiss::editor::editor(mode, &haystack, &pattern, &replacement, &literals)?;
  let expected: String = std::fs::read_to_string(format!("./tests/data/{input}/{id}.txt"))?;
  assert_eq!(result, expected, "{id}");
  Ok(())
}

#[test]
fn full_match() -> Res {
  test("Line", "matches", "a1", r"baa aaa", r"ccc ccc", &[])?;
  test("Line", "matches", "a2", r"abb aba", r"ccc ccc", &[])?;
  test("Line", "matches", "a3", r"aab bab", r"ccc ccc", &[])?;
  test("Line", "matches", "a4", r"aab bab\n", r"ccc ccc", &[])?;
  test("Line", "matches", "b1", r"baa aaa", r"ccc\nccc", &[])?;
  test("Line", "matches", "b2", r"abb aba", r"ccc\nccc", &[])?;
  test("Line", "matches", "b3", r"aab bab", r"ccc\nccc", &[])?;
  test("Line", "matches", "b4", r"aab bab\n", r"ccc\nccc", &[])?;
  test("Line", "matches", "c1", r"baa aaa\nabb aba", r"ccc\nccc", &[])?;
  test("Line", "matches", "c2", r"abb aba\naaa bbb", r"ccc\nccc", &[])?;
  test("Line", "matches", "c3", r"bbb abb\naab bab", r"ccc\nccc", &[])?;
  test("Line", "matches", "c4", r"bbb abb\naab bab\n", r"ccc\nccc", &[])?;
  Ok(())
}

#[test]
fn partial_match() -> Res {
  test("Line", "matches", "d1", r".*aaa.*", r"ccc ccc", &[])?;
  test("Line", "matches", "d2", r".*aba.*", r"ccc ccc", &[])?;
  test("Line", "matches", "d3", r".*bab.*", r"ccc ccc", &[])?;
  test("Line", "matches", "d4", r".*bab.*\n.*", r"ccc ccc", &[])?;
  test("Line", "matches", "e1", r"(.*)baa(.*)", r"&1ccc&2", &[])?;
  test("Line", "matches", "e2", r"(.*) (.*)", r"&1_&2", &[])?;
  test("Line", "matches", "e3", r"(.*)bab(.*)", r"&1ccc&2", &[])?;
  test("Line", "matches", "e4", r"(.*)bab(.*)\n(.*)", r"&1ccc&2&3", &[])?;
  Ok(())
}

#[test]
fn literals() -> Res {
  test("Line", "literals", "a1", r"&1", r"ccc.ccc", &["abc.abc"])?;
  test("Word", "literals", "a2", r"&1", r"dict", &["val"])?;
  test("Line", "literals", "a3", r"(&1)", r"&1\n&1", &["abc.abc"])?;
  test("Line", "literals", "a4", r"&1.&1", r"&0\n&0", &["abc"])?;
  test("Line", "literals", "a5", r"(&1 =).*", r"&1 255 # swiss &0", &["val.a"])?;
  test("Line", "literals", "a6", r".* # swiss (&1.*)", r"&1", &["val.b"])?;
  Ok(())
}
