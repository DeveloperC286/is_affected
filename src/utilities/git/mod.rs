use git2::Repository;

pub fn get_current_directory_prefix(repository: &Repository) -> Result<String, ()> {
    let mut repository_path = repository.path().to_path_buf();
    // Removing the ".git/" at the end.
    repository_path.pop();

    match std::env::current_dir() {
        Ok(current_directory) => match current_directory.strip_prefix(repository_path) {
            Ok(stripped) => match stripped.to_str() {
                Some(stripped) => match stripped.len() {
                    0 => {
                        error!("The current directory prefix is empty.");
                        Err(())
                    }
                    _ => Ok(format!("^{}/", stripped)),
                },
                None => {
                    error!("Can not convert the current directory prefix into a string.");
                    Err(())
                }
            },
            Err(_) => {
                error!("Can not strip the repositories path from the current directory.");
                Err(())
            }
        },
        Err(error) => {
            error!("{:?}", error);
            Err(())
        }
    }
}

pub fn get_repository() -> Result<Repository, ()> {
    match Repository::open_from_env() {
        Ok(repository) => Ok(repository),
        Err(error) => {
            error!("{:?}", error);
            Err(())
        }
    }
}
