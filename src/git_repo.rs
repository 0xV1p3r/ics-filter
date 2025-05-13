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
        .with_context(|| "Failed to acquire repo index!")?;
    index.add_all(&["."], IndexAddOption::DEFAULT, None)?;
    index.write()?;

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
    url.set_host(Some(&config.domain))?;

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
    let oid = index.write_tree()?;
    let parent_commit = repo.head()?.peel_to_commit()?;
    let tree = repo.find_tree(oid)?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &message,
        &tree,
        &[&parent_commit],
    )?;

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
    let oid = repo.index()?.write_tree()?;
    let tree = repo.find_tree(oid)?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )?;
    Ok(())
}

fn push_to_remote(repo: &Repository, username: &String, password: &String) -> Result<()> {
    let head_branch = repo.head()?;
    let head_branch_name = head_branch.name().context("Failed to get branch name!")?;

    let remote_name = repo.branch_upstream_remote(head_branch_name)?;
    let mut remote = repo.find_remote(remote_name.as_str().unwrap())?;

    let mut remote_callbacks = RemoteCallbacks::new();
    remote_callbacks.credentials(|_, _, _| Cred::userpass_plaintext(&username, &password));

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(remote_callbacks);

    remote.push::<&str>(&[head_branch_name], Some(&mut push_options))?;

    Ok(())
}

pub fn initialize_repo(config: &GitConfig) -> Result<()> {
    let repo_path = Path::new(REPO_PATH);
    if is_git_repo(&repo_path) {
        return Ok(());
    }

    if config.remote.is_some() {
        clone_repo(&config.remote.clone().unwrap())?;
    } else {
        Repository::init(repo_path)?;
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
        copy_from_cache(&file1, REPO_PATH)?;
        copy_from_cache(&file2, REPO_PATH)?;
    }

    let repository = Repository::open(REPO_PATH)?;
    add_all(&repository)?;

    let signature = Signature::now(&config.signature.username, &config.signature.email)?;

    if check_if_no_commits_exist(&repository) {
        create_initial_commit(&repository, signature)?;
    } else {
        let msg = create_commit_message(calendar_names);
        commit(&msg, &repository, signature)?;
    }

    if config.remote.is_some() {
        let remote_cfg = &config.remote.unwrap();
        push_to_remote(&repository, &remote_cfg.username, &remote_cfg.token)?;
    }

    Ok(())
}
