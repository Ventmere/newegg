use clap::clap_app;
use newegg::NeweggPlatform;
use serde_json::json;

mod helpers;
use helpers::block_on_unwrap;
mod report;

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
      (@subcommand get =>
        (@arg ORDER_ID: -o --order +required +takes_value "Order ID.")
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
    (@subcommand report =>
      (@subcommand submit =>
        (@arg REPORT_TYPE: -t --type +required +takes_value "Report Type.")
      )
      (@subcommand get_status =>
        (@arg ID: -i --id +required +multiple +takes_value "Report Request ID.")
      )
      (@subcommand get_result =>
        (@arg OPERATION_TYPE: -t --type +required +takes_value "Operation Type.")
        (@arg ID: -i --id +required +takes_value "Report Request ID.")
      )
      (@subcommand get_report_file =>
        (@arg URL: -u --url +required +takes_value "URL.")
      )
      (@subcommand test_parse_us =>
        (@arg FILE: +required "CSV file to parse.")
      )
      (@subcommand test_parse_ca =>
        (@arg FILE: +required "CSV file to parse.")
      )
    )
    (@subcommand feed =>
      (@subcommand get_status =>
        (@arg ID: -i --id +required +multiple +takes_value "Feed Request ID.")
      )
      (@subcommand get_result =>
        (@arg ID: -i --id +required +takes_value "Feed Request ID.")
      )
      (@subcommand test_inventory_update => )
      (@subcommand test_inventory_and_price_update => )
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
          let res = block_on_unwrap(client.get_service_status(domain));
          helpers::dump_json(res)
        })
      )

      (order =>
        (get =>
          (|m| {
            use newegg::order::*;
            let client = helpers::get_client();
            let order_id: i64 = m.value_of("ORDER_ID").unwrap().parse().unwrap();
            let order = client.get_order_info(
              &GetOrderInfoRequest::new()
                .page_index(1)
                .order_number_list(vec![order_id.to_string()])
                .finalize()
            );
            let res = block_on_unwrap(order);
            helpers::dump_json(res);
          })
        )

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
            let res = block_on_unwrap(client.ship_order(order_id, &action));
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

              let res = block_on_unwrap(client.get_order_info(&req));

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

      (report =>
        (submit =>
          (|m| {
            use newegg::report::*;
            let client = helpers::get_client();
            let report_type: &str = m.value_of("REPORT_TYPE").unwrap();

            let (operation_type, body) = match report_type {
              "inventory" => {
                match client.get_platform() {
                  NeweggPlatform::Newegg => (
                    "InternationalInventoryReportRequest",
                    json!({
                      "DailyInventoryReportCriteria": {
                        "FulfillType": "0",
                        "RequestType": "INTERNATIONAL_INVENTORY_REPORT",
                        "FileType": "CSV"
                      }
                    })
                  ),
                  _ => (
                    "DailyInventoryReportRequest",
                    json!({
                      "DailyInventoryReportCriteria": {
                        "FulfillType": "0",
                        "RequestType": "DAILY_INVENTORY_REPORT",
                        "FileType": "CSV"
                      }
                    })
                  )
                }
              },
              "price" => (
                "InternationalPriceReportRequest",
                json!({
                  "DailyPriceReportCriteria": {
                    "RequestType": "INTERNATIONAL_PRICE_REPORT",
                    "FileType": "CSV"
                  }
                })
              ),
              other => panic!("unknown report type: '{}'", other)
            };

            println!("Request:");
            let req = ReportRequest::new(operation_type, body);
            helpers::dump_json(&req);
            print!("\n");
            let res = block_on_unwrap(client.submit_report_request(&req));
            println!("Response:");
            helpers::dump_json(res);
          })
        )
        (get_status =>
          (|m| {
            use newegg::report::*;
            let client = helpers::get_client();
            let ids: Vec<&str> = m.values_of("ID").unwrap().collect();

            let res = block_on_unwrap(client.get_report_status(&ids, None));
            println!("Response:");
            helpers::dump_json(res);
          })
        )
        (get_result =>
          (|m| {
            use newegg::report::*;
            let client = helpers::get_client();
            let op_type: &str = m.value_of("OPERATION_TYPE").unwrap();
            let id: &str = m.value_of("ID").unwrap();

            let res = block_on_unwrap(client.get_report_result(op_type, id, 1, None));
            println!("Response:");
            helpers::dump_json(res);
          })
        )
        (get_report_file =>
          (|m| {
            use newegg::report::*;
            use std::io::Write;
            let client = helpers::get_client();
            let url: &str = m.value_of("URL").unwrap();
            let data = block_on_unwrap(client.get_report_file(url));
            std::io::stdout().write_all(&data).unwrap();
          })
        )
        (test_parse_us =>
          (|m| {
            use std::iter::FromIterator;
            let path = m.value_of("FILE").unwrap();

            println!("Loading csv file: {}", path);

            let file = std::fs::File::open(path).unwrap();
            let rows = Result::<Vec<report::InventoryReportRow>, csv::Error>::from_iter(
              csv::Reader::from_reader(file).into_deserialize(),
            ).unwrap();

            println!("{:#?}", rows);
          })
        )
        (test_parse_ca =>
          (|m| {
            use std::iter::FromIterator;
            let path = m.value_of("FILE").unwrap();

            println!("Loading csv file: {}", path);

            let file = std::fs::File::open(path).unwrap();
            let rows = Result::<Vec<report::CanInventoryReportRow>, csv::Error>::from_iter(
              csv::Reader::from_reader(file).into_deserialize(),
            ).unwrap();

            println!("{:#?}", rows);
          })
        )
      )

      (feed =>
        (get_status =>
          (|m| {
            use newegg::feed::*;
            let client = helpers::get_client();
            let ids: Vec<&str> = m.values_of("ID").unwrap().collect();

            let req = GetRequestStatus {
              request_id_list: RequestIdList {
                request_id: ids.into_iter().map(ToString::to_string).collect()
              },
              ..Default::default()
            };

            println!("Request:");
            helpers::dump_json(&req);

            let res = block_on_unwrap(client.get_feed_status(&req));
            println!("\nResponse:");
            helpers::dump_json(res);
          })
        )
        (get_result =>
          (|m| {
            use newegg::feed::*;
            let client = helpers::get_client();
            let id: &str = m.value_of("ID").unwrap();

            let res = block_on_unwrap(client.get_feed_result::<serde_json::Value>(id));
            println!("Response:");
            helpers::dump_json(res);
          })
        )
        (test_inventory_update =>
          (|_| {
            use newegg::feed::*;
            use newegg::feed::message::*;
            let client = helpers::get_client();

            let msg = InventoryUpdateFeedMessage {
              inventory: InventoryUpdateFeedInventory {
                item: vec![
                  InventoryUpdateFeedItem {
                    seller_part_number: "edifier-r1280t-fba".to_string(),
                    inventory: "1".to_string(),
                    warehouse_location: "USA".to_string(),
                    ..Default::default()
                  },
                  InventoryUpdateFeedItem {
                    seller_part_number: "edifier-r1280db-wood".to_string(),
                    inventory: "2".to_string(),
                    warehouse_location: "USA".to_string(),
                    ..Default::default()
                  }
                ],
              }
            };

            let req = RequestEnvelope::new(
              "BatchInventoryUpdate.xsd",
              &[("DocumentVersion", "2.0")],
              "Inventory",
              msg
            );

            println!("Request:");
            helpers::dump_json(&req);
            let res = block_on_unwrap(client.submit_feed("INVENTORY_DATA", &req));
            println!("\nResponse:");
            helpers::dump_json(res);
          })
        )
        (test_inventory_and_price_update =>
          (|_| {
            use newegg::feed::*;
            use newegg::feed::message::*;
            let client = helpers::get_client();

            let msg = InventoryAndPriceFeedMessage {
              inventory: InventoryAndPriceFeedInventory {
                item: vec![
                  InventoryAndPriceFeedFeedItem {
                    seller_part_number: "edifier-r1280t-fba".to_string(),
                    inventory: "1".to_string(),
                    shipping: "default".to_string(),
                    activation_mark: Some("True".to_string()),
                    ..Default::default()
                  },
                  InventoryAndPriceFeedFeedItem {
                    seller_part_number: "edifier-r1280db-wood".to_string(),
                    inventory: "2".to_string(),
                    shipping: "default".to_string(),
                    activation_mark: Some("True".to_string()),
                    ..Default::default()
                  }
                ],
              }
            };

            let req = RequestEnvelope::new(
              "Inventory.xsd",
              &[("DocumentVersion", "1.0")],
              "Inventory",
              msg
            ).inventory_and_price_data_overwrite(true);

            println!("Request:");
            helpers::dump_json(&req);
            let res = block_on_unwrap(client.submit_feed("INVENTORY_AND_PRICE_DATA", &req));
            println!("\nResponse:");
            helpers::dump_json(res);
          })
        )
      )
  }
}
