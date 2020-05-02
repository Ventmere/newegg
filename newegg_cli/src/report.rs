use bigdecimal::BigDecimal;
use serde::de::{self, Deserializer};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct CanInventoryReportRow {
  #[serde(rename = "Seller Part #")]
  pub seller_part_number: String,
  #[serde(rename = "NE Item #")]
  pub ne_item_number: String,
  #[serde(rename = "Currency")]
  pub currency: String,
  #[serde(rename = "MSRP")]
  pub msrp: Option<BigDecimal>,
  #[serde(rename = "MAP")]
  pub map: Option<BigDecimal>,
  #[serde(rename = "Checkout MAP")]
  #[serde(deserialize_with = "bool_from_string")]
  pub checkout_map: bool,
  #[serde(rename = "Selling Price")]
  pub selling_price: BigDecimal,
  #[serde(rename = "Inventory")]
  pub inventory: i32,
  #[serde(rename = "Fulfillment option")]
  pub fulfillment_option: String,
  #[serde(rename = "Shipping")]
  pub shipping: String,
  #[serde(rename = "Activation Mark")]
  #[serde(deserialize_with = "bool_from_string")]
  pub activation_mark: bool,
}

/// Deserialize bool from String with custom value mapping
fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
  D: Deserializer<'de>,
{
  match String::deserialize(deserializer)?.as_ref() {
    "True" => Ok(true),
    "False" => Ok(false),
    other => Err(de::Error::invalid_value(
      de::Unexpected::Str(other),
      &"True or False",
    )),
  }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct InventoryReportRow {
  #[serde(rename = "Seller Part #")]
  pub seller_part_number: String,
  #[serde(rename = "NE Item #")]
  pub ne_item_number: String,
  #[serde(rename = "Warehouse Location")]
  pub warehouse_location: String,
  #[serde(rename = "Fulfillment Option")]
  pub fulfillment_option: String,
  #[serde(rename = "Inventory")]
  pub inventory: i32,
}
