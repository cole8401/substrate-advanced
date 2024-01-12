use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, traits::Currency};

#[test]
fn it_works_for_created() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let _ = Balances::deposit_creating(&account_id, 1000);

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		System::assert_last_event(
			Event::KittyCreated {
				who: account_id,
				kitty_id,
				kitty: KittiesModule::kitties(kitty_id).unwrap(),
			}
			.into(),
		);

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert!(KittiesModule::kitties(kitty_id).is_some());
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		crate::NextKittyId::<Test>::set(crate::KittyId::MAX);
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id)),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		let kitty_id_1 = 0;
		let kitty_id_2 = 1;
		let account_id = 1;
		let account_id_2 = 2;
		let kitty_id_3 = 2;
		let _ = Balances::deposit_creating(&account_id, 1000);
		let _ = Balances::deposit_creating(&account_id_2, 1000);

		assert_noop!(
			KittiesModule::breed(
				RuntimeOrigin::signed(account_id),
				kitty_id_1,
				kitty_id_1,
				*b"abceabcd"
			),
			Error::<Test>::SameKittyId
		);
		assert_noop!(
			KittiesModule::breed(
				RuntimeOrigin::signed(account_id),
				kitty_id_1,
				kitty_id_2,
				*b"abceabcd"
			),
			Error::<Test>::InvalidKittyId
		);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id_2)));

		assert_ok!(KittiesModule::breed(
			RuntimeOrigin::signed(account_id),
			kitty_id_1,
			kitty_id_2,
			*b"abceabcd"
		));
		System::assert_last_event(
			Event::KittyBred {
				who: account_id,
				kitty_id: kitty_id_3,
				kitty: KittiesModule::kitties(kitty_id_3).unwrap(),
			}
			.into(),
		);
		assert!(KittiesModule::kitties(kitty_id_3).is_some());
		assert_eq!(KittiesModule::kitty_owner(kitty_id_3), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id_3), Some((kitty_id_1, kitty_id_2)));
	});
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let account_id_2 = 2;
		let _ = Balances::deposit_creating(&account_id, 1000);
		let _ = Balances::deposit_creating(&account_id_2, 1000);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::transfer(
			RuntimeOrigin::signed(account_id),
			account_id_2,
			kitty_id,
		));
		System::assert_last_event(
			Event::KittyTransferred { from: account_id, recipient: account_id_2, kitty_id }.into(),
		);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id_2));
		assert_ok!(KittiesModule::transfer(
			RuntimeOrigin::signed(account_id_2),
			account_id,
			kitty_id,
		));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
	});
}

#[test]
fn it_works_for_sale() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let _ = Balances::deposit_creating(&account_id, 1000);
		let origin = RuntimeOrigin::signed(account_id);
		let origin2 = RuntimeOrigin::signed(2);
		assert_noop!(KittiesModule::sale(origin.clone(), kitty_id), Error::<Test>::InvalidKittyId);
		assert_ok!(KittiesModule::create(origin.clone()));
		assert_noop!(KittiesModule::sale(origin2, kitty_id), Error::<Test>::NotOwner);
		assert_ok!(KittiesModule::sale(origin.clone(), kitty_id));
		assert_noop!(KittiesModule::sale(origin.clone(), kitty_id), Error::<Test>::AlreadyOnSale);
	})
}

#[test]
fn it_works_for_buy() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let _ = Balances::deposit_creating(&1, 1000);
		let _ = Balances::deposit_creating(&2, 1000);
		let origin = RuntimeOrigin::signed(1);
		let origin2 = RuntimeOrigin::signed(2);
		assert_noop!(KittiesModule::buy(origin.clone(), kitty_id), Error::<Test>::InvalidKittyId);
		assert_ok!(KittiesModule::create(origin.clone()));
		assert_noop!(KittiesModule::buy(origin2.clone(), kitty_id), Error::<Test>::NotOnSale);
		assert_ok!(KittiesModule::sale(origin.clone(), kitty_id));
		assert_noop!(KittiesModule::buy(origin.clone(), kitty_id), Error::<Test>::AlreadyOnSale);
		assert_ok!(KittiesModule::buy(origin2, kitty_id));
	})
}
