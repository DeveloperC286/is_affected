use std::collections::VecDeque;
use std::str::FromStr;

use anyhow::{bail, Context, Result};
use git2::{Oid, Repository, Revwalk};
use regex::Regex;

use crate::commits::commit::Commit;

mod commit;

pub struct Commits {
    commits: VecDeque<Commit>,
}

impl Commits {
    pub fn from_git<T: AsRef<str>>(repository: &Repository, git: T) -> Result<Commits> {
        let oid = parse_to_oid(repository, git.as_ref()).or_else(|error| {
            get_reference_oid(repository, git.as_ref()).map_err(|e| error.context(e))
        })?;
        get_commits_till_head_from_oid(repository, oid)
    }

    pub fn is_affected<T: AsRef<str>>(&self, affects: &[T]) -> Result<bool> {
        fn to_regexes<T: AsRef<str>>(to_regexes: &[T]) -> Result<Vec<Regex>> {
            let mut regexes = vec![];

            for to_regex in to_regexes {
                let regex = Regex::from_str(to_regex.as_ref())?;
                regexes.push(regex);
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
) -> Result<Commits> {
    fn get_revwalker(repository: &Repository, from_commit_hash: Oid) -> Result<Revwalk> {
        let mut commits = repository.revwalk()?;
        commits.simplify_first_parent()?;
        commits.push_head()?;

        commits.hide(from_commit_hash).context(format!(
            "Can not find a commit with the hash '{}'.",
            from_commit_hash
        ))?;
        Ok(commits)
    }

    let revwalker = get_revwalker(repository, from_commit_hash)?;
    let mut commits = VecDeque::new();

    for commit in revwalker {
        let oid = commit?;

        let commit = Commit::from_git(repository, oid)
            .context(format!("Can not find a commit with the hash '{}'.", oid))?;
        commits.push_front(commit);
    }

    Ok(Commits { commits })
}

fn get_reference_oid(repository: &Repository, matching: &str) -> Result<Oid> {
    let reference = repository
        .resolve_reference_from_short_name(matching)
        .context(format!(
            "Could not find a reference with the name {:?}.",
            matching
        ))?;
    trace!(
        "Matched {:?} to the reference {:?}.",
        matching,
        reference.name().unwrap()
    );
    let commit = reference.peel_to_commit()?;
    Ok(commit.id())
}

fn parse_to_oid(repository: &Repository, oid: &str) -> Result<Oid> {
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
                    Err(_) => None,
                })
                .collect();

            match matched_commit_hashes.len() {
                0 => {
                    bail!(
                        "No commit hashes start with the provided short commit hash {:?}.",
                        matching_oid_lowercase
                    );
                }
                1 => Ok(*matched_commit_hashes.first().unwrap()),
                _ => {
                    bail!("Ambiguous short commit hash, the commit hashes {:?} all start with the provided short commit hash {:?}.", matched_commit_hashes, matching_oid_lowercase);
                }
            }
        }
        _ => git2::Oid::from_str(oid).context(format!("{:?} is not a valid commit hash.", oid)),
    }
}
