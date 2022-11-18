extern crate git2;
use git2::Repository;

pub fn clone_repo(repo: &str) {
    let url = String::from("https://github.com/") + repo;
    println!("Cloning {}...", url);
    let git_name = url.split("/").collect::<Vec<&str>>().pop().unwrap();
    let repo_name = git_name.replace(".git", "");
    match Repository::clone(&url, format!("./temp-{}", repo_name)) {
        Ok(_) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}