pub mod c2c_get_local_user_index_canister_id;
pub mod c2c_is_nobleblocks_user;
pub mod check_email;
pub mod check_username;
pub mod user;
pub mod search_user_by_username;
pub mod search_user;

use ic_cdk_macros::query;
#[query]
fn greet(name: String) -> String {
    format!("Hello, {}\nWelcome to NobleBlocks", name)
}