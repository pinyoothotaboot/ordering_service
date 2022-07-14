use redlock::RedLock;
use dotenv::dotenv;
use std::env;

pub async fn Locking() -> RedLock {
    dotenv().ok();
    let redis_host : String = env::var("REDIS_HOST").expect("Not found redis host in .env file");
    let redis_conn_url = format!("redis://{}", redis_host);

    let rl = RedLock::new(vec![redis_conn_url]);

    return rl;
}