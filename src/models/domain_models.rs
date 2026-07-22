use std::collections::HashMap;

use crate::models::types::{Device, Email, Password};
use chrono::{DateTime, TimeZone, Utc};
use timespan::{DateSpan, NaiveTimeSpan};
use uuid::{Uuid, uuid};

pub struct User {
    pub id: Uuid,
    pub name: String,
    pub plan: String,
    pub devices: HashMap<String, Device>,
    pub email: Email,
    pub password: Password,
    pub change: DateTime<Utc>,
}

pub struct TimeTable {
    pub id: Uuid,
    pub user_id: Uuid,
    pub weeks: Vec<Vec<Lesson>>,
    pub change: DateTime<Utc>,
}

pub struct Lesson {
    pub id: Uuid,
    pub subject: Subject,
    pub duration: NaiveTimeSpan,
    pub teacher: String,
    pub room: String,
    pub change: DateTime<Utc>,
}

pub struct Subject {
    pub id: Uuid,
    pub color: String,
    pub name: String,
    pub short: String,
    pub main: bool,
    pub change: DateTime<Utc>,
}

pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub due_date: DateTime<Utc>,
    pub create_date: DateTime<Utc>,
    pub subject: Subject,
    pub priority: i64,
    pub notifications: String,
    pub change: DateTime<Utc>,
}

pub struct Test {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub due_date: DateTime<Utc>,
    pub create_date: DateTime<Utc>,
    pub subject: Subject,
    pub notifications: String,
    pub change: DateTime<Utc>,
}

pub struct Config {
    pub id: Uuid,
    pub user_id: Uuid,
    pub mode: String,
    pub theme: String,
    pub icon: String,
    pub notifications: String,
    pub change: DateTime<Utc>,
}
