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
#[derive(Debug, PartialEq, Deserialize)]
pub struct APICardBoard {
    //  "https://api.github.com/projects/14550197"
    project_url: String,
    // 83518444
    id: u32,
    // "This is a test"
    note: String,
}
#[derive(Debug, PartialEq)]
pub struct Card {
    project_id: u32,
    id: u32,
    note: String,
}

impl From<APICardBoard> for Card {
    fn from(api_card: APICardBoard) -> Self {
        let regex = Regex::new(r"https://api.github.com/projects/(?P<id>[0-9]+)").unwrap();
        let group = regex
            .captures(&api_card.project_url)
            .expect("Invalid Github Project API Url");
        let project_id: u32 = group
            .name("id")
            .unwrap()
            .as_str()
            .parse()
            .expect("Expected project id to be a number");
        Self {
            project_id,
            id: api_card.id,
            note: api_card.note,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Column {
    pub id: u32,
    pub name: String,
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

    pub fn get_columns(&self, board_id: BoardId) -> Result<Vec<Column>> {
        let url = format!("https://api.github.com/projects/{}/columns", board_id);
        let response: Vec<Column> = reqwest::blocking::Client::new()
            .get(url)
            .header(USER_AGENT, "perty")
            .bearer_auth(self.token.to_owned())
            .header("content-type", "application/json")
            .send()?
            .json()?;

        Ok(response)
    }

    pub fn create_card(&self, column_id: u32, note: String) -> Result<APICardBoard> {
        let url = format!(
            "https://api.github.com/projects/columns/{}/cards",
            column_id
        );
        let response: APICardBoard = reqwest::blocking::Client::new()
            .post(url)
            .header(USER_AGENT, "perty")
            .bearer_auth(self.token.to_owned())
            .header("content-type", "application/json")
            .json(&serde_json::json!({
                "note": &note,
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

    use super::{APICardBoard, Card};

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

    #[test]
    fn cast_api_card_to_card() {
        let api_card = APICardBoard {
            project_url: "https://api.github.com/projects/14550197".to_string(),
            id: 83518444,
            note: "This is a test".to_string(),
        };
        let card: Card = api_card.into();
        assert_eq!(
            card,
            Card {
                project_id: 14550197,
                id: 83518444,
                note: "This is a test".to_string()
            }
        )
    }
}
