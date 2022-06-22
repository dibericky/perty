use anyhow::{Context, Result};
use regex::Regex;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

pub type BoardId = i32;

pub struct Github {
    token: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatedBoard {
    pub html_url: String,
    pub id: BoardId,
}

impl Github {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn create_board(&self, name: String, owner: String, repo: String) -> Result<CreatedBoard> {
        let url = format!("https://api.github.com/repos/{}/{}/projects", owner, repo);
        let response: CreatedBoard = reqwest::blocking::Client::new()
            .post(url)
            .header(USER_AGENT, "perty")
            .bearer_auth(self.token.to_owned())
            .header("content-type", "application/json")
            .json(&serde_json::json!({
                "name": &name,
                "body": "Project created by PERTy: https://github.com/dibericky/perty",
            }))
            .send()?
            .json()?;

        Ok(response)
    }
}

pub fn get_owner_repo_from_url(url: &str) -> Result<(String, String)> {
    let regex = Regex::new(r"http(s)://github\.com/(?P<owner>[^/]+)/(?P<repo>[^/]+)").unwrap();
    let group = regex
        .captures(url)
        .context("Not a valid github repository url")?;
    let owner: String = group.name("owner").unwrap().as_str().to_string();
    let repo: String = group.name("repo").unwrap().as_str().to_string();
    Ok((owner, repo))
}

#[cfg(test)]
mod test {
    use crate::modules::github::get_owner_repo_from_url;

    #[test]
    fn get_owner_repo_from_url_ok() {
        let input = "https://github.com/dibericky/perty";
        let output = get_owner_repo_from_url(input).unwrap();
        assert_eq!(output, ("dibericky".to_owned(), "perty".to_owned()));
    }

    #[test]
    fn get_owner_repo_from_url_missing_repo() {
        let input = "https://github.com/dibericky/";
        let error = get_owner_repo_from_url(input).err().unwrap().to_string();
        assert_eq!(error, "Not a valid github repository url".to_string());
    }

    #[test]
    fn get_owner_repo_from_url_not_github() {
        let input = "https://foobar.com/dibericky/perty";
        let error = get_owner_repo_from_url(input).err().unwrap().to_string();
        assert_eq!(error, "Not a valid github repository url".to_string());
    }
}
