use helpers::NeweggDateTime;
use serde::ser::{Serialize, SerializeStruct, Serializer};

enum_number! {
  OrderStatus {
    Unshipped = 0,
    PartiallyShipped = 1,
    Shipped = 2,
    Invoiced = 3,
    Voided = 4,
  }
}

#[derive(Serialize, Deserialize)]
pub struct OrderInfo {
  #[serde(rename = "CustomerEmailAddress")]
  pub customer_email_address: String,
  #[serde(rename = "CustomerName")]
  pub customer_name: String,
  #[serde(rename = "CustomerPhoneNumber")]
  pub customer_phone_number: String,
  #[serde(rename = "DiscountAmount")]
  pub discount_amount: f64,
  #[serde(rename = "InvoiceNumber")]
  pub invoice_number: i64,
  #[serde(rename = "IsAutoVoid")]
  pub is_auto_void: bool,
  #[serde(rename = "ItemInfoList")]
  pub item_info_list: Vec<OrderItemInfoList>,
  #[serde(rename = "OrderDate")]
  pub order_date: NeweggDateTime,
  #[serde(rename = "OrderDownloaded")]
  pub order_downloaded: bool,
  #[serde(rename = "OrderItemAmount")]
  pub order_item_amount: f64,
  #[serde(rename = "OrderNumber")]
  pub order_number: i64,
  #[serde(rename = "OrderQty")]
  pub order_qty: i64,
  #[serde(rename = "OrderStatus")]
  pub order_status: OrderStatus,
  #[serde(rename = "OrderStatusDescription")]
  pub order_status_description: String,
  #[serde(rename = "OrderTotalAmount")]
  pub order_total_amount: f64,
  #[serde(rename = "PackageInfoList")]
  pub package_info_list: Vec<PackageInfoList>,
  #[serde(rename = "RefundAmount")]
  pub refund_amount: f64,
  #[serde(rename = "SellerID")]
  pub seller_id: String,
  #[serde(rename = "ShipService")]
  pub ship_service: String,
  #[serde(rename = "ShipToAddress1")]
  pub ship_to_address1: String,
  #[serde(rename = "ShipToAddress2")]
  pub ship_to_address2: String,
  #[serde(rename = "ShipToCityName")]
  pub ship_to_city_name: String,
  #[serde(rename = "ShipToCompany")]
  pub ship_to_company: String,
  #[serde(rename = "ShipToCountryCode")]
  pub ship_to_country_code: String,
  #[serde(rename = "ShipToFirstName")]
  pub ship_to_first_name: String,
  #[serde(rename = "ShipToLastName")]
  pub ship_to_last_name: String,
  #[serde(rename = "ShipToStateCode")]
  pub ship_to_state_code: String,
  #[serde(rename = "ShipToZipCode")]
  pub ship_to_zip_code: String,
  #[serde(rename = "ShippingAmount")]
  pub shipping_amount: f64,
}

#[derive(Serialize, Deserialize)]
pub struct OrderItemInfoList {
  #[serde(rename = "Description")]
  pub description: String,
  #[serde(rename = "ExtendShippingCharge")]
  pub extend_shipping_charge: f64,
  #[serde(rename = "ExtendUnitPrice")]
  pub extend_unit_price: f64,
  #[serde(rename = "MfrPartNumber")]
  pub mfr_part_number: String,
  #[serde(rename = "NeweggItemNumber")]
  pub newegg_item_number: String,
  #[serde(rename = "OrderedQty")]
  pub ordered_qty: i64,
  #[serde(rename = "SellerPartNumber")]
  pub seller_part_number: String,
  #[serde(rename = "ShippedQty")]
  pub shipped_qty: i64,
  #[serde(rename = "Status")]
  pub status: i64,
  #[serde(rename = "StatusDescription")]
  pub status_description: String,
  #[serde(rename = "UPCCode")]
  pub upc_code: String,
  #[serde(rename = "UnitPrice")]
  pub unit_price: f64,
}

