//code reference: https://github.com/manuelmauro/algonaut/blob/main/tests/step_defs/integration/abi.rs
/*
TO DO:
(1)Implement To and from variants for Suggested Transaction Parameters  for dictionaries from Algodot core
(2) Rename Structs to be name appropriate
(3) Imlement Box Storage
(4) Impl serder json fron algonaut
*/
pub mod abi_smartcontract {

    /*
    Custom Logic For My Escrow Smart COntract using abi methods
    Temporarily disabled for refactoring

    */

    use algonaut::abi::abi_interactions::AbiMethod;
    //use algonaut::abi::abi_type::AbiType;
    use gdnative::core_types::Variant;
    use gdnative::prelude::ToVariant;

    pub struct Foo {
        pub name: String,
        pub description: String,
        pub type_: String,
        pub parsed: Option<String>,
    }

    impl ToVariant for Foo {
        fn to_variant(&self) -> Variant {
            todo!()
        }
    }
    // rename from foo
    impl Foo {
        //Doc : https://developer.algorand.org/docs/get-details/transactions/signatures/#single-signatures
        //      https://developer.algorand.org/docs/get-details/dapps/smart-contracts/ABI/?from_query=Method%20Signature#reference-types

        pub fn withdraw() -> AbiMethod {
            let method_sig: String = "withdraw(uint64,account)void".to_string();
            //let method_sig : String = "add(uint64,uint64)uint128".to_string();

            println!("Method Signature: {}", &method_sig);

            AbiMethod::from_signature(&method_sig).expect("Error")
        }

        pub fn deposit() -> AbiMethod {
            let method_sig: String = "deposit(PaymentTransaction,account)void".to_string();
            //let method_sig : String = "add(uint64,uint64)uint128".to_string();

            println!("Method Signature: {}", &method_sig);

            AbiMethod::from_signature(&method_sig).expect("Error")
        }
    }
}

pub mod atc {
    /*
    Atomic Transaction Composer Required Traits
    */
    use std::string::String as str;

    pub enum AtomicTransactionComposerStatus {
        Building,
        Built,
        Signed,
        Submitted,
        Committed,
    }
    // rewrite to parse to godot debugger
    impl std::fmt::Display for AtomicTransactionComposerStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                AtomicTransactionComposerStatus::Building => write!(f, "Building"),
                AtomicTransactionComposerStatus::Built => write!(f, "Built"),
                AtomicTransactionComposerStatus::Signed => write!(f, "Signed"),
                AtomicTransactionComposerStatus::Submitted => write!(f, "Submitted"),
                AtomicTransactionComposerStatus::Committed => write!(f, "Committed"),
            }
        }
    }

    #[allow(dead_code)]
    pub trait Into {
        type Into;
        type From;
        type T;

        fn into<T: From<Self::T>>(_b: &T) {
            todo!()
        }

        fn into_boxed_str() -> &'static str;
    }

    // Implement the From trait for AtomicTransactionComposerStatus to &'static str
    impl<'a> From<AtomicTransactionComposerStatus> for &'a str {
        fn from(s: AtomicTransactionComposerStatus) -> &'a str {
            Box::leak(Box::new(s.to_string()))
        }
    }

    // Implement the From trait for &mut AtomicTransactionComposerStatus to &str
    impl<'a> From<&'a mut AtomicTransactionComposerStatus> for &'a str {
        fn from(_: &'a mut AtomicTransactionComposerStatus) -> &'a str {
            todo!()
        }
    }
}

pub mod params {

    /*
        Temporary Fix for Params Error Between algodot_core and algonaut suggested
        transaction parameters

        TO DO :
        (1) Depreciate this codebase and implement godot dictionary trait from algodot code my suggested transactions impl

    */

    //use algonaut::core::MicroAlgos;
    //use algonaut::core::Round;
    use algonaut::core::SuggestedTransactionParams;
    use algonaut_algod::models::TransactionParams200Response;
    //use algonaut_model::transaction::ApiTransaction;

    pub struct MySuggestedTransactionParams(());

    pub trait Into {
        fn _app_id(&self, x: u64) -> u64;
        fn default(&self) -> Option<String>
        where
            Self: Sized,
        {
            None
        }

        fn to_variant(&self, params: SuggestedTransactionParams) -> TransactionParams200Response;
    }
}

pub mod escrow {

    use algonaut::abi::abi_type::AbiValue::Int;
    use algonaut::core::Address;
    use algonaut::core::{to_app_address, Address as OtherAddress, MicroAlgos};
    use algonaut::{
        atomic_transaction_composer::{AbiArgValue, AtomicTransactionComposer},
        //error::Error,
    };

    //use algonaut_algod::models::TransactionParams200Response;
    use algonaut_transaction::{
        builder::Pay, builder::TransactionParams, builder::TxnBuilder, Transaction,
    };

    use algonaut::core::SuggestedTransactionParams as OtherSuggestedTransactionParams;
    use algonaut_transaction::account::Account; //, transaction::Payment

    use std::convert::TryInto;
    use std::str::FromStr;

    use algonaut::atomic_transaction_composer::transaction_signer::TransactionSigner::BasicAccount;
    use algonaut::atomic_transaction_composer::{AbiMethodResult, ExecuteResult};
    use algonaut_crypto::HashDigest;
    //use algonaut::atomic_transaction_composer::ExecuteResult;
    use gdnative::core_types::Dictionary;
    use gdnative::core_types::Variant;
    use gdnative::prelude::OwnedToVariant;

