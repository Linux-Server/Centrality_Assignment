use crate::{mock::*};
use frame_benchmarking::frame_support::BoundedVec;
use frame_support::{assert_ok, assert_noop, error::BadOrigin};
// use bounded_vec::BoundedVec;
// use std::convert::TryInto;
// use frame_benchmarking::Zero;
#[test]
fn create_oracle_event_should_work() {
    new_test_ext().execute_with(|| {
    
        let event_name: BoundedVec<_, KeyLimit> = BoundedVec::try_from(vec!["Oracle Event".as_ptr() as u8]).unwrap();
        let event_description: BoundedVec<_, KeyLimit> = BoundedVec::try_from(vec!["Its a test event for now".as_ptr() as u8]).unwrap();        
        assert_ok!(ClubPalletTesting::create_oracle_event(Origin::root(), event_name.clone(), event_description.clone()));
		    assert_eq!(ClubPalletTesting::create_oracle_event(Origin::root(), event_name, event_description),  Ok(()));
    })
}


#[test]
fn create_oracle_event_should_not_work() {
    new_test_ext().execute_with(|| {
        let event_name: BoundedVec<_, KeyLimit> = BoundedVec::try_from(vec!["sam".as_ptr() as u8]).unwrap();
        let event_description: BoundedVec<_, KeyLimit> = BoundedVec::try_from(vec!["sam".as_ptr() as u8]).unwrap();        

        assert_noop!(
          ClubPalletTesting::create_oracle_event(Origin::signed(2), event_name.clone(), event_description.clone()),
          BadOrigin
        );

		    assert_ne!(ClubPalletTesting::create_oracle_event(Origin::signed(1), event_name, event_description),  Ok(()));
        
    })
}


// #[test]
// fn valid_weights() {
//     use frame_support::pallet_prelude::*;
//     use frame_support::weights::RuntimeDbWeight;
//     use scale_info::prelude::num::Wrapping;
//     new_test_ext().execute_with(|| {
//       let db_weights: RuntimeDbWeight = <Test as frame_system::Config>::DbWeight::get();
     

//         assert_eq!(
//             ClubPalletTesting::on_initialize(1),
//             db_weights.writes(3));
//     })
// }



