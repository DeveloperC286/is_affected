use crate::model::commits::commit::Commit;
use git2::{Oid, Repository, Revwalk};
use std::process::exit;

mod commit;

pub struct Commits {
    commits: Vec<Commit>,
}

impl Commits {
    pub fn from_git(from_commit_hash: String) -> Self {
        fn get_commits_till_head_from_oid(
            repository: &Repository,
            from_commit_hash: Oid,
        ) -> Commits {
            fn get_commit_revwalker(repository: &Repository, from_commit_hash: Oid) -> Revwalk {
                match repository.revwalk() {
                    Ok(mut commits) => {
                        match commits.push_head() {
                            Ok(_) => {}
                            Err(_) => {
                                error!("Unable to push HEAD onto the Git revision walker.");
                                exit(crate::ERROR_EXIT_CODE);
                            }
                        }

                        match commits.hide(from_commit_hash) {
                            Ok(_) => {}
                            Err(_) => {
                                error!(
                                    "Can not find commit hash '{}' on the Git revision walker.",
                                    from_commit_hash
                                );
                                exit(crate::ERROR_EXIT_CODE);
                            }
                        }

                        commits
                    }
                    Err(error) => {
                        error!("{:?}", error);
                        exit(crate::ERROR_EXIT_CODE);
                    }
                }
            }

            Commits {
                commits: get_commit_revwalker(repository, from_commit_hash)
                    .map(|oid| match oid {
                        Ok(oid) => Commit::from_git(repository, oid),
                        Err(error) => {
                            error!("{:?}", error);
                            exit(crate::ERROR_EXIT_CODE);
                        }
                    })
                    .collect(),
            }
        }

        fn get_repository() -> Repository {
            match Repository::open_from_env() {
                Ok(repository) => repository,
                Err(error) => {
                    error!("{:?}", error);
                    exit(crate::ERROR_EXIT_CODE);
                }
            }
        }

        fn parse_to_oid(repository: &Repository, oid: String) -> Oid {
            match oid.len() {
                0 => {
                    error!("Provided Git commit hash is empty and can not be parsed.");
                    exit(crate::ERROR_EXIT_CODE);
                }
                1..=39 => {
                    trace!(
                        "Attempting to find a match for the short commit hash {:?}",
                        oid
                    );
                    let matching_oid_lowercase = oid.to_lowercase();

                    match repository.revwalk() {
                        Ok(mut revwalk) => {
                            match revwalk.push_head() {
                                Ok(_) => {}
                                Err(_) => {
                                    error!("Unable to push HEAD onto the Git revision walker.");
                                    exit(crate::ERROR_EXIT_CODE);
                                }
                            }

                            let matched_commit_hashes: Vec<Oid> = revwalk
                                .into_iter()
                                .map(|result| {
                                    return match result {
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
                                    };
                                })
                                .flatten()
                                .collect();

                            match matched_commit_hashes.len() {
                                0 => {
                                    error!("No actual commit hashes start with the provided short commit hash {:?}.", matching_oid_lowercase);
                                    exit(crate::ERROR_EXIT_CODE);
                                }
                                1 => *matched_commit_hashes.first().unwrap(),
                                _ => {
                                    error!("Ambiguous short commit hash, the commit hashes {:?} all start with the provided short commit hash {:?}.", matched_commit_hashes, matching_oid_lowercase);
                                    exit(crate::ERROR_EXIT_CODE);
                                }
                            }
                        }
                        Err(error) => {
                            error!("{:?}", error);
                            exit(crate::ERROR_EXIT_CODE);
                        }
                    }
                }
                40 => match git2::Oid::from_str(&oid) {
                    Ok(oid) => oid,
                    Err(error) => {
                        error!("{:?}", error);
                        exit(crate::ERROR_EXIT_CODE);
                    }
                },
                _ => {
                    error!("Provided Git commit hash is too long and can not be parsed.");
                    exit(crate::ERROR_EXIT_CODE);
                }
            }
        }

        let repository = get_repository();

        get_commits_till_head_from_oid(&repository, parse_to_oid(&repository, from_commit_hash))
    }

    pub fn is_effected(&self, effects: &[String]) -> bool {
        let regexes = crate::utilities::regex::from(effects);

        for commit in self.commits.iter() {
            if commit.is_effected(&regexes) {
                return true;
            }
        }

        false
    }
}
