use chrono::{DateTime, Local};
use reqwest::{Client, Method, Result};
use serde::{Deserialize, Serialize};

/// Repository : Repository represents a repository
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Repository {
  // #[serde(rename = "allow_fast_forward_only_merge", skip_serializing_if = "Option::is_none")]
  // pub allow_fast_forward_only_merge: Option<bool>,
  // #[serde(rename = "allow_manual_merge", skip_serializing_if = "Option::is_none")]
  // pub allow_manual_merge: Option<bool>,
  // #[serde(rename = "allow_merge_commits", skip_serializing_if = "Option::is_none")]
  // pub allow_merge_commits: Option<bool>,
  // #[serde(rename = "allow_rebase", skip_serializing_if = "Option::is_none")]
  // pub allow_rebase: Option<bool>,
  // #[serde(rename = "allow_rebase_explicit", skip_serializing_if = "Option::is_none")]
  // pub allow_rebase_explicit: Option<bool>,
  // #[serde(rename = "allow_rebase_update", skip_serializing_if = "Option::is_none")]
  // pub allow_rebase_update: Option<bool>,
  // #[serde(rename = "allow_squash_merge", skip_serializing_if = "Option::is_none")]
  // pub allow_squash_merge: Option<bool>,
  // #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
  // pub archived: Option<bool>,
  // #[serde(rename = "archived_at", skip_serializing_if = "Option::is_none")]
  // pub archived_at: Option<String>,
  // #[serde(rename = "autodetect_manual_merge", skip_serializing_if = "Option::is_none")]
  // pub autodetect_manual_merge: Option<bool>,
  // #[serde(rename = "avatar_url", skip_serializing_if = "Option::is_none")]
  // pub avatar_url: Option<String>,
  // #[serde(rename = "clone_url", skip_serializing_if = "Option::is_none")]
  // pub clone_url: Option<String>,
  // #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
  // pub created_at: Option<String>,
  // #[serde(rename = "default_allow_maintainer_edit", skip_serializing_if = "Option::is_none")]
  // pub default_allow_maintainer_edit: Option<bool>,
  // #[serde(rename = "default_branch", skip_serializing_if = "Option::is_none")]
  // pub default_branch: Option<String>,
  // #[serde(rename = "default_delete_branch_after_merge", skip_serializing_if = "Option::is_none")]
  // pub default_delete_branch_after_merge: Option<bool>,
  // #[serde(rename = "default_merge_style", skip_serializing_if = "Option::is_none")]
  // pub default_merge_style: Option<String>,
  // #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
  // pub description: Option<String>,
  // #[serde(rename = "empty", skip_serializing_if = "Option::is_none")]
  // pub empty: Option<bool>,
  // #[serde(rename = "external_tracker", skip_serializing_if = "Option::is_none")]
  // pub external_tracker: Option<Box<models::ExternalTracker>>,
  // #[serde(rename = "external_wiki", skip_serializing_if = "Option::is_none")]
  // pub external_wiki: Option<Box<models::ExternalWiki>>,
  // #[serde(rename = "fork", skip_serializing_if = "Option::is_none")]
  // pub fork: Option<bool>,
  // #[serde(rename = "forks_count", skip_serializing_if = "Option::is_none")]
  // pub forks_count: Option<i64>,
  //#[serde(rename = "full_name", skip_serializing_if = "Option::is_none")]
  pub full_name: String,
  // #[serde(rename = "has_actions", skip_serializing_if = "Option::is_none")]
  // pub has_actions: Option<bool>,
  // #[serde(rename = "has_code", skip_serializing_if = "Option::is_none")]
  // pub has_code: Option<bool>,
  // #[serde(rename = "has_issues", skip_serializing_if = "Option::is_none")]
  // pub has_issues: Option<bool>,
  // #[serde(rename = "has_packages", skip_serializing_if = "Option::is_none")]
  // pub has_packages: Option<bool>,
  // #[serde(rename = "has_projects", skip_serializing_if = "Option::is_none")]
  // pub has_projects: Option<bool>,
  // #[serde(rename = "has_pull_requests", skip_serializing_if = "Option::is_none")]
  // pub has_pull_requests: Option<bool>,
  // #[serde(rename = "has_releases", skip_serializing_if = "Option::is_none")]
  // pub has_releases: Option<bool>,
  // #[serde(rename = "has_wiki", skip_serializing_if = "Option::is_none")]
  // pub has_wiki: Option<bool>,
  #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
  pub html_url: Option<String>,
  #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
  pub id: Option<i64>,
  // #[serde(rename = "ignore_whitespace_conflicts", skip_serializing_if = "Option::is_none")]
  // pub ignore_whitespace_conflicts: Option<bool>,
  // #[serde(rename = "internal", skip_serializing_if = "Option::is_none")]
  // pub internal: Option<bool>,
  // #[serde(rename = "internal_tracker", skip_serializing_if = "Option::is_none")]
  // pub internal_tracker: Option<Box<models::InternalTracker>>,
  // #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
  // pub language: Option<String>,
  // #[serde(rename = "languages_url", skip_serializing_if = "Option::is_none")]
  // pub languages_url: Option<String>,
  // #[serde(rename = "licenses", skip_serializing_if = "Option::is_none")]
  // pub licenses: Option<Vec<String>>,
  // #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
  // pub link: Option<String>,
  // #[serde(rename = "mirror", skip_serializing_if = "Option::is_none")]
  // pub mirror: Option<bool>,
  // #[serde(rename = "mirror_interval", skip_serializing_if = "Option::is_none")]
  // pub mirror_interval: Option<String>,
  // #[serde(rename = "mirror_updated", skip_serializing_if = "Option::is_none")]
  // pub mirror_updated: Option<String>,
  // #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
  // pub name: Option<String>,
  // /// ObjectFormatName of the underlying git repository
  // #[serde(rename = "object_format_name", skip_serializing_if = "Option::is_none")]
  // pub object_format_name: Option<ObjectFormatName>,
  // #[serde(rename = "open_issues_count", skip_serializing_if = "Option::is_none")]
  // pub open_issues_count: Option<i64>,
  // #[serde(rename = "open_pr_counter", skip_serializing_if = "Option::is_none")]
  // pub open_pr_counter: Option<i64>,
  // #[serde(rename = "original_url", skip_serializing_if = "Option::is_none")]
  // pub original_url: Option<String>,
  // #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
  // pub owner: Option<Box<models::User>>,
  // #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
  // pub parent: Option<Box<models::Repository>>,
  // #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
  // pub permissions: Option<Box<models::Permission>>,
  // #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
  // pub private: Option<bool>,
  // #[serde(rename = "projects_mode", skip_serializing_if = "Option::is_none")]
  // pub projects_mode: Option<String>,
  // #[serde(rename = "release_counter", skip_serializing_if = "Option::is_none")]
  // pub release_counter: Option<i64>,
  // #[serde(rename = "repo_transfer", skip_serializing_if = "Option::is_none")]
  // pub repo_transfer: Option<Box<models::RepoTransfer>>,
  // #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
  // pub size: Option<i64>,
  // #[serde(rename = "ssh_url", skip_serializing_if = "Option::is_none")]
  // pub ssh_url: Option<String>,
  // #[serde(rename = "stars_count", skip_serializing_if = "Option::is_none")]
  // pub stars_count: Option<i64>,
  // #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
  // pub template: Option<bool>,
  // #[serde(rename = "topics", skip_serializing_if = "Option::is_none")]
  // pub topics: Option<Vec<String>>,
  // #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
  // pub updated_at: Option<String>,
  // #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
  // pub url: Option<String>,
  // #[serde(rename = "watchers_count", skip_serializing_if = "Option::is_none")]
  // pub watchers_count: Option<i64>,
  // #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
  // pub website: Option<String>,
}

