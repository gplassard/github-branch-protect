mod git;
mod github;

fn main() {
    let current_repo = github::current_repo();
    let current_branch = git::current_branch();
    let protected = github::is_branch_protected(current_repo, current_branch);
    println!("hello {}", protected);
}
