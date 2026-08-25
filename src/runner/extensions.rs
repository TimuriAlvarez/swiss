use crate::Res;

fn enhance_functions(content: &str) -> Res::<String> {
    let mut content: String = content.to_string();
    // Extract declarations for user defined functions from justfile
    let declarations: String = crate::editor::editor(true, &content, r"(.*\(.*\))\s:=\s.*", "&1\n", &[])?;
    for old in declarations.lines().filter(|line: &&str| line.contains("=")) {
        // Make all declarations justfile-compatible
        let last: usize = old.len() - 1;
        let index: usize = old.find("=").expect("unable to obtain index");
        let new: String = format!("{}{}", &old[..index], &old[last..]);
        content = crate::editor::editor(false, &content, r"&1(.*)", &format!("{new}&1"), &[old.to_string()])?;
        // Expand missing arguments
        let name: &str = &old[..old.find('(').unwrap()];
        let value: &str = &old[index+1..last];
        let argc: usize = new.matches(',').count();
        let sep: &str = if argc == 0 { "" } else { ", " };
        let argv: String = if argc == 0 { String::new() } else {
            // A pattern that matches either a quoted string (which may contain commas) or an unquoted non-comma sequence
            vec![r#"(?:"[^"]*"|'[^']*'|[^,]+)"#; argc].join(r",\s")
        };
        let pattern: String = format!(r"(.*){name}\(({argv})\)(.*)");
        let replacement: String = format!("&1{name}(&2{sep}{value})&3");
        // Keep expanding until there is nothing left to expand anymore
        loop {
            let new_content: String = crate::editor::editor(false, &content, &pattern, &replacement, &[name.to_string()])?;
            if content == new_content { break }
            content = new_content;
        }
    }
    Ok(content)
}

pub fn apply(content: &str) -> Res::<String> {
    enhance_functions(content)
}