/// ActionWorkflowRun : ActionWorkflowRun represents a WorkflowRun
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionWorkflowRun {
  // #[serde(rename = "actor", skip_serializing_if = "Option::is_none")]
  // pub actor: Option<Box<models::User>>,
  #[serde(rename = "completed_at", skip_serializing_if = "Option::is_none")]
  pub completed_at: Option<DateTime<Local>>,
  #[serde(rename = "conclusion", skip_serializing_if = "Option::is_none")]
  pub conclusion: Option<String>,
  #[serde(rename = "display_title", skip_serializing_if = "Option::is_none")]
  pub display_title: Option<String>,
  // #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
  // pub event: Option<String>,
  // #[serde(rename = "head_branch", skip_serializing_if = "Option::is_none")]
  // pub head_branch: Option<String>,
  // #[serde(rename = "head_repository", skip_serializing_if = "Option::is_none")]
  // pub head_repository: Option<Box<models::Repository>>,
  // #[serde(rename = "head_sha", skip_serializing_if = "Option::is_none")]
  // pub head_sha: Option<String>,
  #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
  pub html_url: Option<String>,
  #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
  pub id: Option<i64>,
  // #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
  // pub path: Option<String>,
  // #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
  // pub repository: Option<Box<models::Repository>>,
  // #[serde(rename = "repository_id", skip_serializing_if = "Option::is_none")]
  // pub repository_id: Option<i64>,
  #[serde(rename = "run_attempt", skip_serializing_if = "Option::is_none")]
  pub run_attempt: Option<i64>,
  //#[serde(rename = "run_number", skip_serializing_if = "Option::is_none")]
  pub run_number: i64,
  #[serde(rename = "started_at", skip_serializing_if = "Option::is_none")]
  pub started_at: Option<DateTime<Local>>,
  //#[serde(rename = "status", skip_serializing_if = "Option::is_none")]
  pub status: String,
  // #[serde(rename = "trigger_actor", skip_serializing_if = "Option::is_none")]
  // pub trigger_actor: Option<Box<models::User>>,
  // #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
  // pub url: Option<String>,
}

