use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_created() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

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

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id_1, kitty_id_1),
			Error::<Test>::SamedKittyId
		);
		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id_1, kitty_id_2),
			Error::<Test>::InvalidKittyId
		);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id_2)));

		assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id_1, kitty_id_2));
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
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::transfer(
			RuntimeOrigin::signed(account_id),
			account_id_2,
			kitty_id,
		));
		System::assert_last_event(
			Event::KittyTransferred { who: account_id, recipient: account_id_2, kitty_id }.into(),
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