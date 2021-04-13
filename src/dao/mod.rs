use super::model::{Group, User};

pub mod db_context;
mod group_dao;
mod user_dao;
mod user_to_group_dao;

pub type Database = db_context::Database;
pub type Table<T> = db_context::Table<T>;
pub type JoinTable<T1, T2> = db_context::JoinTable<T1, T2>;