/// ActionWorkflowRunsResponse : ActionWorkflowRunsResponse returns ActionWorkflowRuns
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionWorkflowRunsResponse {
  #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
  pub total_count: Option<i64>,
  #[serde(rename = "workflow_runs", skip_serializing_if = "Option::is_none")]
  pub workflow_runs: Option<Vec<ActionWorkflowRun>>,
}

/// Gitea API
pub struct Gitea {
  pub base_url: String,
  pub token: String,
  client: Client,
}

impl Gitea {
  pub fn new(base_url: String, token: String) -> Self {
    // let client  = Client::builder().proxy(Proxy::all("127.0.0.1:9000").unwrap()).build().unwrap();
    Self {
      base_url: format!("{}/api/v1", base_url.trim_end_matches('/')),
      token,
      client: Client::new(),
    }
  }

  pub fn reset(&mut self, base_url: String, token: String) {
    self.base_url = format!("{}/api/v1", base_url.trim_end_matches('/'));
    self.token = token;
  }

  pub async fn user_current_list_repos(
    &self,
    page: Option<i32>,
    limit: Option<i32>,
  ) -> Result<Vec<Repository>> {
    let url = format!("{}/user/repos", self.base_url);
    let mut req = self
      .client
      .request(Method::GET, url)
      .bearer_auth(&self.token);

    if let Some(page) = page {
      req = req.query(&[("page", page.to_string())]);
    }

    if let Some(limit) = limit {
      req = req.query(&[("limit", limit.to_string())]);
    }

    let req = req.build()?;
    self
      .client
      .execute(req)
      .await?
      .json::<Vec<Repository>>()
      .await
  }

  pub async fn get_workflow_runs(
    &self,
    repo: &Repository,
    page: Option<i32>,
    limit: Option<i32>,
  ) -> Result<ActionWorkflowRunsResponse> {
    let url = format!("{}/repos/{}/actions/runs", self.base_url, repo.full_name);
    let mut req = self
      .client
      .request(Method::GET, url)
      .bearer_auth(&self.token);

    if let Some(page) = page {
      req = req.query(&[("page", page.to_string())]);
    }

    if let Some(limit) = limit {
      req = req.query(&[("limit", limit.to_string())]);
    }

    let req = req.build()?;
    self
      .client
      .execute(req)
      .await?
      .json::<ActionWorkflowRunsResponse>()
      .await
  }
}
