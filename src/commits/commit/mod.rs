use std::collections::HashSet;

use git2::{Oid, Repository, TreeWalkMode, TreeWalkResult};
use regex::Regex;

pub(super) struct Commit {
    oid: Oid,
    affects: HashSet<String>,
}

impl Commit {
    pub(super) fn from_git(repository: &Repository, oid: Oid) -> Result<Commit, git2::Error> {
        fn get_all_files_changed_in_commit(
            repository: &Repository,
            commit: &git2::Commit,
        ) -> Result<HashSet<String>, git2::Error> {
            let mut files = HashSet::new();

            match commit.tree() {
                Ok(commit_tree) => {
                    match commit.parent(0) {
                        Ok(parent) => {
                            // Some merge commits can have multiple parents, just take the first.
                            match parent.tree() {
                                Ok(parent_tree) => {
                                    trace!(
                                        "Using the commit '{}' as the parent for the commit '{}'.",
                                        parent.id(),
                                        commit.id()
                                    );
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
                                            return Err(error);
                                        }
                                    }
                                }
                                Err(error) => {
                                    error!("{:?}", error);
                                    return Err(error);
                                }
                            }
                        }
                        Err(_) => {
                            // Root Commit
                            match commit_tree.walk(TreeWalkMode::PostOrder, |directory, entry| {
                                match entry.name() {
                                    Some(name) => {
                                        let file = format!("{}{}", directory, name);
                                        files.insert(file);
                                    }
                                    None => {
                                        warn!(
                                            "The root commit with the hash '{}' has non valid UTF-8 files.",
                                            commit.id()
                                        )
                                    }
                                }
                                TreeWalkResult::Ok
                            }) {
                                Ok(_) => {}
                                Err(error) => {
                                    error!("{:?}", error);
                                    return Err(error);
                                }
                            }
                        }
                    }
                }

                Err(error) => {
                    error!("{:?}", error);
                    return Err(error);
                }
            }

            Ok(files)
        }

        match repository.find_commit(oid) {
            Ok(commit) => {
                let affects = get_all_files_changed_in_commit(repository, &commit)?;
                let oid = commit.id();

                debug!("Commit {:?} affects the files {:?}.", oid, affects);

                Ok(Commit { oid, affects })
            }
            Err(error) => {
                error!("Can not find commit with the hash '{}'.", oid);
                Err(error)
            }
        }
    }

    pub(super) fn is_affected(&self, regexes: &[Regex]) -> bool {
        for affected in self.affects.iter() {
            for regex in regexes {
                if regex.is_match(affected) {
                    info!(
                        "Commit {:?} affects the file {:?} which matches \"{:?}\".",
                        self.oid, affected, regex
                    );
                    return true;
                }
            }
        }

        false
    }

    pub(super) fn get_affected_resources(&self) -> &HashSet<String> {
        &self.affects
    }
}

#[cfg(test)]
mod tests;
