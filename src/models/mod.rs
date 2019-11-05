mod demographics;
mod promotion;
mod expression_parser;
#[macro_use]
mod create_hashmap;
mod promotion_repo;

pub use demographics::*;
pub use promotion::*;
pub use promotion_repo::*;

use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

