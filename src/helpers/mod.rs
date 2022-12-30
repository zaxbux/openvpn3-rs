//! Helper structs to simplify creating and managing VPN sessions.
//!
//! # Examples
//!
//! ```
//! use async_std::task;
//! use openvpn3_rs::helpers::OpenVPN3;
//!
//! fn main() {
//!     task::block_on(async {
//!         let openvpn3 = OpenVPN3::connect().await.unwrap();
//!         
//!         let config_str = "
//!         remote my-server 1194
//!         ";
//!         
//!         let config = openvpn3.import("VPN", config_str, true, true).await.unwrap();
//!         let session = config.new_tunnel().await.unwrap();
//!     
//!         let mut ready = false;
//!         while !ready {
//!             if let Err(err) = session.ready().await {
//!                 if err == openvpn3_rs::Error::MissingUserCredentials {
//!                     for ui in session.fetch_user_input_slots().await.unwrap() {
//!                         let var_name = ui.variable_name();
//!                         if var_name == "username" {
//!                             ui.provide_input("smith").await.unwrap();
//!                         }
//!                         if var_name == "password" {
//!                             ui.provide_input("hunter2").await.unwrap();
//!                         }
//!                     }
//!                 } else if err == openvpn3_rs::Error::BackendNotReady {
//!                     task::sleep(std::time::Duration::from_secs(1));
//!                 }
//!             } else {
//!                 ready = true;
//!             }
//!         }
//!         
//!         session.connect().await.unwrap();
//!
//!     });
//! }
//! ```

mod client;
mod configuration;
mod session;

pub use client::OpenVPN3;
pub use configuration::Configuration;
pub use session::{Session, UserInputSlot};
