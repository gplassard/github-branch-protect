use std::process::{Command, Stdio};
use serde_json::Result;
use crate::github_api::BranchProtection;

#[derive(Debug, Clone)]
pub(crate) struct Repo {
    pub owner: String,
    pub name: String,
}

impl ToString for Repo {
    fn to_string(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
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


pub(crate) fn get_branch_protection(repo: &Repo, branch: &String) -> Option<BranchProtection> {
    let output = Command::new("gh")
        .arg("api")
        .arg(format!("/repos/{}/{}/branches/{}/protection", repo.owner, repo.name, branch))
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let res = String::from_utf8(output.stdout).unwrap();
    if output.status.success() {
        let json_res: BranchProtection = serde_json::from_str(&res).unwrap();
        return Option::Some(json_res)
    } else {
        let err = String::from_utf8(output.stderr).unwrap();
        if err.contains("Branch not protected") {
            return Option::None
        }
        else {
            panic!("Error while checking branch protection \n{}", err);
        }
    }
}
