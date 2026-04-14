
use serde::{Serialize, Deserialize};

pub type Welcome = Vec<WelcomeElement>;

#[derive(Serialize, Deserialize, Debug)]
pub struct WelcomeElement {
    pub id: String,
    #[serde(rename = "type")]
    pub welcome_type: String,
    pub actor: Actor,
    pub repo: Repo,
    pub payload: Payload,
    pub public: bool,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actor {
    pub id: i64,
    pub login: String,
    pub display_login: String,
    pub gravatar_id: String,
    pub url: String,
    pub avatar_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub repository_id: Option<i64>,
    pub push_id: Option<i64>,
    #[serde(rename = "ref")]
    pub payload_ref: Option<String>,
    pub head: Option<String>,
    pub before: Option<String>,
    pub ref_type: Option<String>,
    pub full_ref: Option<String>,
    pub master_branch: Option<String>,
    pub description: Option<String>,
    pub pusher_type: Option<String>,
    pub action: Option<String>,
    pub commits: Option<Vec<Commit>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub sha: String,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    pub id: i64,
    pub name: String,
    pub url: String,
}
