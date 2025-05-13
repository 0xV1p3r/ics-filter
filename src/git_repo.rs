use crate::cache::copy_from_cache;
use crate::config::{GitConfig, GitRemoteConfig};
use anyhow::{Context, Result, bail};
use git2::{Cred, IndexAddOption, PushOptions, RemoteCallbacks, Repository, Signature};
use std::path::Path;
use url::Url;

pub static REPO_PATH: &str = "calendar_repo";

fn add_all(repo: &Repository) -> Result<()> {
    let mut index = repo
        .index()
        .with_context(|| "Failed to acquire repo index")?;
    index
        .add_all(&["."], IndexAddOption::DEFAULT, None)
        .with_context(|| "Failed to add all files to index")?;
    index.write().with_context(|| "Failed to write index")?;

    Ok(())
}

fn check_if_no_commits_exist(repo: &Repository) -> bool {
    match repo.head() {
        Ok(_) => false,
        Err(_) => true,
    }
}

fn clone_repo(config: &GitRemoteConfig) -> Result<()> {
    let mut url = Url::parse("https://github.com")?;
    url.set_host(Some(&config.domain))
        .with_context(|| "Failed to insert configured domain into URL")?;

    let result = url.set_username(&config.username);
    if result.is_err() {
        bail!("Failed to add username to URL!")
    }

    let result = url.set_password(Some(&config.token));
    if result.is_err() {
        bail!("Failed to add token to URL!")
    }

    let url_path = format!("{}/{}.git", config.username, config.repository);
    url.set_path(&url_path);

    Repository::clone(&url.to_string(), REPO_PATH).with_context(|| "Failed to clone repository")?;
    Ok(())
}

fn commit(message: &str, repo: &Repository, signature: Signature) -> Result<()> {
    let mut index = repo
        .index()
        .with_context(|| "Failed to acquire repo index!")?;
    let oid = index
        .write_tree()
        .with_context(|| "Failed to write index as tree")?;
    let parent_commit = repo
        .head()
        .with_context(|| "Failed to resolve repo HEAD")?
        .peel_to_commit()
        .with_context(|| "Failed to get commit")?;
    let tree = repo
        .find_tree(oid)
        .with_context(|| "Failed to lookup reference")?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &message,
        &tree,
        &[&parent_commit],
    )
    .with_context(|| "Failed to commit")?;

    Ok(())
}

fn create_commit_message(calendar_names: Vec<String>) -> String {
    let mut modded_files = String::new();
    for name in calendar_names {
        modded_files.push_str(&format!("{name},"));
    }
    modded_files.pop();
    format!("AUTOMATED COMMIT -- Updated {modded_files}")
}
fn create_initial_commit(repo: &Repository, signature: Signature) -> Result<()> {
    let oid = repo
        .index()
        .with_context(|| "Failed to acquire repo index")?
        .write_tree()
        .with_context(|| "Failed to write index as tree")?;
    let tree = repo
        .find_tree(oid)
        .with_context(|| "Failed to write index as tree")?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )
    .with_context(|| "Failed to commit")?;
    Ok(())
}

fn push_to_remote(repo: &Repository, username: &String, password: &String) -> Result<()> {
    let head_branch = repo.head().with_context(|| "Failed to resolve repo HEAD")?;
    let head_branch_name = head_branch.name().context("Failed to get branch name!")?;

    let remote_name = repo
        .branch_upstream_remote(head_branch_name)
        .with_context(|| "Failed to retrieve name of upstream remote")?;
    let mut remote = repo
        .find_remote(remote_name.as_str().unwrap())
        .with_context(|| "Failed to fetch remote info")?;

    let mut remote_callbacks = RemoteCallbacks::new();
    remote_callbacks.credentials(|_, _, _| Cred::userpass_plaintext(&username, &password));

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(remote_callbacks);

    remote
        .push::<&str>(&[head_branch_name], Some(&mut push_options))
        .with_context(|| "Failed to push")?;

    Ok(())
}

pub fn initialize_repo(config: &GitConfig) -> Result<()> {
    let repo_path = Path::new(REPO_PATH);
    if is_git_repo(&repo_path) {
        return Ok(());
    }

    if config.remote.is_some() {
        clone_repo(&config.remote.clone().unwrap())
            .with_context(|| "Failed to clone repository")?;
    } else {
        Repository::init(repo_path).with_context(|| "Failed to initialize repository")?;
    }

    Ok(())
}

fn is_git_repo(path: &Path) -> bool {
    match Repository::open(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn update_repo(calendar_names: Vec<String>, config: GitConfig) -> Result<()> {
    for name in &calendar_names {
        let file1 = format!("{name}.ics");
        let file2 = format!("{name}_filtered.ics");
        let dest1 = format!("{REPO_PATH}/{file1}");
        let dest2 = format!("{REPO_PATH}/{file2}");

        copy_from_cache(&file1, &dest1)
            .with_context(|| format!("Failed to copy file '{file1}' from cache"))?;
        copy_from_cache(&file2, &dest2)
            .with_context(|| format!("Failed to copy file '{file2}' from cache"))?;
    }

    let repository = Repository::open(REPO_PATH).with_context(|| "Failed to read repo")?;
    add_all(&repository).with_context(|| "Failed to update index")?;

    let signature = Signature::now(&config.signature.username, &config.signature.email)
        .with_context(|| "Failed to create signature")?;

    if check_if_no_commits_exist(&repository) {
        create_initial_commit(&repository, signature).with_context(|| "Failed to commit")?;
    } else {
        let msg = create_commit_message(calendar_names);
        commit(&msg, &repository, signature).with_context(|| "Failed to commit")?;
    }

    if config.remote.is_some() {
        let remote_cfg = &config.remote.unwrap();
        push_to_remote(&repository, &remote_cfg.username, &remote_cfg.token)
            .with_context(|| "Failed to push to remote")?;
    }

    Ok(())
}
