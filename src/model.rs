use std::io::Write;

use diesel::{deserialize, serialize};
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::TinyInt;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, FromSqlRow, AsExpression, Clone, Copy)]
#[repr(i8)]
#[sql_type = "TinyInt"]
pub enum Class {
    Unknown = -1,
    Fighter = 1,
    Magician = 2,
    Swordsman = 3,
    Ranger = 4,
    Support = 5,
    Beast = 6,
    Machinist = 7,
    Supernatural = 8,
    Scholar = 9,
    Worker = 10,
    Musician = 11,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, AsExpression, FromSqlRow, Clone, Copy)]
#[repr(i8)]
#[sql_type = "TinyInt"]
pub enum Genre {
    Unknown = -1,
    Action = 1,
    Adventure = 2,
    SciFi = 3,
    Sports = 4,
    Mystery = 5,
    SliceOfLife = 6,
    Comedy = 7,
    Romance = 8,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, AsExpression, FromSqlRow, Clone, Copy)]
#[repr(i8)]
#[sql_type = "TinyInt"]
pub enum Rarity {
    Unknown = -1,
    Silver = 1,
    Gold = 2,
    Epic = 3,
    Legend = 4,
}

impl_tinyint_sql_op!(Class);
impl_tinyint_sql_op!(Genre);
impl_tinyint_sql_op!(Rarity);

impl Default for Class {
    fn default() -> Self {
        Class::Unknown
    }
}

impl Default for Genre {
    fn default() -> Self {
        Genre::Unknown
    }
}

impl Default for Rarity {
    fn default() -> Self {
        Rarity::Unknown
    }
}


impl ToString for Rarity {
    fn to_string(&self) -> String {
        match self {
            Rarity::Silver => "Silver".to_string(),
            Rarity::Gold => "Gold".to_string(),
            Rarity::Epic => "Epic".to_string(),
            Rarity::Legend => "Legend".to_string(),
            _ => "".to_string()
        }
    }
}

impl ToString for Class {
    fn to_string(&self) -> String {
        match self {
            Class::Fighter => "Fighter".to_string(),
            Class::Magician => "Magician".to_string(),
            Class::Swordsman => "Swordsman".to_string(),
            Class::Ranger => "Ranger".to_string(),
            Class::Support => "Support".to_string(),
            Class::Beast => "Beast".to_string(),
            Class::Machinist => "Machinist".to_string(),
            Class::Supernatural => "Supernatural".to_string(),
            Class::Scholar => "Scholar".to_string(),
            Class::Worker => "Worker".to_string(),
            Class::Musician => "Musician".to_string(),
            Class::Unknown => "".to_string(),
        }
    }
}

impl ToString for Genre {
    fn to_string(&self) -> String {
        match self {
            Genre::Action => "Action".to_string(),
            Genre::Adventure => "Adventure".to_string(),
            Genre::SciFi => "Sci-Fi".to_string(),
            Genre::Sports => "Sports".to_string(),
            Genre::Mystery => "Mystery".to_string(),
            Genre::SliceOfLife => "Slice of Life".to_string(),
            Genre::Comedy => "Comedy".to_string(),
            Genre::Romance => "Romance".to_string(),
            Genre::Unknown => "".to_string(),
        }
    }
}