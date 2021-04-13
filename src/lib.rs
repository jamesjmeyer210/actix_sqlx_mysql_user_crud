use crate::dao::Database;
use std::sync::{Arc, Mutex};

pub mod config;
pub mod controller;
pub mod dao;
pub mod model;

// AppState
// This the primary dependency for our application's dependency injection.
// Each controller_test function that interacts with the database will require an `AppState` instance in
// order to communicate with the database.
pub struct AppState<'c> {
    pub connections: Mutex<u32>,
    pub context: Arc<Database<'c>>,
}
