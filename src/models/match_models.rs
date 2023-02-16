use crate::schema::matches;
use chrono::{DateTime, TimeZone, Utc};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};

pub mod match_models{}

#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct Match {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub player_one_name: String,
    pub player_two_name: String
}

#[derive(Insertable)]
#[diesel(table_name = matches)]
pub struct NewMatch {
    pub title: String,
    pub body: String,
    pub player_one_name: String,
    pub player_two_name: String,
}

pub struct NewMatchDto {
    pub title: String,
    pub body: String,
    pub player_one_name: String,
    pub player_two_name: String,
}