#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::{
	cmp::{Eq, PartialEq},
	prelude::Vec,
};
pub use scale_info::TypeInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub use auction::{Auction, AuctionHandler, AuctionInfo, OnNewBidResult};
pub use currency::{
	BalanceStatus, BasicCurrency, BasicCurrencyExtended, BasicLockableCurrency,
	BasicReservableCurrency, LockIdentifier, MultiCurrency, MultiCurrencyExtended,
	MultiLockableCurrency, MultiReservableCurrency, OnDust,
};
pub use data_provider::{DataFeeder, DataProvider, DataProviderExtended};
pub use get_by_key::GetByKey;
pub use nft::NFT;
pub use price::{DefaultPriceProvider, PriceProvider};
pub use rewards::RewardHandler;

pub mod account;
pub mod arithmetic;
pub mod auction;
pub mod currency;
pub mod data_provider;
pub mod get_by_key;
pub mod nft;
pub mod price;
pub mod rewards;

/// New data handler
#[impl_trait_for_tuples::impl_for_tuples(30)]
pub trait OnNewData<AccountId, Key, Value> {
	/// New data is available
	fn on_new_data(who: &AccountId, key: &Key, value: &Value);
}

/// Combine data provided by operators
pub trait CombineData<Key, TimestampedValue> {
	/// Combine data provided by operators
	fn combine_data(
		key: &Key,
		values: Vec<TimestampedValue>,
		prev_value: Option<TimestampedValue>,
	) -> Option<TimestampedValue>;
}

/// Indicate if should change a value
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum Change<Value> {
	/// No change.
	NoChange,
	/// Changed to new value.
	NewValue(Value),
}

#[derive(Encode, Decode, RuntimeDebug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TimestampedValue<Value: Ord + PartialOrd, Moment> {
	pub value: Value,
	pub timestamp: Moment,
}

#[impl_trait_for_tuples::impl_for_tuples(30)]
pub trait Happened<T> {
	fn happened(t: &T);
}
