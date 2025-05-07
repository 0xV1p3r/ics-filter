pub fn create_initial_commit(repo: &git2::Repository, signature: git2::Signature) {
    // TODO: Better error handling (Result & anyhow)
    let oid = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )
    .unwrap();
}

pub fn add_all(repo: &git2::Repository) {
    // TODO: Better error handling (Result & anyhow)
    let mut index = repo.index().unwrap();
    index
        .add_all(&["."], git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    index.write().unwrap();
}

pub fn check_if_no_commits_exist(repo: &git2::Repository) -> bool {
    match repo.head() {
        Ok(_) => false,
        Err(_) => true,
    }
}

pub fn commit(message: &str, repo: &git2::Repository, signature: git2::Signature) {
    // TODO: Better error handling (Result & anyhow)
    let mut index = repo.index().unwrap();
    let oid = index.write_tree().unwrap();
    let parent_commit = repo.head().unwrap().peel_to_commit().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &message,
        &tree,
        &[&parent_commit],
    )
    .unwrap();
}

pub fn push_to_remote(repo: &git2::Repository, username: &String, password: &String) {
    // TODO: Better error handling (Result & anyhow)

    let head_branch = repo.head().unwrap();
    let head_branch_name = head_branch.name().unwrap();

    let remote_name = repo.branch_upstream_remote(head_branch_name).unwrap();
    let mut remote = repo.find_remote(remote_name.as_str().unwrap()).unwrap();

    let mut remote_callbacks = git2::RemoteCallbacks::new();
    remote_callbacks.credentials(|_, _, _| git2::Cred::userpass_plaintext(&username, &password));

    let mut push_options = git2::PushOptions::new();
    push_options.remote_callbacks(remote_callbacks);

    remote
        .push::<&str>(&[head_branch_name], Some(&mut push_options))
        .expect("Unable to push!");
}
