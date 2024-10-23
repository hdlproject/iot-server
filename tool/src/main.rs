use std::env;
use postgres::{Client, NoTls};

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("../iot-manager/migration");
}

fn main() {
    let mut conn = Client::connect(env::var("POSTGRES_URL").unwrap().as_str(), NoTls).unwrap();
    embedded::migrations::runner().run(&mut conn).unwrap();
}
