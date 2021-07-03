use git2::{Oid, Repository, TreeWalkMode, TreeWalkResult};
use std::collections::HashSet;
use std::process::exit;

pub struct Commit {
    _affects: HashSet<String>,
}

impl Commit {
    pub fn from_git(repository: &Repository, oid: Oid) -> Self {
        fn get_all_files_changed_in_commit(
            repository: &Repository,
            commit: &git2::Commit,
        ) -> HashSet<String> {
            let mut files = HashSet::new();

            match commit.tree() {
                Ok(commit_tree) => {
                    if commit.parent_count() == 0 {
                        // Root Commit
                        commit_tree
                            .walk(TreeWalkMode::PostOrder, |directory, entry| {
                                match entry.name() {
                                    Some(name) => {
                                        let file = format!("{}{}", directory, name);
                                        files.insert(file);
                                    }
                                    None => {
                                        warn!(
                                            "Commit with the hash '{}' has not valid utf-8 files.",
                                            commit.id()
                                        )
                                    }
                                }
                                TreeWalkResult::Ok
                            })
                            .unwrap();
                    } else {
                        // Some merge commits can have multiple parents.
                        for parent in commit.parents() {
                            match parent.tree() {
                                Ok(parent_tree) => {
                                    match repository.diff_tree_to_tree(
                                        Some(&parent_tree),
                                        Some(&commit_tree),
                                        None,
                                    ) {
                                        Ok(diff) => {
                                            for delta in diff.deltas() {
                                                if let Some(new_file) = delta.new_file().path() {
                                                    files.insert(new_file.display().to_string());
                                                }

                                                if let Some(old_file) = delta.old_file().path() {
                                                    files.insert(old_file.display().to_string());
                                                }
                                            }
                                        }
                                        Err(error) => {
                                            error!("{:?}", error);
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
                    }
                }
                Err(error) => {
                    error!("{:?}", error);
                    exit(crate::ERROR_EXIT_CODE);
                }
            }

            files
        }

        match repository.find_commit(oid) {
            Ok(commit) => Commit {
                _affects: get_all_files_changed_in_commit(repository, &commit),
            },
            Err(error) => {
                error!("{:?}", error);
                error!("Can not find commit with the hash '{}'.", oid);
                exit(crate::ERROR_EXIT_CODE);
            }
        }
    }
}
