use serde::{Serialize, Deserialize};
use std::marker::Copy;

#[derive(Serialize, Deserialize, Debug)]
pub struct Human {
    pub name : String,
    pub age : i8
}