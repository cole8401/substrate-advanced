#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	pub use frame_support::pallet_prelude::*;
	pub use frame_system::pallet_prelude::*;
	pub use sp_std::prelude::*;
	pub use super::WeightInfo;

	// 通过继承拥有了 frame_system::Config 里定义的数据类型
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// pallet::constant 用于声明这是个链上的常量
		#[pallet::constant]
		/// The maximum length of claim that can be added.
		type MaxClaimLength: Get<u32>;

		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// 定义存储项
	#[pallet::storage]
	pub type Proofs<T: Config> = StorageMap<
		_,
		// 密码安全的hash算法
		Blake2_128Concat,
		BoundedVec<u8, T::MaxClaimLength>,
		(T::AccountId, T::BlockNumber),
	>;

	// 定义事件
	#[pallet::event]
	// 生成工具函数
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
		ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
		ClaimTransfered(T::AccountId, T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
	}

	// 定义错误
	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}

	// 用于定义回调函数，在区块的不同时期执行
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// 定义可调用函数
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_claim(claim.len() as u32))]
		pub fn create_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResultWithPostInfo {
			// 验证签名
			let sender = ensure_signed(origin)?;

			// 验证是否已经存储过
			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			Proofs::<T>::insert(
				&claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);

			Self::deposit_event(Event::ClaimCreated(sender, claim));

			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::revoke_claim(claim.len() as u32))]
		pub fn revoke_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResultWithPostInfo {
			// 验证签名
			let sender = ensure_signed(origin)?;

			// 校验是否已经存在存证
			let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

			// 验证存证的所有者是否是当前用户
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			// 从存储里删除存证
			Proofs::<T>::remove(&claim);

			// 发送存证吊销事件
			Self::deposit_event(Event::ClaimRevoked(sender, claim));

			Ok(().into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::transfer_claim(claim.len() as u32))]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			claim: BoundedVec<u8, T::MaxClaimLength>,
			dest: T::AccountId,
		) -> DispatchResultWithPostInfo {
			// 验证签名
			let sender = ensure_signed(origin)?;

			// 校验是否已经存在存证
			let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

			// 验证存证的所有者是否是当前用户
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			// 从存储里删除存证
			Proofs::<T>::insert(&claim, (dest, frame_system::Pallet::<T>::block_number()));

			// 发送存证转移事件
			Self::deposit_event(Event::ClaimTransfered(owner, sender, claim));

			Ok(().into())
		}
	}
}
