use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use once_cell::sync::Lazy;
use sp_runtime::traits::AccountIdConversion;

const KITTY_ID: u32 = 0;
const KITTY_NAME: [u8; 8] = *b"test0000";
const ACCOUNT_ID: u64 = 1;
const ACCOUNT_ID2: u64 = 2;
const ACCOUNT_BALANCE: u128 = 100000;
const ACCOUNT_BALANCE2: u128 = 100000;

static PALLET_ACCOUNT_ID: Lazy<u64> = Lazy::new(|| {
	KittyPalletId::get().into_account_truncating()
});
const PALLET_BALANCE: u128 = 0;


#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), ACCOUNT_ID, ACCOUNT_BALANCE, 0));

		// 成功创建一个 kitty 的情况
		assert_eq!(KittiesModule::next_kitty_id(), KITTY_ID);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(ACCOUNT_ID), KITTY_NAME));
		assert_eq!(KittiesModule::next_kitty_id(), KITTY_ID + 1);
		assert_eq!(Balances::free_balance(ACCOUNT_ID), ACCOUNT_BALANCE - EXISTENTIAL_DEPOSIT * 10);
		assert_eq!(
			Balances::free_balance(*PALLET_ACCOUNT_ID),
			PALLET_BALANCE + EXISTENTIAL_DEPOSIT * 10
		);

		assert!(KittiesModule::kitties(KITTY_ID).is_some());
		assert!(KittiesModule::kitties(KITTY_ID).unwrap().name == KITTY_NAME);

		assert_eq!(KittiesModule::kitty_owner(KITTY_ID), Some(ACCOUNT_ID));
		assert!(KittiesModule::kitty_parents(KITTY_ID).is_none());

		let kitty = KittiesModule::kitties(KITTY_ID).unwrap();

		// Assert that the correct event was deposited
		System::assert_last_event(
			Event::KittyCreated { who: ACCOUNT_ID, kitty_id: KITTY_ID, kitty }.into(),
		);

		// 当 kitty_id 达到阈值，创建失败
		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(ACCOUNT_ID), KITTY_NAME),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), ACCOUNT_ID, ACCOUNT_BALANCE, 0));

		// 当两个 kitty_id 相同时, breed 失败
		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(ACCOUNT_ID), KITTY_ID, KITTY_ID, KITTY_NAME),
			Error::<Test>::SamedKittyId
		);
		assert_eq!(Balances::free_balance(ACCOUNT_ID), ACCOUNT_BALANCE);
		assert_eq!(Balances::free_balance(*PALLET_ACCOUNT_ID), PALLET_BALANCE);

		// 当两个 kitty_id 不同, 但 kitty 不存在时, breed 失败
		assert_noop!(
			KittiesModule::breed(
				RuntimeOrigin::signed(ACCOUNT_ID),
				KITTY_ID,
				KITTY_ID + 1,
				KITTY_NAME
			),
			Error::<Test>::InvalidKittyId
		);
		assert_eq!(Balances::free_balance(ACCOUNT_ID), ACCOUNT_BALANCE);
		assert_eq!(Balances::free_balance(*PALLET_ACCOUNT_ID), PALLET_BALANCE);

		// 当两个 kitty_id 不同, kitty 存在时, breed 成功
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(ACCOUNT_ID), KITTY_NAME));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(ACCOUNT_ID), KITTY_NAME));
		assert_eq!(KittiesModule::next_kitty_id(), KITTY_ID + 2);
		assert_eq!(
			Balances::free_balance(ACCOUNT_ID),
			ACCOUNT_BALANCE - 2 * EXISTENTIAL_DEPOSIT * 10
		);
		assert_eq!(
			Balances::free_balance(*PALLET_ACCOUNT_ID),
			PALLET_BALANCE + 2 * EXISTENTIAL_DEPOSIT * 10
		);

		assert_ok!(KittiesModule::breed(
			RuntimeOrigin::signed(ACCOUNT_ID),
			KITTY_ID,
			KITTY_ID + 1,
			KITTY_NAME
		));
		assert_eq!(
			Balances::free_balance(ACCOUNT_ID),
			ACCOUNT_BALANCE - 3 * EXISTENTIAL_DEPOSIT * 10
		);
		assert_eq!(
			Balances::free_balance(*PALLET_ACCOUNT_ID),
			PALLET_BALANCE + 3 * EXISTENTIAL_DEPOSIT * 10
		);

		let breed_kitty_id = 2;
		assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
		assert!(KittiesModule::kitties(breed_kitty_id).is_some());
		assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(ACCOUNT_ID));
		assert_eq!(KittiesModule::kitty_parents(breed_kitty_id), Some((KITTY_ID, KITTY_ID + 1)));

		let breed_kitty = KittiesModule::kitties(breed_kitty_id).unwrap();
		System::assert_last_event(
			Event::KittyBred { who: ACCOUNT_ID, kitty_id: 2, kitty: breed_kitty }.into(),
		);
	});
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		// 账号充值
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), ACCOUNT_ID, ACCOUNT_BALANCE, 0));

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(ACCOUNT_ID), KITTY_NAME));
		assert_eq!(KittiesModule::kitty_owner(KITTY_ID), Some(ACCOUNT_ID));

		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(ACCOUNT_ID2), ACCOUNT_ID2, KITTY_ID),
			Error::<Test>::NotOwner
		);

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(ACCOUNT_ID), ACCOUNT_ID2, KITTY_ID));
		assert_eq!(KittiesModule::kitty_owner(KITTY_ID), Some(ACCOUNT_ID2));
		System::assert_last_event(
			Event::KittyTransferred { who: ACCOUNT_ID, recipient: ACCOUNT_ID2, kitty_id: 0 }.into(),
		);

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(ACCOUNT_ID2), ACCOUNT_ID2, KITTY_ID));
		System::assert_last_event(
			Event::KittyTransferred { who: ACCOUNT_ID2, recipient: ACCOUNT_ID2, kitty_id: 0 }.into(),
		);
	});
}

