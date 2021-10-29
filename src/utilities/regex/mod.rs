use std::str::FromStr;

use regex::Regex;

pub(crate) fn from(to_regexes: &[String]) -> Result<Vec<Regex>, ()> {
    let mut regexes = vec![];

    for to_regex in to_regexes {
        match Regex::from_str(to_regex) {
            Ok(regex) => {
                regexes.push(regex);
            }
            Err(_) => {
                error!("Unable to compile {:?} into a regex.", to_regex);
                return Err(());
            }
        }
    }

    Ok(regexes)
}
