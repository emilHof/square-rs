/*!
The endpoints of the [Square API](https://developer.squareup.com).

To ensure the crate remains as extensible as possible, we are using
the Display trait for the URL of all of the endpoints
 */

pub mod payment;
pub mod bookings;
pub mod locations;
pub mod catalog;
pub mod customers;
pub mod cards;
pub mod checkout;
pub mod inventory;
pub mod sites;
pub mod terminal;
pub mod orders;

use crate::client::ClientMode;
use crate::client::SquareClient;
use std::fmt;

/// All of the endpoints of the [Square API](https://developer.squareup.com)
/// for which we have implemented some of the functionality.
#[non_exhaustive]
pub enum SquareAPI {
    Payments(String),
    Bookings(String),
    Locations(String),
    Catalog(String),
    Customers(String),
    Cards(String),
    Checkout(String),
    Inventory(String),
    Sites(String),
    Terminals(String),
    Orders(String),
}

/// All of the HTTP verbs that have been implemented and are accepted by the different
/// [Square API](https://developer.squareup.com) endpoints.
pub enum Verb {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

/// Implement the Display trait for all of the endpoints we need this allows
/// for them to be changed in the future without effecting the existing code
/// base.
impl fmt::Display for SquareAPI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SquareAPI::Payments(path) => write!(f, "payments{}", path),
            SquareAPI::Bookings(path) => write!(f, "bookings{}", path),
            SquareAPI::Locations(path) => write!(f, "locations{}", path),
            SquareAPI::Catalog(path) => write!(f, "catalog{}", path),
            SquareAPI::Customers(path) => write!(f, "customers{}", path),
            SquareAPI::Cards(path) => write!(f, "cards{}", path),
            SquareAPI::Checkout(path) => write!(f, "online-checkout{}", path),
            SquareAPI::Inventory(path) => write!(f, "inventory{}", path),
            SquareAPI::Sites(path) => write!(f, "sites{}", path),
            SquareAPI::Terminals(path) => write!(f, "terminals{}", path),
            SquareAPI::Orders(path) => write!(f, "orders{}", path),
        }
    }
}

impl SquareClient {
    pub fn endpoint(&self, end_point: SquareAPI) -> String {
        /// The main base URL for the Square API
        const SQUARE_PRODUCTION_BASE: &str = "https://connect.squareup.com/v2/";
        const SQUARE_SANDBOX_BASE: &str = "https://connect.squareupsandbox.com/v2/";

        match self.client_mode {
            ClientMode::Production => format!("{}{}", SQUARE_PRODUCTION_BASE, end_point),
            ClientMode::Sandboxed => format!("{}{}", SQUARE_SANDBOX_BASE, end_point),
        }
    }
}
