pub struct StringPadding {}
impl StringPadding {
    pub fn spaces_around(s : String, count : usize) -> String {
        return format!(
            "{}{}{}",
            " ".repeat(count),
            s,
            " ".repeat(count) 
        );
    }
}

