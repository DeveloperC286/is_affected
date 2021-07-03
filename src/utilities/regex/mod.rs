use regex::Regex;
use std::str::FromStr;
use std::process::exit;

pub fn from(regexes: &[String]) -> Vec<Regex> {
    regexes.iter().map(|regex| match Regex::from_str(regex) {
        Ok(regex) => regex,
        Err(error) => {
            error!("{:?}", error);
            error!("Unable to compile {:?} into a regex.", regex);
            exit(crate::ERROR_EXIT_CODE);
        }
    }
    ).collect()
}