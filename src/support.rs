use regex::Regex;

pub fn strip_leading_dots(input: &str) -> &str {
    let re = Regex::new(r"^\.+(.*)$").unwrap();
    match re.captures(input) {
        Some(cap) => cap.get(1).unwrap().as_str(),
        None => input,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_leading_dots_works() {
        assert_eq!(strip_leading_dots("..abc"), "abc");
        assert_eq!(strip_leading_dots(".abc"), "abc");
        assert_eq!(strip_leading_dots(".abc."), "abc.");
        assert_eq!(strip_leading_dots("abc."), "abc.");
        assert_eq!(strip_leading_dots("abc"), "abc");
    }
}
