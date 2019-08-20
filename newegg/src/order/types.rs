use chrono::{DateTime, Utc};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_derive::{Deserialize, Serialize};

use crate::helpers::NeweggDateTime;

enum_number! {
  OrderStatus {
    Unshipped = 0,
    PartiallyShipped = 1,
    Shipped = 2,
    Invoiced = 3,
    Voided = 4,
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageInfoListItemInfoList {
  #[serde(rename = "MfrPartNumber")]
  pub mfr_part_number: String,
  #[serde(rename = "SellerPartNumber")]
  pub seller_part_number: String,
  #[serde(rename = "ShippedQty")]
  pub shipped_qty: i64,
}

#[derive(Debug, Serialize, Deserialize)]
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

impl GetOrderInfoResponse {
  pub fn total(&self) -> i64 {
    self.response_body.page_info.total_count
  }

  pub fn len(&self) -> usize {
    self
      .response_body
      .order_info_list
      .as_ref()
      .map(|l| l.len())
      .unwrap_or(0)
  }

  pub fn is_empty(&self) -> bool {
    self
      .response_body
      .order_info_list
      .as_ref()
      .map(|l| l.is_empty())
      .unwrap_or(true)
  }

  pub fn info_list(&self) -> Option<&Vec<OrderInfo>> {
    self.response_body.order_info_list.as_ref()
  }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody {
  #[serde(rename = "PageInfo")]
  pub page_info: PageInfo,
  #[serde(rename = "OrderInfoList")]
  pub order_info_list: Option<Vec<OrderInfo>>,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct RequestCriteria {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "OrderNumberList")]
  pub order_number_list: Option<OrderNumberList>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "SellerOrderNumberList")]
  pub seller_order_number_list: Option<SellerOrderNumberList>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "Status")]
  pub status: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "Type")]
  pub type_: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "OrderDateFrom")]
  pub order_date_from: Option<NeweggDateTime>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "OrderDateTo")]
  pub order_date_to: Option<NeweggDateTime>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "OrderDownloaded")]
  pub order_downloaded: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "CountryCode")]
  pub country_code: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "PremierOrder")]
  pub premier_order: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct OrderNumberList {
  #[serde(rename = "OrderNumber")]
  pub order_number: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SellerOrderNumberList {
  #[serde(rename = "SellerOrderNumber")]
  pub seller_order_number: Vec<String>,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct RequestBody {
  #[serde(rename = "PageIndex")]
  pub page_index: Option<i32>,
  #[serde(rename = "PageSize")]
  pub page_size: Option<i32>,
  #[serde(rename = "RequestCriteria")]
  pub request_criteria: RequestCriteria,
}

#[derive(Default, Clone)]
pub struct GetOrderInfoRequest {
  pub request_body: RequestBody,
}

pub struct GetOrderInfoRequestBuilder {
  inner: GetOrderInfoRequest,
}

enum_number! {
  OrderTypeFilter {
    All = 0,
    ShippedByNewEgg = 1,
    ShippedBySeller = 2,
    MultiChannel = 3,
  }
}

enum_number! {
  PremierOrderFilter {
    All = 0,
    PremierOrderOnly = 1,
    NoPremierOrder = 2,
  }
}

impl GetOrderInfoRequestBuilder {
  pub fn finalize(&mut self) -> GetOrderInfoRequest {
    let mut req = ::std::mem::replace(&mut self.inner, GetOrderInfoRequest::default());
    if req.request_body.page_size.is_none() {
      req.request_body.page_size = Some(100);
    }
    req
  }

  pub fn page_index(&mut self, v: i32) -> &mut Self {
    self.inner.request_body.page_index = Some(v);
    self
  }

  pub fn page_size(&mut self, v: i32) -> &mut Self {
    self.inner.request_body.page_size = Some(v);
    self
  }

  pub fn order_number_list(&mut self, v: Vec<String>) -> &mut Self {
    self.inner.request_body.request_criteria.order_number_list =
      Some(OrderNumberList { order_number: v });
    self
  }

  pub fn seller_order_number_list(&mut self, v: Vec<String>) -> &mut Self {
    self
      .inner
      .request_body
      .request_criteria
      .seller_order_number_list = Some(SellerOrderNumberList {
      seller_order_number: v,
    });
    self
  }

  pub fn order_status(&mut self, v: OrderStatus) -> &mut Self {
    self.inner.request_body.request_criteria.status = Some(v.as_str().to_owned());
    self
  }

  pub fn order_type(&mut self, v: OrderTypeFilter) -> &mut Self {
    self.inner.request_body.request_criteria.type_ = Some(v.as_str().to_owned());
    self
  }

  pub fn order_date_from(&mut self, v: DateTime<Utc>) -> &mut Self {
    self.inner.request_body.request_criteria.order_date_from = Some(NeweggDateTime::from_utc(v));
    self
  }

  pub fn order_date_to(&mut self, v: DateTime<Utc>) -> &mut Self {
    self.inner.request_body.request_criteria.order_date_to = Some(NeweggDateTime::from_utc(v));
    self
  }

  pub fn order_downloaded(&mut self, v: bool) -> &mut Self {
    self.inner.request_body.request_criteria.order_downloaded = Some(if v { 1 } else { 0 });
    self
  }

  pub fn country_code(&mut self, v: &str) -> &mut Self {
    self.inner.request_body.request_criteria.country_code = Some(v.to_owned());
    self
  }

  pub fn premier_order(&mut self, v: PremierOrderFilter) -> &mut Self {
    self.inner.request_body.request_criteria.premier_order = Some(v.as_str().to_owned());
    self
  }
}

impl GetOrderInfoRequest {
  pub fn new() -> GetOrderInfoRequestBuilder {
    GetOrderInfoRequestBuilder {
      inner: GetOrderInfoRequest::default(),
    }
  }
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

#[derive(Debug, Serialize, Deserialize)]
pub enum UpdateStatusActionType {
  #[serde(rename = "1")]
  CancelOrder,

  #[serde(rename = "2")]
  ShipOrder,
}

#[derive(Debug, Serialize)]
pub struct CancelOrderAction {
  #[serde(rename = "Action")]
  action: UpdateStatusActionType,
  #[serde(rename = "Value")]
  value: CancelOrderReasonCode,
}

impl CancelOrderAction {
  pub fn new(reason: CancelOrderReasonCode) -> Self {
    Self {
      action: UpdateStatusActionType::CancelOrder,
      value: reason,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CancelOrderReasonCode {
  #[serde(rename = "24")]
  OutOfStock,
  #[serde(rename = "72")]
  CustomerRequestedToCancel,
  #[serde(rename = "74")]
  UnableToFulfillOrder,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrderResponse {
  #[serde(rename = "IsSuccess")]
  is_success_: String,
  #[serde(rename = "Result")]
  result: Option<CancelOrderResponseResult>,
}

impl CancelOrderResponse {
  pub fn is_success(&self) -> bool {
    self.is_success_ == "true"
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrderResponseResult {
  #[serde(rename = "OrderNumber")]
  order_number: String,
  #[serde(rename = "SellerID")]
  seller_id: String,
  #[serde(rename = "OrderStatus")]
  order_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipOrderAction {
  #[serde(rename = "Action")]
  action: UpdateStatusActionType,
  #[serde(rename = "Value")]
  value: ShipOrderActionValue,
}

impl ShipOrderAction {
  pub fn new(seller_id: &str, order_number: i64) -> ShipOrderActionBuilder {
    ShipOrderActionBuilder::new(seller_id, order_number)
  }
}

pub struct ShipOrderActionBuilder {
  inner: ShipOrderActionValue,
}

impl ShipOrderActionBuilder {
  pub fn new(seller_id: &str, order_number: i64) -> Self {
    ShipOrderActionBuilder {
      inner: ShipOrderActionValue::new(seller_id, order_number),
    }
  }

  pub fn add_package(&mut self, package: Package) -> &mut Self {
    self.inner.shipment.package_list.package.push(package);
    self
  }

  pub fn finalize(&mut self) -> ShipOrderAction {
    let replace = ShipOrderActionValue::new(
      &self.inner.shipment.header.seller_id,
      self.inner.shipment.header.so_number,
    );
    let value = ::std::mem::replace(&mut self.inner, replace);
    ShipOrderAction {
      action: UpdateStatusActionType::ShipOrder,
      value,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipOrderActionValue {
  #[serde(rename = "Shipment")]
  shipment: Shipment,
}

impl ShipOrderActionValue {
  pub fn new(seller_id: &str, order_number: i64) -> Self {
    ShipOrderActionValue {
      shipment: Shipment {
        header: ShipmentHeader {
          seller_id: seller_id.to_owned(),
          so_number: order_number,
        },
        package_list: PackageList { package: vec![] },
      },
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shipment {
  #[serde(rename = "Header")]
  header: ShipmentHeader,
  #[serde(rename = "PackageList")]
  package_list: PackageList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipmentHeader {
  #[serde(rename = "SellerID")]
  seller_id: String,
  #[serde(rename = "SONumber")]
  so_number: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageList {
  #[serde(rename = "Package")]
  package: Vec<Package>,
}

#[derive(Debug, Clone)]
pub enum ShipCarrier {
  Ups,
  UpsMi,
  FedEx,
  Dhl,
  Usps,
  Other(String),
}

impl ToString for ShipCarrier {
  fn to_string(&self) -> String {
    match *self {
      ShipCarrier::Ups => "UPS".to_string(),
      ShipCarrier::UpsMi => "UPS MI".to_string(),
      ShipCarrier::FedEx => "FedEx".to_string(),
      ShipCarrier::Dhl => "DHL".to_string(),
      ShipCarrier::Usps => "USPS".to_string(),
      ShipCarrier::Other(ref other) => other.to_string(),
    }
  }
}

impl<T> From<T> for ShipCarrier
where
  T: AsRef<str>,
{
  fn from(v: T) -> Self {
    let normalized = v
      .as_ref()
      .to_lowercase()
      .split_whitespace()
      .collect::<Vec<&str>>()
      .join(" ");
    match normalized.as_ref() {
      "ups" => ShipCarrier::Ups,
      "ups mi" => ShipCarrier::UpsMi,
      "fedex" => ShipCarrier::FedEx,
      "dhl" => ShipCarrier::Dhl,
      "usps" => ShipCarrier::Usps,
      _ => ShipCarrier::Other(v.as_ref().trim().to_string()),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
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

impl Package {
  pub fn new(carrier: ShipCarrier, service: &str, tracking: &str) -> PackageBuilder {
    PackageBuilder::new(carrier, service, tracking)
  }
}

pub struct PackageBuilder {
  tracking_number: String,
  ship_carrier: ShipCarrier,
  ship_service: String,
  items: Vec<ItemElement>,
}

impl PackageBuilder {
  pub fn new(carrier: ShipCarrier, service: &str, tracking: &str) -> Self {
    PackageBuilder {
      tracking_number: tracking.to_owned(),
      ship_carrier: carrier,
      ship_service: service.to_owned(),
      items: vec![],
    }
  }

  pub fn add_item(&mut self, seller_part_number: &str, qty: i32) -> &mut Self {
    self.items.push(ItemElement {
      seller_part_number: seller_part_number.to_owned(),
      shipped_qty: qty.to_string(),
    });
    self
  }

  pub fn finalize(&mut self) -> Option<Package> {
    let replace = Self::new(
      self.ship_carrier.clone(),
      &self.ship_service,
      &self.tracking_number,
    );
    let mut b = ::std::mem::replace(self, replace);
    if b.items.is_empty() {
      return None;
    }

    let item_list = if b.items.len() == 1 {
      match b.items.pop() {
        Some(item) => ItemList {
          item: ItemUnion::ItemElement(item),
        },
        None => return None,
      }
    } else {
      ItemList {
        item: ItemUnion::ItemElementArray(b.items),
      }
    };

    Some(Package {
      tracking_number: b.tracking_number,
      ship_carrier: b.ship_carrier.to_string(),
      ship_service: b.ship_service,
      item_list,
    })
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemList {
  #[serde(rename = "Item")]
  item: ItemUnion,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemElement {
  #[serde(rename = "SellerPartNumber")]
  seller_part_number: String,
  #[serde(rename = "ShippedQty")]
  shipped_qty: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemUnion {
  ItemElement(ItemElement),
  ItemElementArray(Vec<ItemElement>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipOrderResponse {
  /// Used to identify the operation of order update is successful.
  /// Always returns true.
  #[serde(rename = "IsSuccess")]
  is_success_: bool,
  #[serde(rename = "PackageProcessingSummary")]
  package_processing_summary: PackageProcessingSummary,
  #[serde(rename = "Result")]
  result: ShipOrderResponseResult,
}

impl ShipOrderResponse {
  pub fn is_success(&self) -> bool {
    self.package_processing_summary.fail_count == 0
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageProcessingSummary {
  #[serde(rename = "FailCount")]
  fail_count: i64,
  #[serde(rename = "SuccessCount")]
  success_count: i64,
  #[serde(rename = "TotalPackageCount")]
  total_package_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipOrderResponseResult {
  #[serde(rename = "OrderNumber")]
  order_number: String,
  #[serde(rename = "OrderStatus")]
  order_status: String,
  #[serde(rename = "SellerID")]
  seller_id: String,
  #[serde(rename = "Shipment")]
  shipment: ShipOrderResponseShipment,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipOrderResponseShipment {
  #[serde(rename = "PackageList")]
  package_list: Vec<ShipOrderResponsePackageList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipOrderResponsePackageList {
  #[serde(rename = "ItemList")]
  item_list: Vec<ShipOrderResponseItemList>,
  #[serde(rename = "ProcessResult")]
  process_result: String,
  #[serde(rename = "ProcessStatus")]
  process_status: bool,
  #[serde(rename = "ShipDate")]
  ship_date: String,
  #[serde(rename = "TrackingNumber")]
  tracking_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipOrderResponseItemList {
  #[serde(rename = "NeweggItemNumber")]
  newegg_item_number: Option<String>,
  #[serde(rename = "SellerPartNumber")]
  seller_part_number: String,
  #[serde(rename = "ShippedQty")]
  shipped_qty: i32,
}
