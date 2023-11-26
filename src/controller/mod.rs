use super::AppState;
use std::sync::Mutex;

pub mod role_controller;
pub mod index_controller;
pub mod user_controller;
pub mod realm_controller;

pub use realm_controller::init as init_realm_controller;
pub use role_controller::init as init_role_controller;
pub use index_controller::init as init_index_controller;
pub use user_controller::init as init_user_controller;

fn log_request(route: &'static str, connections: &Mutex<u32>) {
    let mut con = connections.lock().unwrap();
    *con += 1;
    println!("{}\n\tconnections: {}", route, con);
}
