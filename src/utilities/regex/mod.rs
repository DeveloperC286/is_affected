use regex::Regex;
use std::process::exit;
use std::str::FromStr;

pub fn from(regexes: &[String]) -> Vec<Regex> {
    regexes
        .iter()
        .map(|regex| match Regex::from_str(regex) {
            Ok(regex) => regex,
            Err(error) => {
                error!("{:?}", error);
                error!("Unable to compile {:?} into a regex.", regex);
                exit(crate::ERROR_EXIT_CODE);
            }
        })
        .collect()
}
