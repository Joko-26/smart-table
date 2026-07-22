use std::collections::HashMap;

use crate::models::{
    database_models::{
        ConfigRow, LessonWithSubjectRow, SubjectRow, TaskRow, TestRow, TimeTableRow, UserRow,
    },
    domain_models,
    types::{Device, Email, Password},
};
use chrono::{DateTime, TimeZone, Utc};
use sqlx::types::Json;
use std::fmt::{self, write};
use timespan::{DateSpan, NaiveTimeSpan};
use uuid::{Uuid, uuid};

#[derive(Debug, Clone)]
pub struct DomainError;

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Something with the domain conversion went wrong (row <- domain)"
        )
    }
}

pub struct User {
    pub id: Uuid,
    pub name: String,
    pub plan: String,
    pub devices: HashMap<String, Device>,
    pub email: Email,
    pub password: Password,
    pub change: DateTime<Utc>,
}

impl TryFrom<User> for UserRow {
    type Error = DomainError;

    fn try_from(domain: User) -> Result<Self, Self::Error> {
        Ok(UserRow {
            id: (domain.id),
            name: (domain.name),
            plan: (domain.plan),
            devices: (Json(domain.devices)),
            email: (domain.email.as_str().to_owned()),
            password: (domain.password.password_hash),
            change: (domain.change.format("%d-%b-%Y %H:%M:%S %P %z").to_string()),
        })
    }
}

pub struct TimeTable {
    pub id: Uuid,
    pub user_id: Uuid,
    pub weeks: Vec<Vec<Lesson>>,
    pub change: DateTime<Utc>,
}

impl TryFrom<TimeTable> for TimeTableRow {
    type Error = DomainError;

    fn try_from(domain: TimeTable) -> Result<Self, Self::Error> {
        Ok(TimeTableRow {
            id: (domain.id),
            user_id: (domain.user_id),
            change: (domain.change.format("%d-%b-%Y %H:%M:%S %P %z").to_string()),
            weeks: (Json((domain.weeks.into_iter().map(|week| {
                week.into_iter().map(LessonWithSubjectRow::try_from).collect::<Result<Vec<_>, _>>()
            }).collect::<Result<Vec<_>, _>>()?))),
        })
    }
}

pub struct Lesson {
    pub id: Uuid,
    pub subject: Subject,
    pub duration: NaiveTimeSpan,
    pub teacher: String,
    pub room: String,
    pub change: DateTime<Utc>,
}

impl TryFrom<Lesson> for LessonWithSubjectRow {
    type Error = DomainError;

    fn try_from(domain: Lesson) -> Result<Self, Self::Error> {
        Ok(LessonWithSubjectRow {
            id: (domain.id),
            duration: (domain
                .duration
                .format("{start} - {end}", "%I.%M %P", "%I.%M %P")
                .to_string()),
            teacher: (domain.teacher),
            room: (domain.room),
            change: (domain.change.format("%d-%b-%Y %H:%M:%S %P %z").to_string()),
            subject: (domain.subject.try_into()?),
        })
    }
}

pub struct Subject {
    pub id: Uuid,
    pub color: String,
    pub name: String,
    pub short: String,
    pub main: bool,
    pub change: DateTime<Utc>,
}

impl TryFrom<Subject> for SubjectRow {
    type Error = DomainError;

    fn try_from(domain: Subject) -> Result<Self, Self::Error> {
        Ok(SubjectRow {
            id: (domain.id),
            color: (domain.color),
            name: (domain.name),
            short: (domain.short),
            main: (domain.main),
            change: (domain.change.format("%d-%b-%Y %H:%M:%S %P %z").to_string()),
        })
    }
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

impl TryFrom<Task> for TaskRow {
    type Error = DomainError;

    fn try_from(domain: Task) -> Result<Self, Self::Error> {
        Ok(TaskRow {
            id: (domain.id),
            user_id: (domain.user_id),
            name: (domain.name),
            description: (domain.description),
            due_date: (domain
                .due_date
                .format("%d-%b-%Y %H:%M:%S %P %z")
                .to_string()),
            create_date: (domain
                .create_date
                .format("%d-%b-%Y %H:%M:%S %P %z")
                .to_string()),
            subject: (domain.subject.try_into()?),
            priority: (domain.priority),
            notifications: (domain.notifications),
            change: (domain.change.format("%d-%b-%Y %H:%M:%S %P %z").to_string()),
        })
    }
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

impl TryFrom<Test> for TestRow {
    type Error = DomainError;

    fn try_from(domain: Test) -> Result<Self, Self::Error> {
        Ok(TestRow {
            id: (domain.id),
            user_id: (domain.user_id),
            name: (domain.name),
            description: (domain.description),
            due_date: (domain
                .due_date
                .format("%d-%b-%Y %H:%M:%S %P %z")
                .to_string()),
            create_date: (domain
                .create_date
                .format("%d-%b-%Y %H:%M:%S %P %z")
                .to_string()),
            subject: (domain.subject.try_into()?),
            notifications: (domain.notifications),
            change: domain.change.format("%d-%b-%Y %H:%M:%S %P %z").to_string(),
        })
    }
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

impl TryFrom<Config> for ConfigRow {
    type Error = DomainError;

    fn try_from(domain: Config) -> Result<Self, Self::Error> {
        Ok(ConfigRow {
            id: (domain.id),
            user_id: (domain.user_id),
            mode: (domain.mode),
            theme: (domain.theme),
            icon: (domain.icon),
            notifications: (domain.notifications),
            change: (domain.change.format("%d-%b-%Y %H:%M:%S %P %z").to_string()),
        })
    }
}
