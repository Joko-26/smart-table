use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use std::collections::HashMap;
use std::fmt::{self, write};
use timespan::{NaiveTimeSpan, Span};
use uuid::Uuid;

use crate::models::domain_models::{Config, Lesson, Subject, Task, Test, TimeTable, User};
use crate::models::types::{Device, Email, Password};

#[derive(Debug, Clone)]
pub struct DomainError;

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Something with the domain conversion went wrong")
    }
}

#[derive(sqlx::FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub name: String,
    pub plan: String,
    pub devices: Json<HashMap<String, Device>>,
    pub email: String,
    pub password: String,
    pub change: String,
}

impl TryFrom<UserRow> for User {
    type Error = DomainError;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        Ok(User {
            id: row.id,
            name: row.name,
            plan: row.plan,
            devices: row.devices.0,
            email: Email::new(row.email).unwrap(),
            password: Password {
                password_hash: row.password,
            },
            change: DateTime::parse_from_str(&row.change, "%d-%b-%Y %H:%M:%S %P %z")
                .unwrap()
                .into(),
        })
    }
}

#[derive(sqlx::FromRow)]
pub struct TimeTableRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub change: String,

    #[sqlx(flatten)]
    pub weeks: Json<Vec<Vec<LessonWithSubjectRow>>>,
}

impl TryFrom<TimeTableRow> for TimeTable {
    type Error = DomainError;

    fn try_from(row: TimeTableRow) -> Result<Self, Self::Error> {
        Ok(TimeTable {
            id: (row.id),
            user_id: (row.user_id),
            weeks: (row
                .weeks
                .0
                .into_iter()
                .map(|week| {
                    week.into_iter()
                        .map(Lesson::try_from)
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?),
            change: (DateTime::parse_from_str(&row.change, "%d-%b-%Y %H:%M:%S %P %z")
                .unwrap()
                .into()),
        })
    }
}

#[derive(sqlx::FromRow)]
pub struct LessonWithSubjectRow {
    pub id: Uuid,
    pub duration: String,
    pub teacher: String,
    pub room: String,
    pub change: String,

    #[sqlx(flatten)]
    pub subject: SubjectRow,
}

#[derive(sqlx::FromRow)]
pub struct SubjectRow {
    pub id: Uuid,
    pub color: String,
    pub name: String,
    pub short: String,
    pub main: bool,
    pub change: String,
}

impl TryFrom<SubjectRow> for Subject {
    type Error = DomainError;

    fn try_from(row: SubjectRow) -> Result<Self, Self::Error> {
        Ok(Subject {
            id: row.id,
            color: row.color,
            name: row.name,
            short: row.short,
            main: row.main,
            change: (DateTime::parse_from_str(&row.change, "%d-%b-%Y %H:%M:%S %P %z")
                .unwrap()
                .into()),
        })
    }
}

impl TryFrom<LessonWithSubjectRow> for Lesson {
    type Error = DomainError;

    fn try_from(row: LessonWithSubjectRow) -> Result<Self, Self::Error> {
        Ok(Lesson {
            id: (row.id),
            subject: (row.subject.try_into()?),
            duration: (NaiveTimeSpan::parse_from_str(
                &row.duration,
                "{start} - {end}",
                "%I.%M %P",
                "%I.%M %P",
            )
            .unwrap()),
            teacher: (row.teacher),
            room: (row.room),
            change: (DateTime::parse_from_str(&row.change, "%d-%b-%Y %H:%M:%S %P %z")
                .unwrap()
                .into()),
        })
    }
}

#[derive(sqlx::FromRow)]
pub struct TaskRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub due_date: String,
    pub create_date: String,
    pub subject: SubjectRow,
    pub priority: i64,
    pub notifications: String,
    pub change: String,
}

#[derive(sqlx::FromRow)]
pub struct TaskWithSubjectRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub due_date: String,
    pub create_date: String,
    pub subject: SubjectRow,
    pub priority: i64,
    pub notifications: String,
    pub change: String,
}

impl TryFrom<TaskWithSubjectRow> for Task {
    type Error = DomainError;

    fn try_from(row: TaskWithSubjectRow) -> Result<Self, Self::Error> {
        Ok(Task {
            id: (row.id),
            user_id: (row.user_id),
            name: (row.name),
            description: (row.description),
            due_date: (DateTime::parse_from_str(&row.due_date, "%d-%b-%Y %H:%M:%S %P %z")
                .unwrap()
                .into()),
            create_date: (DateTime::parse_from_str(&row.create_date, "%d-%b-%Y %H:%M:%S %P %z")
                .unwrap()
                .into()),
            subject: (row.subject.try_into()?),
            priority: (row.priority),
            notifications: (row.notifications),
            change: (DateTime::parse_from_str(&row.change, "%d-%b-%Y %H:%M:%S %P %z")
                .unwrap()
                .into()),
        })
    }
}

#[derive(sqlx::FromRow)]
pub struct TestRow {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub due_date: String,
    pub create_date: String,
    pub subject: SubjectRow,
    pub notifications: String,
    pub change: String,
}

#[derive(sqlx::FromRow)]
pub struct TestWithSubjectRow {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub due_date: String,
    pub create_date: String,
    pub subject: SubjectRow,
    pub notifications: String,
    pub change: String,
}

impl TryFrom<TestWithSubjectRow> for Test {
    type Error = DomainError;

    fn try_from(row: TestWithSubjectRow) -> Result<Self, Self::Error> {
        Ok(Test {
            id: (row.id),
            user_id: (row.user_id),
            name: (row.name),
            description: (row.description),
            due_date: (DateTime::parse_from_str(&row.due_date, "%d-%b-%Y %H:%M:%S %P %z")
                .unwrap()
                .into()),
            create_date: (DateTime::parse_from_str(&row.create_date, "%d-%b-%Y %H:%M:%S %P %z")
                .unwrap()
                .into()),
            subject: (row.subject.try_into()?),
            notifications: (row.notifications),
            change: (DateTime::parse_from_str(&row.change, "%d-%b-%Y %H:%M:%S %P %z")
                .unwrap()
                .into()),
        })
    }
}

#[derive(sqlx::FromRow)]
pub struct ConfigRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub mode: String,
    pub theme: String,
    pub icon: String,
    pub notifications: String,
    pub change: String,
}

impl TryFrom<ConfigRow> for Config {
    type Error = DomainError;

    fn try_from(row: ConfigRow) -> Result<Self, Self::Error> {
        Ok(Config {
            id: row.id,
            user_id: row.user_id,
            mode: row.mode,
            theme: row.theme,
            icon: row.icon,
            notifications: row.notifications,
            change: (DateTime::parse_from_str(&row.change, "%d-%b-%Y %H:%M:%S %P %z")
                .unwrap()
                .into()),
        })
    }
}
