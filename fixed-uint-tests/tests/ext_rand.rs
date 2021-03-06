// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! check_rand {
    ($name:ident, $uint:ident) => {
        #[test]
        fn $name() {
            let x = nfuint::$uint::thread_random();
            let y = nfuint::$uint::thread_random();
            // If this test is failed, please check if there is a bug or you are too luckly.
            assert!(!x.is_zero());
            assert!(!y.is_zero());
            assert!(x != y);
        }
    };
}

check_rand!(rand_u128, U128);
check_rand!(rand_u160, U160);
check_rand!(rand_u224, U224);
check_rand!(rand_u256, U256);
check_rand!(rand_u384, U384);
check_rand!(rand_u512, U512);
check_rand!(rand_u520, U520);
check_rand!(rand_u1024, U1024);
check_rand!(rand_u2048, U2048);
check_rand!(rand_u4096, U4096);
