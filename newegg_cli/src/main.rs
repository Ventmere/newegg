extern crate chrono;
extern crate dotenv;
extern crate newegg;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate clap;

mod helpers;

macro_rules! dispatch {
  ($matches:expr => $head:tt $($rest:tt)*) => {
    dispatch!(ITEM $matches, $head);
    dispatch!($matches => $($rest)*);
  };

  ($matches:expr => ) => {};

  (ITEM $matches:expr, ($handler:expr)) => {
    ($handler as fn(&clap::ArgMatches))(&$matches)
  };

  (ITEM $matches:expr, ($cmd:ident => $($sub:tt)+)) => {
    if let Some(matches) = $matches.subcommand_matches(stringify!($cmd)) {
      dispatch!(matches => $($sub)*); 
    }
  };
}

fn main() {
  ::dotenv::dotenv().unwrap();

  let matches = clap_app!(myapp =>
    (@subcommand order =>
      (about: "Manage orders")
      (@subcommand test_orders =>
        (@arg FILE: +required "JSON file contains an order array.")
      )
    )
  ).get_matches();

  dispatch! {
    matches =>
      (order =>
        // (list_orders =>
        //   (|_| {
        //     use bestbuy::order::*;
        //     use chrono::{Utc, Duration};
        //     let client = helpers::get_client();
        //     let mut params = ListOrdersParams::default();
        //     params.start_date = Some(Utc::now() - Duration::days(1));
        //     helpers::dump_json(client.list_orders(
        //       &params,
        //       None,
        //       None,
        //     ).unwrap())
        //   })
        // )

        (test_orders =>
          (|m| {
            use std::fs::{self, File};
            use serde_json::Value;
            let path = m.value_of("FILE").unwrap();

            println!("Loading json file: {}", path);

            let file = File::open(path).unwrap();
            let items: Vec<Value> = serde_json::from_reader(file).unwrap();

            println!("Items: {}", items.len());

            for (i, item) in items.into_iter().enumerate() {
              let text = serde_json::to_string_pretty(&item).unwrap();
              fs::write("last_order.json", &text).unwrap();

              println!("Testing {}...", i);
              serde_json::from_str::<::newegg::order::OrderInfo>(&text).unwrap();
            }

            println!("OK.");

            fs::remove_file("last_order.json").unwrap();
          })
        )
      )
  }
}
