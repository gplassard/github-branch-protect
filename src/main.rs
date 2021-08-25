mod git;
mod github;
mod github_api;

fn main() {
    let current_repo = github::current_repo();
    println!("Repo {}", current_repo.to_string());

    let current_branch = git::current_branch();
    let protected = github::get_branch_protection(&current_repo, &current_branch);
    let status = if protected.is_some() {"✔️ (protected)"} else {"❌ (not protected)"};
    println!("Branch {} {}", current_branch, status);
}
