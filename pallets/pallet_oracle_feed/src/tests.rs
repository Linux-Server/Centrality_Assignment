use crate::{mock::*};
use frame_benchmarking::frame_support::BoundedVec;
use frame_support::{assert_ok};
// use bounded_vec::BoundedVec;
// use std::convert::TryInto;
#[test]
fn create_oracle_event_should_work() {
    new_test_ext().execute_with(|| {
		//Anyone can submit a request to root waiting list
        let data: BoundedVec<_, KeyLimit> = BoundedVec::try_from(vec!["sam".as_ptr() as u8]).unwrap();
        let data1: BoundedVec<_, KeyLimit> = BoundedVec::try_from(vec!["sam".as_ptr() as u8]).unwrap();        
        assert_ok!(ClubPalletTesting::create_oracle_event(Origin::root(), data, data1));

		// Submitted trnx should return a Ok
		//  assert_eq!(ClubPalletTesting::request_to_join_club(Origin::signed(2)),  Ok(()));
    })
}



// // #[test]
// // fn remove_member_from_club(){
// //     new_test_ext().execute_with(|| {
// // 		// Not accessible by a normal user
// // 		assert_noop!(ClubPalletTesting::remove_member_from_club(Origin::signed(1),100), BadOrigin);
	
// //     })
// // }

// // #[test]
// // fn add_requested_member_to_club() {
// //     new_test_ext().execute_with(|| {

// // 		// trying to add a value , which is not in the root request list
// //         assert_err!(
// //             ClubPalletTesting::add_requested_member_to_club(Origin::root(), 51), 
// // 			Error::<Test>::NotMember
// //         );
// //     })
// // }