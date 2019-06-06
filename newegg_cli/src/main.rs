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
  let matches = clap_app!(myapp =>
    (@arg ENV: -e --env +takes_value ".env file path.")
    (@subcommand get_service_status =>
      (@arg DOMAIN: +required "Service domain.")
    )
    (@subcommand order =>
      (about: "Manage orders")
      (@subcommand list_orders =>
      )
      (@subcommand ship =>
        (@arg ORDER_ID: -o --order +required +takes_value "Order ID.")
        (@arg SKU: -i --sku +required +takes_value "Item SKU.")
        (@arg CARRIER: -c --carrier +required +takes_value "Carrier ID.")
        (@arg TRACKING: -t --tracking +required +takes_value "Tracking Number.")
      )
      (@subcommand download_orders =>
        (about: "Download all orders")
        (@arg OUTPUT: +required "Output file path.")
      )
      (@subcommand test_orders =>
        (@arg FILE: +required "JSON file contains an order array.")
      )
    )
  )
  .get_matches();

  let env = matches.value_of("ENV").unwrap_or(".env");

  ::dotenv::from_filename(&env).unwrap();

  dispatch! {
    matches =>
      (get_service_status =>
        (|m| {
          use newegg::service_status::*;
          let client = helpers::get_client();
          let domain_str = m.value_of("DOMAIN").unwrap();
          let domain = ServiceStatusDomain::from_str(&domain_str).ok_or_else(|| {
            format!("Unknown domain: '{}'", domain_str)
          }).unwrap();
          let res = client.get_service_status(domain).unwrap();
          helpers::dump_json(res)
        })
      )

      (order =>
        (ship =>
          (|m| {
            use newegg::order::*;
            let client = helpers::get_client();
            let order_id: i64 = m.value_of("ORDER_ID").unwrap().parse().unwrap();
            let sku = m.value_of("SKU").unwrap();
            let carrier = m.value_of("CARRIER").unwrap();
            let tracking = m.value_of("TRACKING").unwrap();

            let action = ShipOrderAction::new(client.seller_id(), order_id)
              .add_package(
                Package::new(ShipCarrier::Other(carrier.to_owned()), carrier, tracking)
                  .add_item(&sku, 1)
                  .finalize().unwrap()
              )
              .finalize();
            println!("Request:");
            helpers::dump_json(&action);
            print!("\n");
            println!("Response:");
            let res = client.ship_order(order_id, &action).unwrap();
            helpers::dump_json(res);
          })
        )

        (download_orders =>
          (|m| {
            use newegg::order::*;
            use chrono::{DateTime, Utc};
            let client = helpers::get_client();
            let output_path = m.value_of("OUTPUT").unwrap();
            use std::{
              time,
              fs,
              thread::sleep
            };

            let mut page = 1;
            let mut infos = vec![];

            loop {
              println!("Downloading page {}...", page);

              let req = GetOrderInfoRequest::new()
                .page_index(page)
                .page_size(30)
                .finalize();

              let res = client.get_order_info(&req).unwrap();

              println!("total = {}, page_total = {}", res.total(), res.len());

              let mut min_date: Option<DateTime<Utc>> = None;
              let mut max_date: Option<DateTime<Utc>> = None;

              for info in res.info_list().iter().flat_map(|v| v.iter()) {
                let date = info.order_date.as_utc();
                let min = min_date.get_or_insert(date);
                let max = max_date.get_or_insert(date);

                if date < *min {
                  *min = date;
                }

                if date > *max {
                  *max = date;
                }
              }

              println!("min date = {:?}, max date = {:?}",
                min_date, max_date
              );

              if res.is_empty() {
                break
              }

              if let Some(items) = res.info_list() {
                infos.append(&mut (items.clone() as Vec<_>));
              }

              println!("downloaded_total = {}", infos.len());

              page = page + 1;

              sleep(time::Duration::from_secs(1));
            }

            println!("Saving to {}...", output_path);
            fs::write(&output_path, serde_json::to_string_pretty(&infos).unwrap()).unwrap();
          })
        )

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
