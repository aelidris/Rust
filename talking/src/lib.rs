pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }

    let is_yelling = all_letters_upper(text);
    let is_question = text.trim_end().ends_with('?');

    match (is_yelling, is_question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        (false, false) => "Interesting",
    }
}

pub fn all_letters_upper(s: &str) -> bool {
    let letters = s.chars().filter(|c| c.is_alphabetic()).collect::<Vec<_>>();
    !letters.is_empty() && letters.iter().all(|c| c.is_uppercase())
}