pub mod c2c_is_nobleblocks_user;
pub mod check_email;
pub mod check_username;
pub mod get_random_users;
pub mod get_user_info;
pub mod get_user_info_by_username;
pub mod get_user_infos;
pub mod get_users;
pub mod http_request;
pub mod login_user;
pub mod search_user_by_username;
pub mod search_user;

use ic_cdk_macros::query;
#[query]
fn greet(name: String) -> String {
    format!("Hello, {}\nWelcome to NobleBlocks\n{}", name, ic_cdk::api::canister_balance128())
}