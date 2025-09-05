use chrono::{DateTime, Utc};
use std::fmt;
use std::fmt::Display;
use uuid::Uuid;

pub struct User {
    name: String,
    link: Option<String>,
    working_on: Option<Goal>,
}

pub struct Roadmap {
    id: Uuid,
    title: String,
    content: Vec<String>,
    author: User,
    goals: Vec<Goal>,
}

pub struct Goal {
    id: Uuid,
    title: String,
    description: Option<Vec<String>>,
    author: String,
    status: Status,
    priority: Priority,
    tags: Option<Vec<Tag>>,
    progress: Progress,
    deadline: Option<DateTime<Utc>>,
    workers: Option<Vec<User>>,
    inner: Option<Vec<Goal>>,
}

pub struct Project {
    pub goal: Goal,
    pub maintainers: Vec<User>,
}

pub struct Progress {
    fulfilled: u32,
    total: u32,
}

impl Progress {
    pub fn completed(&self) -> bool {
        self.fulfilled >= self.total
    }
}

impl Display for Progress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.completed() {
            write!(f, "[{}/{} done] ✔️", self.total, self.total)
        } else {
            write!(f, "[{}/{} done] ❌", self.fulfilled, self.total)
        }
    }
}

pub enum Status {
    Uncertain,
    Planned,
    UnderConstruction,
    WorkedOn,
    Fulfilled,
}

pub enum Tag {
    NeedsWork,
    Placeholder,
    Suggestion,
    SubjectToChange,
}

pub enum Priority {
    Unplanned,
    Low,
    Medium,
    High,
    Critical,
    GalaxyCollapse,
}
