use super::model::{Group, User};

pub mod db_context;
mod group_dao;
mod user_dao;
mod user_to_group_dao;

pub type Database<'a> = db_context::Database<'a>;
pub type Table<'a, T> = db_context::Table<'a, T>;
pub type JoinTable<'a, T1, T2> = db_context::JoinTable<'a, T1, T2>;
