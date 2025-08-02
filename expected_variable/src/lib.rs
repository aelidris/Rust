use convert_case::{ Case, Casing };
use edit_distance::edit_distance;
pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    let compared = compared.to_lowercase();
    let expected = expected.to_lowercase();

    if !compared.is_case(Case::Camel) && !compared.is_case(Case::Snake) {
        return None;
    }

    let distance = edit_distance(&compared, &expected);

    let percentage = 100 - ((distance * 100) / expected.len()).min(100);
    if percentage >= 50 {
        Some(format!("{percentage}%"))
    } else {
        None
    }
}
