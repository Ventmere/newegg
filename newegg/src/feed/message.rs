use serde_derive::{Deserialize, Serialize};

/// https://developer.newegg.com/newegg_marketplace_api/datafeed_management/submit_feed/inventory_update_feed/
#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryUpdateFeedMessage {
  #[serde(rename = "Inventory")]
  pub inventory: InventoryUpdateFeedInventory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryUpdateFeedInventory {
  #[serde(rename = "Item")]
  pub item: Vec<InventoryUpdateFeedItem>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryUpdateFeedItem {
  #[serde(rename = "SellerPartNumber")]
  pub seller_part_number: String,
  #[serde(rename = "NeweggItemNumber")]
  pub newegg_item_number: Option<String>,
  #[serde(rename = "WarehouseLocation")]
  pub warehouse_location: String,
  #[serde(rename = "FulfillmentOption")]
  pub fulfillment_option: Option<String>,
  #[serde(rename = "Inventory")]
  pub inventory: String,
}

/// https://developer.newegg.com/newegg_marketplace_api/datafeed_management/submit_feed/inventory_and_price_feed/
#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryAndPriceFeedMessage {
  #[serde(rename = "Inventory")]
  pub inventory: InventoryAndPriceFeedInventory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryAndPriceFeedInventory {
  #[serde(rename = "Item")]
  pub item: Vec<InventoryAndPriceFeedFeedItem>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InventoryAndPriceFeedFeedItem {
  #[serde(rename = "SellerPartNumber")]
  pub seller_part_number: String,
  #[serde(rename = "SellingPrice")]
  pub selling_price: Option<String>,
  #[serde(rename = "Inventory")]
  pub inventory: String,
  #[serde(rename = "FulfillmentOption")]
  pub fulfillment_option: Option<String>,
  #[serde(rename = "Shipping")]
  pub shipping: String,
  #[serde(rename = "ActivationMark")]
  pub activation_mark: Option<String>,
  #[serde(rename = "Currency")]
  pub currency: Option<String>,
  #[serde(rename = "NeweggItemNumber")]
  pub newegg_item_number: Option<String>,
}
