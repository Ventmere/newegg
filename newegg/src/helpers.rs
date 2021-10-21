use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::{Tz, US::Pacific};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;

use crate::result::{NeweggResult, NeweggError};

/// Newegg Marketplace API requires the datetime field to be in Pacific Standard Time
/// in all request and response content. Please ensure in all your files and requests,
/// the datetime field are specified in Pacific Standard Time.
#[derive(Debug, Clone, Copy)]
pub struct NeweggDateTime(DateTime<Tz>);

const FORMAT: &'static str = "%m/%d/%Y %H:%M:%S";

impl NeweggDateTime {
  pub fn as_utc(&self) -> DateTime<Utc> {
    self.0.with_timezone(&Utc)
  }

  pub fn from_utc(datetime: DateTime<Utc>) -> Self {
    NeweggDateTime(datetime.with_timezone(&Pacific))
  }
}

impl Serialize for NeweggDateTime {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.0.format(FORMAT).to_string().serialize::<S>(serializer)
  }
}

struct NeweggDateTimeVisitor;

impl<'de> Visitor<'de> for NeweggDateTimeVisitor {
  type Value = NeweggDateTime;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("an datetime string like '02/12/2018 12:16:43'")
  }

  fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    let pdt = Pacific
      .datetime_from_str(s, FORMAT)
      .map_err(|err| E::custom(format!("parse date error: {}", err)))?;
    Ok(NeweggDateTime(pdt))
  }
}

impl<'de> Deserialize<'de> for NeweggDateTime {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(NeweggDateTimeVisitor)
  }
}

/// https://serde.rs/enum-number.html
#[macro_export]
macro_rules! enum_number {
  ($name:ident { $($variant:ident = $value:expr, )* }) => {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum $name {
      $($variant = $value,)*
    }

    impl $name {
      pub fn as_str(&self) -> &'static str {
        match *self {
          $( $name::$variant => stringify!($value), )*
        }
      }
    }

    impl ::serde::Serialize for $name {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ::serde::Serializer
      {
        // Serialize the enum as a u64.
        serializer.serialize_u64(*self as u64)
      }
    }

    impl<'de> ::serde::Deserialize<'de> for $name {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: ::serde::Deserializer<'de>
      {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
          type Value = $name;

          fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            formatter.write_str("positive integer")
          }

          fn visit_u64<E>(self, value: u64) -> Result<$name, E>
          where E: ::serde::de::Error
          {
            // Rust does not come with a simple way of converting a
            // number to an enum, so use a big `match`.
            match value {
              $( $value => Ok($name::$variant), )*
              _ => Err(E::custom(
                format!("unknown {} value: {}",
                stringify!($name), value)
              )),
            }
          }
        }

        // Deserialize the enum from a u64.
        deserializer.deserialize_u64(Visitor)
      }
    }
  }
}

pub async fn block<F, R>(f: F) -> NeweggResult<R>
where
  F: FnOnce() -> NeweggResult<R> + Send + 'static,
  R: Send + 'static,
{
  tokio::task::spawn_blocking(f).await.map_err(NeweggError::from)?
}