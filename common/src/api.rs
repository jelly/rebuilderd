use crate::errors::*;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use crate::{Distro, PkgRelease, Status};
use crate::auth;
use reqwest::blocking::{Client as HttpClient, RequestBuilder};

pub const AUTH_COOKIE_HEADER: &str = "X-Auth-Cookie";
pub const WORKER_KEY_HEADER: &str = "X-Worker-Key";
pub const SIGNUP_SECRET_HEADER: &str = "X-Signup-Secret";

pub struct Client {
    endpoint: String,
    client: HttpClient,
    auth_cookie: Option<String>,
    worker_key: Option<String>,
    signup_secret: Option<String>,
}

impl Client {
    pub fn new(endpoint: String) -> Client {
        let client = HttpClient::new();
        Client {
            endpoint,
            client,
            auth_cookie: None,
            worker_key: None,
            signup_secret: None,
        }
    }

    pub fn with_auth_cookie(&mut self) -> Result<&mut Self> {
        let auth_cookie = auth::find_auth_cookie()
            .context("Failed to load auth cookie")?;
        Ok(self.auth_cookie(auth_cookie))
    }

    pub fn auth_cookie<I: Into<String>>(&mut self, cookie: I) -> &mut Self {
        self.auth_cookie = Some(cookie.into());
        self
    }

    pub fn worker_key<I: Into<String>>(&mut self, key: I) {
        self.worker_key = Some(key.into());
    }

    pub fn signup_secret<I: Into<String>>(&mut self, secret: I) {
        self.signup_secret = Some(secret.into());
    }

    pub fn get(&self, path: &'static str) -> RequestBuilder {
        let mut req = self.client.get(&format!("{}{}", self.endpoint, path));
        if let Some(auth_cookie) = &self.auth_cookie {
            req = req.header(AUTH_COOKIE_HEADER, auth_cookie);
        }
        if let Some(worker_key) = &self.worker_key {
            req = req.header(WORKER_KEY_HEADER, worker_key);
        }
        if let Some(signup_secret) = &self.signup_secret {
            req = req.header(SIGNUP_SECRET_HEADER, signup_secret);
        }
        req
    }

    pub fn post(&self, path: &'static str) -> RequestBuilder {
        let mut req = self.client.post(&format!("{}{}", self.endpoint, path));
        if let Some(auth_cookie) = &self.auth_cookie {
            req = req.header(AUTH_COOKIE_HEADER, auth_cookie);
        }
        if let Some(worker_key) = &self.worker_key {
            req = req.header(WORKER_KEY_HEADER, worker_key);
        }
        if let Some(signup_secret) = &self.signup_secret {
            req = req.header(SIGNUP_SECRET_HEADER, signup_secret);
        }
        req
    }

    pub fn list_workers(&self) -> Result<Vec<Worker>> {
        let workers = self.get("/api/v0/workers")
            .send()?
            .error_for_status()?
            .json()?;

        Ok(workers)
    }

    pub fn sync_suite(&self, import: &SuiteImport) -> Result<()> {
        self.post("/api/v0/pkgs/sync")
            .json(import)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    pub fn list_pkgs(&self, list: &ListPkgs) -> Result<Vec<PkgRelease>> {
        let pkgs = self.get("/api/v0/pkgs/list")
            .query(list)
            .send()?
            .error_for_status()?
            .json()?;
        Ok(pkgs)
    }

    pub fn list_queue(&self, list: &ListQueue) -> Result<QueueList> {
        let pkgs = self.post("/api/v0/queue/list")
            .json(list)
            .send()?
            .error_for_status()?
            .json()?;
        Ok(pkgs)
    }

    pub fn push_queue(&self, push: &PushQueue) -> Result<()> {
        self.post("/api/v0/queue/push")
            .json(push)
            .send()?
            .error_for_status()?
            .json()?;
        Ok(())
    }

    pub fn pop_queue(&self, query: &WorkQuery) -> Result<JobAssignment> {
        let assignment = self.post("/api/v0/queue/pop")
            .json(query)
            .send()?
            .error_for_status()?
            .json()?;
        Ok(assignment)
    }

    pub fn drop_queue(&self, query: &DropQueueItem) -> Result<()> {
        self.post("/api/v0/queue/drop")
            .json(query)
            .send()?
            .error_for_status()?
            .json()?;
        Ok(())
    }

    pub fn ping_build(&self, ticket: &QueueItem) -> Result<()> {
        self.post("/api/v0/build/ping")
            .json(ticket)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    pub fn report_build(&self, ticket: &BuildReport) -> Result<()> {
        self.post("/api/v0/build/report")
            .json(ticket)
            .send()?
            .error_for_status()?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Success {
    Ok,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Worker {
    pub key: String,
    pub addr: String,
    pub status: Option<String>,
    pub last_ping: NaiveDateTime,
    pub online: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkQuery {
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JobAssignment {
    Nothing,
    Rebuild(QueueItem),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuiteImport {
    pub distro: Distro,
    pub suite: String,
    pub architecture: String,
    pub pkgs: Vec<PkgRelease>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPkgs {
    pub name: Option<String>,
    pub status: Option<Status>,
    pub distro: Option<String>,
    pub suite: Option<String>,
    pub architecture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueList {
    pub now: NaiveDateTime,
    pub queue: Vec<QueueItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueItem {
    pub id: i32,
    pub package: PkgRelease,
    pub queued_at: NaiveDateTime,
    pub worker_id: Option<i32>,
    pub started_at: Option<NaiveDateTime>,
    pub last_ping: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListQueue {
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushQueue {
    pub name: String,
    pub version: Option<String>,
    pub distro: String,
    pub suite: String,
    pub architecture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DropQueueItem {
    pub name: String,
    pub version: Option<String>,
    pub distro: String,
    pub suite: String,
    pub architecture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BuildStatus {
    Good,
    Bad,
    Fail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildReport {
    pub queue: QueueItem,
    pub status: BuildStatus,
}
