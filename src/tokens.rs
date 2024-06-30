use crate::utils::*;

pub static GROCERIES: &str = "Expenses:Groceries";
pub static ALCOHOL: &str = "Expenses:Alcohol";
pub static EATING_OUT: &str = "Expenses:Eating Out";

pub fn name_to_destination(name: &str) -> String {
    match name {
        "Iceland" |
        "Aldi" |
        "Ryman"
        => GROCERIES,

        "McTucky's"
        => EATING_OUT,

        "On Bar Manchester" |
        "Brewdog Man Doghouse"
        => ALCOHOL,

        _ => "Expenses:",
     }.to_string()
}

pub fn process_name(name: &str) -> String {
    let mut name_parts: Vec<String> = name.split(' ').filter(|&x| !x.is_empty())
        .map(|x| capitalize(&x.to_lowercase())).collect();
    name_parts.pop();
    name_parts.pop();

    let mut name = name_parts.join(" ");

    if name.starts_with("Crv*") {
        name = capitalize(&name[4..name.len()-7]);
    }

    match name.as_str() {
        "Mctuckys"
        => "McTucky's",

        _ => &name,
    }.to_string()
}
