use std::collections::HashMap;

use chrono::{DateTime, TimeZone, Utc};
use timespan::{DateSpan, NaiveDateTimeSpan};
use uuid::{uuid, Uuid};
use crate::models::types::{Device, Email, Password};


pub struct User {
    id: Uuid,
    name: String,
    plan: String,
    devices: HashMap<String, Device>,
    email: Email,
    password: Password,
    change: DateTime<Utc>
}

pub struct TimeTable {
    id: Uuid,
    owner_id: Uuid,
    weeks: Vec<Vec<Lesson>>,
    change: DateTime<Utc>,
}

pub struct Lesson {
    id: Uuid,
    owner_id: Uuid,
    subject: Subject,
    duration: NaiveDateTimeSpan,
    teacher: String,
    room: String,
    change: DateTime<Utc>
}

pub struct Subject {
    id: Uuid,
    color: String,
    name: String,
    short: String,
    main: bool,
    change: DateTime<Utc>,
}

pub struct Task {
    id: Uuid,
    owner_id: Uuid,
    name: String,
    description: String,
    due_date: DateTime<Utc>,
    create_date: DateTime<Utc>,
    subject: String,
    priority: i64,
    notifications: String,
    change: DateTime<Utc>,
}

pub struct Test {
    id: Uuid,
    owner_id: String,
    name: String,
    description: String,
    due_date: DateTime<Utc>,
    create_date: DateTime<Utc>,
    subject: Subject,
    notifications: String,
    change: DateTime<Utc>,
}

pub struct Config {
    user_id: Uuid,
    mode: String,
    theme: String,
    icon: String,
    notifications: String,
    change: DateTime<Utc>,
}