#[derive(Serialize, Deserialize)]
pub struct PackageInfoList {
  #[serde(rename = "ItemInfoList")]
  pub item_info_list: Vec<PackageInfoListItemInfoList>,
  #[serde(rename = "PackageType")]
  pub package_type: String,
  #[serde(rename = "ShipCarrier")]
  pub ship_carrier: String,
  #[serde(rename = "ShipDate")]
  pub ship_date: NeweggDateTime,
  #[serde(rename = "ShipService")]
  pub ship_service: String,
  #[serde(rename = "TrackingNumber")]
  pub tracking_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct PackageInfoListItemInfoList {
  #[serde(rename = "MfrPartNumber")]
  pub mfr_part_number: String,
  #[serde(rename = "SellerPartNumber")]
  pub seller_part_number: String,
  #[serde(rename = "ShippedQty")]
  pub shipped_qty: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GetOrderInfoResponse {
  #[serde(rename = "ResponseDate")]
  pub response_date: String,
  #[serde(rename = "Memo")]
  pub memo: String,
  #[serde(rename = "IsSuccess")]
  pub is_success: bool,
  #[serde(rename = "OperationType")]
  pub operation_type: String,
  #[serde(rename = "SellerID")]
  pub seller_id: String,
  #[serde(rename = "ResponseBody")]
  pub response_body: ResponseBody,
}

#[derive(Serialize, Deserialize)]
pub struct PageInfo {
  #[serde(rename = "TotalCount")]
  pub total_count: i64,
  #[serde(rename = "TotalPageCount")]
  pub total_page_count: i64,
  #[serde(rename = "PageIndex")]
  pub page_index: i64,
  #[serde(rename = "PageSize")]
  pub page_size: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseBody {
  #[serde(rename = "PageInfo")]
  pub page_info: PageInfo,
  #[serde(rename = "OrderInfoList")]
  pub order_info_list: Vec<OrderInfo>,
}

#[derive(Serialize)]
pub struct RequestCriteria {
  #[serde(rename = "OrderNumberList")]
  pub order_number_list: Option<OrderNumberList>,
  #[serde(rename = "SellerOrderNumberList")]
  pub seller_order_number_list: Option<SellerOrderNumberList>,
  #[serde(rename = "Status")]
  pub status: Option<String>,
  #[serde(rename = "Type")]
  pub request_criteria_type: Option<String>,
  #[serde(rename = "OrderDateFrom")]
  pub order_date_from: Option<String>,
  #[serde(rename = "OrderDateTo")]
  pub order_date_to: Option<String>,
  #[serde(rename = "OrderDownloaded")]
  pub order_downloaded: Option<i64>,
  #[serde(rename = "CountryCode")]
  pub country_code: Option<String>,
  #[serde(rename = "PremierOrder")]
  pub premier_order: Option<String>,
}

#[derive(Serialize)]
pub struct OrderNumberList {
  #[serde(rename = "OrderNumber")]
  pub order_number: Vec<String>,
}

#[derive(Serialize)]
pub struct SellerOrderNumberList {
  #[serde(rename = "SellerOrderNumber")]
  pub seller_order_number: Vec<String>,
}

#[derive(Serialize)]
pub struct RequestBody {
  #[serde(rename = "PageIndex")]
  pub page_index: Option<i64>,
  #[serde(rename = "PageSize")]
  pub page_size: Option<i64>,
  #[serde(rename = "RequestCriteria")]
  pub request_criteria: RequestCriteria,
}

pub struct GetOrderInfoRequest {
  pub request_body: RequestBody,
}

impl Serialize for GetOrderInfoRequest {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("GetOrderInfoRequest", 2)?;
    state.serialize_field("OperationType", "GetOrderInfoRequest")?;
    state.serialize_field("RequestBody", &self.request_body)?;
    state.end()
  }
}

#[derive(Serialize, Deserialize)]
pub enum UpdateStatusAction {
  #[serde(rename = "1")]
  CancelOrder,

  #[serde(rename = "2")]
  ShipOrder,
}

#[derive(Serialize, Deserialize)]
pub enum CancelReasonCode {
  #[serde(rename = "24")]
  OutOfStock,
  #[serde(rename = "72")]
  CustomerRequestedToCancel,
  #[serde(rename = "74")]
  UnableToFulfillOrder,
}

#[derive(Serialize, Deserialize)]
pub struct ShipOrderActionValue {
  #[serde(rename = "Shipment")]
  shipment: Shipment,
}

#[derive(Serialize, Deserialize)]
pub struct Shipment {
  #[serde(rename = "Header")]
  header: ShipmentHeader,
  #[serde(rename = "PackageList")]
  package_list: PackageList,
}

#[derive(Serialize, Deserialize)]
pub struct ShipmentHeader {
  #[serde(rename = "SellerID")]
  seller_id: String,
  #[serde(rename = "SONumber")]
  so_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct PackageList {
  #[serde(rename = "Package")]
  package: Vec<Package>,
}

#[derive(Serialize, Deserialize)]
pub struct Package {
  #[serde(rename = "TrackingNumber")]
  tracking_number: String,
  #[serde(rename = "ShipCarrier")]
  ship_carrier: String,
  #[serde(rename = "ShipService")]
  ship_service: String,
  #[serde(rename = "ItemList")]
  item_list: ItemList,
}

#[derive(Serialize, Deserialize)]
pub struct ItemList {
  #[serde(rename = "Item")]
  item: ItemUnion,
}

#[derive(Serialize, Deserialize)]
pub struct ItemElement {
  #[serde(rename = "SellerPartNumber")]
  seller_part_number: String,
  #[serde(rename = "ShippedQty")]
  shipped_qty: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemUnion {
  ItemElement(ItemElement),
  ItemElementArray(Vec<ItemElement>),
}
