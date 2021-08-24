use std::process::{Command, Stdio};

#[derive(Debug)]
pub(crate) struct Repo {
    pub owner: String,
    pub name: String,
}

pub(crate) fn current_repo() -> Repo {
    let repo_name = Command::new("gh")
        .arg("repo")
        .arg("view")
        .arg("--json")
        .arg("name")
        .arg("--jq")
        .arg(".name")
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let repo_name_str = String::from_utf8(repo_name.stdout).unwrap()
        .replace('\n', "");

    let repo_name_with_owner = Command::new("gh")
        .arg("repo")
        .arg("view")
        .arg("--json")
        .arg("nameWithOwner")
        .arg("--jq")
        .arg(".nameWithOwner")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let owner = String::from_utf8(repo_name_with_owner.stdout).unwrap()
        .replace('\n', "")
        .replace(&repo_name_str, "")
        .replace("/", "");

    Repo {
        owner,
        name: repo_name_str
    }
}


pub(crate) fn is_branch_protected(repo: Repo, branch: String) -> bool {
    let output = Command::new("gh")
        .arg("api")
        .arg(format!("/repos/{}/{}/branches/{}/protection", repo.owner, repo.name, branch))
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let res = String::from_utf8(output.stdout).unwrap();
    let err = String::from_utf8(output.stderr).unwrap();
    println!("{} {}", res, err);
    true
}
