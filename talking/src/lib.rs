pub fn talking(text: &str) -> &str {
    if text.is_empty() {
        return "Just say something!";
    }

    let is_yelling = all_letters_upper(text);
    let is_question = text.contains('?');

    match (is_yelling, is_question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        (false, false) => "Interesting",
    }
}

pub fn all_letters_upper(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_uppercase() {
            return false;
        }
    }
    return true;
}
