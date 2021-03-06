#[macro_use]
extern crate rouille;
// extern crate rocket;
extern crate sqlx;

mod database;
pub use database::*;

use rouille::{
    // Request,
    Response,
};

fn main() {
    rouille::start_server("localhost:8080", move |req| {
        // A simple test
        router!(req,
            _ => Response::empty_404()
        )
    });
}