    #[derive(Debug, Clone)]
    pub struct Foo<'a> {
        // Escrow DaPP Struct
        pub withdrw_amt: u32,
        pub withdrw_to_addr: [u8; 32],
        pub arg1: AbiArgValue,
        pub arg2: AbiArgValue,
        pub _app_id: u64,
        pub _escrow_address: Address,
        pub atc: &'a AtomicTransactionComposer,
    }

    pub struct MyExecuteResult {
        pub confirmed_round: Option<u64>,
        pub tx_ids: Vec<String>,
        pub method_results: Vec<AbiMethodResult>,
    }

    #[allow(dead_code)]
    /* Escrow Smart Contract Mod Traits */
    trait MyTrait {
        type Foo<'a>;
        type Params;
        type Parsed;
        type Payment;

        type Error;

        fn _app_id(&self, x: u64) -> u64;
        fn arg1(withdrw_amt: u64) -> AbiArgValue {
            AbiArgValue::AbiValue(Int(withdrw_amt.into()))
        }
        fn arg2(withdrw_amt: u64) -> AbiArgValue {
            AbiArgValue::AbiValue(Int(withdrw_amt.into()))
        }
    }

    // Required Trait for Suggested Transation Params

    /*Godot ENgine Traits */
    pub trait ToVariant {
        fn to_variant(&self) -> Variant;
    }

    impl ToVariant for ExecuteResult {
        fn to_variant(&self) -> Variant {
            let dict = Dictionary::new();
            dict.insert("confirmed_round", Some(self.confirmed_round));
            dict.insert("tx_ids", self.tx_ids.clone());
            //dict.insert("method_results", self.method_results);
            dict.owned_to_variant()
        }
    }

    /*Algonaut Traits */
    // Create a wrapper around the foreign type using Ne Type method trait impl
    pub struct MyTransactionParams {
        pub params: OtherSuggestedTransactionParams, // Existing field
        pub genesis_id: String,
        pub hash: HashDigest, // Add a HashDigest field
        pub last_round: u64,
        pub min_fee: u64,
    }

    // Implement the trait for your wrapper
    impl TransactionParams for MyTransactionParams {
        // placeholder code to stop compiler errors for now
        fn last_round(&self) -> u64 {
            self.last_round
        }

        fn min_fee(&self) -> u64 {
            self.min_fee
        }

        fn genesis_hash(&self) -> HashDigest {
            self.hash // Return the HashDigest stored in the struct
        }
        fn genesis_id(&self) -> &String {
            &self.genesis_id // Return a reference to the `genesis_id` field
        }
    }
    //impl TransactionParams200Response for MyTransactionParams {}
    /*Escrow Smart Contract Arc 4 Implementation*/

    impl Foo<'_> {
        pub fn note(size: u32) -> Option<Vec<u8>> {
            Some(vec![size.try_into().unwrap()])
        }

        pub fn withdraw_amount(amount: u32) -> AbiArgValue {
            /*
            Converts a U64 int to Big Uint and returns an AbiArg Value

            //code reference: https://github.com/manuelmauro/algonaut/blob/main/tests/step_defs/integration/abi.rs

            */
            //let withdrw_amt : num_bigint::BigUint = BigUint::new(vec![amount]); //in MicroAlgos

            let arg1: AbiArgValue = AbiArgValue::AbiValue(Int(amount.into()));
            arg1
        }

        //use algonaut_core::Address;
        pub fn pay(
            to_address: algonaut::core::Address,
            acct1: Account,
            _params: MyTransactionParams, // My Own Params That Satisfy all traits //algonaut::core::SuggestedTransactionParams,
        ) -> Transaction {
            /*
                Constructs a Payment Transaction to an Address
            */

            TxnBuilder::with(
                &_params,
                Pay::new(acct1.address(), to_address, MicroAlgos(123_456)).build(),
            )
            .build()
            .unwrap()
        }

        pub fn app_address(app_id: &u64) -> Address {
            to_app_address(*app_id)
        }
        pub fn new_atc() -> AtomicTransactionComposer {
            /*
            Constructs a Default Atomic Transation Composer
            */
            AtomicTransactionComposer::default()
        }

        pub fn address_to_address(s: &str) -> OtherAddress {
            /*
            Constructs a 32 Bit Byte Slice froma Given Address String
            */
            OtherAddress::from_str(s).unwrap()

            //let mut _to_addr: [u8; 32] = [0; 32];
            //_to_addr.copy_from_slice(&addr.as_bytes()[..32]);
        }

        pub fn address_to_bytes(addr: String) -> [u8; 32] {
            /*
            Constructs a 32 Bit Byte Slice froma Given Address String
            */

            let mut _to_addr: [u8; 32] = [0; 32];
            _to_addr.copy_from_slice(&addr.as_bytes()[..32]);
            _to_addr
        }

        //let arg2: AbiArgValue = AbiArgValue::AbiValue(algonaut_abi::abi_type::AbiValue::Address(OtherAddress::new(withdrw_to_addr)));

        pub fn address(addr: OtherAddress) -> AbiArgValue {
            /* Returns an Address abi value from an Address as [u8,32]*/
            AbiArgValue::AbiValue(algonaut::abi::abi_type::AbiValue::Address(addr))
        }

        pub fn basic_account(
            mnemonic: &str,
        ) -> algonaut::atomic_transaction_composer::transaction_signer::TransactionSigner {
            BasicAccount(algonaut::transaction::account::Account::from_mnemonic(mnemonic).unwrap())
        }

        //pub fn fee(amount: u64) -> TxnFee {
        //    Fixed(MicroAlgos(amount))
        //}
    }
}
