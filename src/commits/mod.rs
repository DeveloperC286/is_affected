use std::collections::VecDeque;
use std::str::FromStr;

use git2::{Oid, Repository, Revwalk};
use regex::Regex;

use crate::commits::commit::Commit;

mod commit;

/// A representation of a range of commits within a Git repository.
pub struct Commits {
    commits: VecDeque<Commit>,
}

impl Commits {
    pub fn from_reference<T: AsRef<str>>(
        repository: &Repository,
        reference: T,
    ) -> Result<Commits, git2::Error> {
        let reference_oid = get_reference_oid(repository, reference.as_ref())?;
        get_commits_till_head_from_oid(repository, reference_oid)
    }

    pub fn from_commit_hash<T: AsRef<str>>(
        repository: &Repository,
        commit_hash: T,
    ) -> Result<Commits, git2::Error> {
        let commit_oid = parse_to_oid(repository, commit_hash.as_ref())?;
        get_commits_till_head_from_oid(repository, commit_oid)
    }

    pub fn is_affected<T: AsRef<str>>(&self, affects: &[T]) -> Result<bool, regex::Error> {
        fn to_regexes<T: AsRef<str>>(to_regexes: &[T]) -> Result<Vec<Regex>, regex::Error> {
            let mut regexes = vec![];

            for to_regex in to_regexes {
                match Regex::from_str(to_regex.as_ref()) {
                    Ok(regex) => {
                        regexes.push(regex);
                    }
                    Err(error) => {
                        error!("{:?}", error);
                        return Err(error);
                    }
                }
            }

            Ok(regexes)
        }
        let regexes = to_regexes(affects)?;

        for commit in self.commits.iter() {
            if commit.is_affected(&regexes) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn get_affected_resources(&self) -> Vec<String> {
        let mut affected_resources: Vec<String> = self
            .commits
            .iter()
            .flat_map(|commit| commit.get_affected_resources())
            .cloned()
            .collect();

        affected_resources.sort();
        affected_resources.dedup();
        affected_resources
    }
}

fn get_commits_till_head_from_oid(
    repository: &Repository,
    from_commit_hash: Oid,
) -> Result<Commits, git2::Error> {
    fn get_revwalker(
        repository: &Repository,
        from_commit_hash: Oid,
    ) -> Result<Revwalk, git2::Error> {
        let mut commits = repository.revwalk()?;
        commits.simplify_first_parent()?;
        commits.push_head()?;

        match commits.hide(from_commit_hash) {
            Ok(_) => Ok(commits),
            Err(error) => {
                error!(
                    "Can not find a commit with the hash '{}'.",
                    from_commit_hash
                );
                Err(error)
            }
        }
    }

    let revwalker = get_revwalker(repository, from_commit_hash)?;
    let mut commits = VecDeque::new();

    for commit in revwalker {
        let oid = commit?;

        match Commit::from_git(repository, oid) {
            Ok(commit) => commits.push_front(commit),
            Err(error) => {
                error!("Can not find a commit with the hash '{}'.", oid);
                return Err(error);
            }
        }
    }

    Ok(Commits { commits })
}

fn get_reference_oid(repository: &Repository, matching: &str) -> Result<Oid, git2::Error> {
    match repository.resolve_reference_from_short_name(matching) {
        Ok(reference) => {
            trace!(
                "Matched {:?} to the reference {:?}.",
                matching,
                reference.name().unwrap()
            );
            let commit = reference.peel_to_commit()?;
            Ok(commit.id())
        }
        Err(error) => {
            error!("Could not find a reference with the name {:?}.", matching);
            Err(error)
        }
    }
}

fn parse_to_oid(repository: &Repository, oid: &str) -> Result<Oid, git2::Error> {
    match oid.len() {
        1..=39 => {
            trace!(
                "Attempting to find a match for the short commit hash {:?}.",
                oid
            );
            let matching_oid_lowercase = oid.to_lowercase();

            let mut revwalker = repository.revwalk()?;
            revwalker.push_head()?;

            let matched_commit_hashes: Vec<Oid> = revwalker
                .filter_map(|result| match result {
                    Ok(oid) => {
                        let oid_lowercase = oid.to_string().to_lowercase();

                        if oid_lowercase.starts_with(&matching_oid_lowercase) {
                            return Some(oid);
                        }

                        None
                    }
                    Err(error) => {
                        error!("{:?}", error);

                        None
                    }
                })
                .collect();

            match matched_commit_hashes.len() {
                0 => {
                    let error_message = format!(
                        "No commit hashes start with the provided short commit hash {:?}.",
                        matching_oid_lowercase
                    );
                    error!("{}", error_message);
                    Err(git2::Error::from_str(&error_message))
                }
                1 => Ok(*matched_commit_hashes.first().unwrap()),
                _ => {
                    let error_message = format!("Ambiguous short commit hash, the commit hashes {:?} all start with the provided short commit hash {:?}.", matched_commit_hashes, matching_oid_lowercase);
                    error!("{}", error_message);
                    Err(git2::Error::from_str(&error_message))
                }
            }
        }
        _ => match git2::Oid::from_str(oid) {
            Ok(oid) => Ok(oid),
            Err(error) => {
                error!("{:?} is not a valid commit hash.", oid);
                Err(error)
            }
        },
    }
}
