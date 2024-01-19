use core::fmt;

use codec::{Decode, Encode};
use frame_support::inherent::Vec;
use scale_info::TypeInfo;
use serde::{Deserialize, Deserializer};
use sp_core::ConstU32;
use sp_runtime::BoundedVec;

/// 快递100价格查询返回的数据
#[derive(Deserialize, Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
pub struct Kuaidi100Price {
	#[serde(deserialize_with = "de_string_to_bounded_bytes")]
	pub name: BoundedVec<u8, ConstU32<32>>,
	#[serde(deserialize_with = "de_string_to_bounded_bytes", rename(deserialize = "kuaidiCom"))]
	pub kuaidi_com: BoundedVec<u8, ConstU32<32>>,
	#[serde(deserialize_with = "de_string_to_u32", rename(deserialize = "totalprice"))]
	pub total_price: u32,
}

impl fmt::Debug for Kuaidi100Price {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Kuaidi100Price")
			.field("name", &sp_std::str::from_utf8(&self.name).map_err(|_| fmt::Error)?)
			.field("kuaidi_com", &sp_std::str::from_utf8(&self.kuaidi_com).map_err(|_| fmt::Error)?)
			.field("total_price", &self.total_price)
			.finish()
	}
}

/// 快递100价格查询响应
#[derive(Deserialize, Debug, Encode, Decode)]
pub struct Kuaidi100PriceResponse {
	pub status: i32,
	#[serde(deserialize_with = "de_vec_to_bounded_vec")]
	pub data: BoundedVec<Kuaidi100Price, ConstU32<10>>,
}

/// 反序列化字符串到 BoundedVec<u8, ConstU32<32>>
pub fn de_string_to_bounded_bytes<'de, D>(de: D) -> Result<BoundedVec<u8, ConstU32<32>>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: &str = Deserialize::deserialize(de)?;
	Ok(BoundedVec::<u8, ConstU32<32>>::try_from(s.as_bytes().to_vec())
		.map_err(|_| serde::de::Error::custom("BoundedVec error"))?)
}

/// 反序列化字符串到 u32
pub fn de_string_to_u32<'de, D>(de: D) -> Result<u32, D::Error>
where
	D: Deserializer<'de>,
{
	let s: &str = Deserialize::deserialize(de)?;
	let price = s.parse::<f32>().map_err(|_| serde::de::Error::custom("Invalid f32"))?;
	let price = (price * 100f32) as u32;
	Ok(price)
}

/// 反序列化 Vec<Kuaidi100Price> 到 BoundedVec<Kuaidi100Price, ConstU32<10>>
pub fn de_vec_to_bounded_vec<'de, D>(
	de: D,
) -> Result<BoundedVec<Kuaidi100Price, ConstU32<10>>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: Vec<Kuaidi100Price> = Deserialize::deserialize(de)?;
	let a = s
		.into_iter()
		.take(10)
		.collect::<Vec<Kuaidi100Price>>()
		.try_into()
		.map_err(|_| serde::de::Error::custom("BoundedVec error"))?;

	Ok(a)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn deserialize_works() {
		let s = r#"{"message":"success","status":200,"data":[{"sign":"6fVwnRiSBkdIMarket","logo":"https://cdn.kuaidi100.com/images/all/56/yuantong_new.png","defPrice":null,"totalprice":"5.5","comService":"yuantongnull","name":"圆通","costTotalPrice":"5.5","discountsAmount":"0","linePrice":"10","tips":"揽收高","discountsId":null,"kuaidiCom":"yuantong"},{"sign":"7lfSRRZ3S74KMarket","logo":"https://cdn.kuaidi100.com/images/all/56/shentong.png","defPrice":null,"totalprice":"6","comService":"shentongnull","name":"申通","costTotalPrice":"6","discountsAmount":"0","linePrice":"10","tips":"价格低","discountsId":null,"kuaidiCom":"shentong"},{"sign":"6pCHSRjHMLkeMarket","logo":"https://cdn.kuaidi100.com/images/all/56/yunda.png","defPrice":null,"totalprice":"6.5","comService":"yundanull","name":"韵达","costTotalPrice":"6.5","discountsAmount":"0","linePrice":"10","tips":"价格低","discountsId":null,"kuaidiCom":"yunda"},{"sign":"6f3JqRiSPjFLMarket","logo":"https://cdn.kuaidi100.com/images/all/56/jtexpress.png","defPrice":null,"totalprice":"10","comService":"jtexpressnull","name":"极兔","costTotalPrice":"7","discountsAmount":"3","linePrice":"10","tips":"价格低","discountsId":11937934935673,"kuaidiCom":"jtexpress"}]}"#;
		let response = serde_json::from_str::<Kuaidi100PriceResponse>(s).unwrap();

		assert!(response.status == 200);
		assert!(!response.data.is_empty());

		println!("{:#?}", response);
	}
}
