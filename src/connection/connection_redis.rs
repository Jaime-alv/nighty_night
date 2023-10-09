use redis::Client;

use crate::configuration::settings::Setting;

pub async fn poll() -> Client {
    let address = Setting::RedisHost.get();
    Client::open(address).expect("Can't connect to redis")
}
