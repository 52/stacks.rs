// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#![allow(unused_macros, unused_imports, dead_code)]

/// Generates a test for a token-transfer transaction.
#[macro_export]
macro_rules! generate_token_transfer_test {
    (
        Standard,
        $name:ident,
        $recipient:expr,
        $amount:expr,
        $fee:expr,
        $nonce:expr,
        $network:expr,
        $anchor_mode:expr,
        $memo:expr,
        $post_condition_mode:expr,
        $post_conditions:expr,
        $sponsored:expr,
        $expected_tx_hex:expr,
        $expected_tx_id_hex:expr
    ) => {
        #[test]
        fn $name() {
            let tx = STXTokenTransfer::builder()
                .recipient(clarity!(PrincipalStandard, $recipient))
                .network($network)
                .sender(private_key())
                .amount($amount)
                .fee($fee)
                .nonce($nonce)
                .anchor_mode($anchor_mode)
                .memo($memo)
                .post_condition_mode($post_condition_mode)
                .post_conditions($post_conditions)
                .sponsored($sponsored)
                .build()
                .transaction()
                .sign(private_key())
                .expect("Failed to sign the transaction");

            let encoded = tx.encode().expect("Failed to encode the transaction");

            let hash = tx.hash().expect("Failed to hash the transaction");
            let tx_id = hash.as_bytes();

            let tx_hex = bytes_to_hex(&encoded);
            let tx_id_hex = bytes_to_hex(&tx_id);

            assert_eq!(
                tx_hex, $expected_tx_hex,
                "Encoded transaction hex does not match expected."
            );

            assert_eq!(
                tx_id_hex, $expected_tx_id_hex,
                "Transaction ID hex does not match expected."
            );
        }
    };
}

/// Generates a test for a contract-call transaction.
#[macro_export]
macro_rules! generate_contract_call_test {
    (
        Standard,
        $name:ident,
        $function_args:expr,
        $fee:expr,
        $nonce:expr,
        $network:expr,
        $anchor_mode:expr,
        $post_condition_mode:expr,
        $post_conditions:expr,
        $sponsored:expr,
        $expected_tx_hex:expr,
        $expected_tx_id_hex:expr
    ) => {
        #[test]
        fn $name() {
            let tx = STXContractCall::builder()
                .contract(contract())
                .fn_name("function-name")
                .fn_args($function_args)
                .sender(private_key())
                .network($network)
                .fee($fee)
                .nonce($nonce)
                .anchor_mode($anchor_mode)
                .post_condition_mode($post_condition_mode)
                .post_conditions($post_conditions)
                .sponsored($sponsored)
                .build()
                .transaction()
                .sign(private_key())
                .expect("Failed to sign the transaction");

            let encoded = tx.encode().expect("Failed to encode the transaction");

            let hash = tx.hash().expect("Failed to hash the transaction");
            let tx_id = hash.as_bytes();

            let tx_hex = bytes_to_hex(&encoded);
            let tx_id_hex = bytes_to_hex(&tx_id);

            assert_eq!(
                tx_hex, $expected_tx_hex,
                "Encoded transaction hex does not match expected."
            );

            assert_eq!(
                tx_id_hex, $expected_tx_id_hex,
                "Transaction ID hex does not match expected."
            );
        }
    };
}