#[test]
fn it_works_for_sale() {
	new_test_ext().execute_with(|| {
		// 账户充值
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), ACCOUNT_ID, ACCOUNT_BALANCE, 0));

		// 当不存在 kitty 时失败
		assert_noop!(
			KittiesModule::sale(RuntimeOrigin::signed(ACCOUNT_ID), KITTY_ID),
			Error::<Test>::InvalidKittyId
		);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(ACCOUNT_ID), KITTY_NAME));
		assert_eq!(Balances::free_balance(ACCOUNT_ID), ACCOUNT_BALANCE - EXISTENTIAL_DEPOSIT * 10);
		assert_eq!(
			Balances::free_balance(*PALLET_ACCOUNT_ID),
			PALLET_BALANCE + EXISTENTIAL_DEPOSIT * 10
		);
		// 当所有者不正确时失败
		assert_noop!(
			KittiesModule::sale(RuntimeOrigin::signed(ACCOUNT_ID2), KITTY_ID),
			Error::<Test>::NotOwner
		);

		// 所有者正确，成功
		assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(ACCOUNT_ID), KITTY_ID));
		assert!(KittiesModule::kitty_on_sale(KITTY_ID).is_some());
		System::assert_last_event(Event::KittyOnSale { who: ACCOUNT_ID, kitty_id: 0 }.into());

		// 重复 sale, 失败
		assert_noop!(
			KittiesModule::sale(RuntimeOrigin::signed(ACCOUNT_ID), KITTY_ID),
			Error::<Test>::AlreadyOnSale
		);
	});
}

#[test]
fn it_works_for_buy() {
	new_test_ext().execute_with(|| {
		// 账户充值
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), ACCOUNT_ID, ACCOUNT_BALANCE, 0));
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), ACCOUNT_ID2, ACCOUNT_BALANCE2, 0));

		// 当不存在 kitty 时失败
		assert_noop!(
			KittiesModule::buy(RuntimeOrigin::signed(ACCOUNT_ID), KITTY_ID),
			Error::<Test>::InvalidKittyId
		);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(ACCOUNT_ID), KITTY_NAME));
		assert_eq!(Balances::free_balance(ACCOUNT_ID), ACCOUNT_BALANCE - EXISTENTIAL_DEPOSIT * 10);

		// 当购买者与所有者相同时失败
		assert_noop!(
			KittiesModule::buy(RuntimeOrigin::signed(ACCOUNT_ID), KITTY_ID),
			Error::<Test>::AlreadyOwned
		);

		// 当没有上架时，失败
		assert_noop!(
			KittiesModule::buy(RuntimeOrigin::signed(ACCOUNT_ID2), KITTY_ID),
			Error::<Test>::NotOnSale
		);

		// 上述失败条件不存在时，成功
		assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(ACCOUNT_ID), KITTY_ID));
		assert_ok!(KittiesModule::buy(RuntimeOrigin::signed(ACCOUNT_ID2), KITTY_ID));
		assert!(KittiesModule::kitty_on_sale(KITTY_ID).is_none());
		assert_eq!(KittiesModule::kitty_owner(KITTY_ID), Some(ACCOUNT_ID2));
		assert_eq!(Balances::free_balance(ACCOUNT_ID), ACCOUNT_BALANCE);
		assert_eq!(Balances::free_balance(ACCOUNT_ID2), ACCOUNT_BALANCE2 - EXISTENTIAL_DEPOSIT * 10);
		System::assert_last_event(Event::KittyBought { who: ACCOUNT_ID2, kitty_id: 0 }.into());
	});
}
