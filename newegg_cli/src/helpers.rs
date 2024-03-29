use newegg::client::*;
use serde::Serialize;
use serde_json;
use std::env;
use std::io::stdout;

pub fn get_client() -> NeweggClient {
  NeweggClient::new(
    match env::var("PLATFORM").unwrap().as_ref() {
      "Newegg" => NeweggPlatform::Newegg,
      "NeweggCanada" => NeweggPlatform::NeweggCanada,
      "NeweggBusiness" => NeweggPlatform::NeweggBusiness,
      v => panic!("Unknown platform: '{}'", v),
    },
    &env::var("SELLER_ID").unwrap(),
    &env::var("TOKEN").unwrap(),
    &env::var("SECRET_KEY").unwrap(),
  )
  .unwrap()
}

pub fn dump_json<T: Serialize>(v: T) {
  serde_json::to_writer_pretty(stdout(), &v).unwrap()
}
