pub mod bar {
    //use algonaut_abi::abi_interactions::AbiMethodArg;
    //use algonaut_abi::abi_interactions::AbiReturn;
    use algonaut::abi::abi_type::AbiType;


    use algonaut::abi::abi_interactions::AbiMethod;
    use algonaut::atomic_transaction_composer::AtomicTransactionComposer;
    use gdnative::core_types::{Variant, ToVariant};
    
    pub struct Foo {
        pub name: String,
        pub description: String,
        pub type_: String, 
        pub parsed: Option<String>,
        
    }

    impl MyTrait for Foo {
        type Foo = Foo;
        type Type = String;
        type Parsed = Option<String>;

        fn new() -> Self::Foo {
            Foo {
                name: "".to_string(),
                description: "".to_string(),
                type_: "".to_string(),
                parsed: None,
            }
        }

        fn r#type() -> String { "".to_string() }
        fn parsed() -> Option<AbiType> { None }
    }

    trait MyTrait {
        type Foo;
        type Type: ToString;
        type Parsed;

        fn new() -> Self::Foo;
        fn r#type() -> String;
        fn parsed() -> Option<AbiType>;
    }



    impl Foo {
        //Doc : https://developer.algorand.org/docs/get-details/transactions/signatures/#single-signatures
        //      https://developer.algorand.org/docs/get-details/dapps/smart-contracts/ABI/?from_query=Method%20Signature#reference-types
        // Boilerplate
        //pub fn new() -> AbiMethod {
        //    let method_sig : String = "withdraw(uint64,account)void".to_string();
            //let method_sig : String = "add(uint64,uint64)uint128".to_string();

            
        //    println!("{}",&method_sig);

        //    AbiMethod::from_signature(&method_sig)
        //    .expect("Error")
            
        //}
        
        pub fn withdraw() -> AbiMethod {
            let method_sig : String = "withdraw(uint64,account)void".to_string();
            //let method_sig : String = "add(uint64,uint64)uint128".to_string();

            
            println!("Method Signature: {}",&method_sig);

            AbiMethod::from_signature(&method_sig)
            .expect("Error")
            
        }

        pub fn deposit() -> AbiMethod {
            let method_sig : String = "deposit(PaymentTransaction,account)void".to_string();
            //let method_sig : String = "add(uint64,uint64)uint128".to_string();

            
            println!("Method Signature: {}",&method_sig);

            AbiMethod::from_signature(&method_sig)
            .expect("Error")
            
        }
   
    }
}

#[macro_export]
mod escrow {

    /*Atomic Transaction Composer Helper Modules*/
    use algonaut::atomic_transaction_composer::transaction_signer::TransactionSigner::BasicAccount;
    //use crate::core::Account;
    use algonaut::transaction::account::Account;
    //use crate::algod::Account;
    
    //use algonaut::algod::v2::Algod;
    use algonaut::abi::abi_type::AbiValue::Int;
    use algonaut::core::Address;
    
    //use num_bigint::BigUint;
   // use crate::algod::bar::Foo;
    use algonaut::{
        
        atomic_transaction_composer::{
            transaction_signer::TransactionSigner, AbiArgValue, 
            AtomicTransactionComposer, //AbiReturnDecodeError, AddMethodCallParams, 
            TransactionWithSigner, //AtomicTransactionComposerStatus, 
        },
        error::ServiceError,
    };
    
    use algonaut::core::{to_app_address, Address as OtherAddress, MicroAlgos, CompiledTeal};
    //use algonaut::abi::abi_interactions::AbiMethod;
    use algonaut::transaction::{
        builder::TxnFee, builder::TxnFee::Fixed,
        transaction::{ApplicationCallOnComplete, StateSchema},
        Pay, TxnBuilder,
    };

    use algonaut::core::SuggestedTransactionParams as OtherSuggestedTransactionParams;
    use algonaut::transaction::{transaction::Payment}; //account::Account
  
    //use algonaut::crypto::HashDigest;
    use std::convert::TryInto;   
    //use gdnative::prelude::*;

   // use std::marker::Sized;
    use std::fmt::Display as Display;
    use algonaut::atomic_transaction_composer::AtomicTransactionComposerStatus as OtherAtomicTransactionComposerStatus;

    use crate::algod::bar::Foo as OtherFoo;
    //#[derive(Clone, Debug, escrow::ToVariant::to_variant(&atc))] //PartialEq,
    
    //#[derive(Clone, Debug, escrow::MyTrait::to_variant(&atc))] //PartialEq,
            
    //#[derive(Clone<'_>, Debug<'_>, gdnative::prelude::ToVariant::to_variant(atc))] //PartialEq,
    
    //#[derive(Clone, Debug, escrow::OwnedToVariant::to_variant(&atc))] //PartialEq,

    #[derive(gdnative::prelude::ToVariant::to_variant(&atc), Debug)] //PartialEq,
    //#[derive (gdnative::prelude::ToVariant::to_variant(Foo))]
     

    //lifetime Parameter
    pub struct Foo <'a> {
        pub withdrw_amt: u32,
        pub withdrw_to_addr: [u8; 32],
        pub arg1: AbiArgValue,
        pub arg2: AbiArgValue,
        pub _app_id: u64,
        pub _escrow_address: Address,
        pub atc: &'a AtomicTransactionComposer,
        
    }
    

    //All lifetime traits
    pub trait MyTrait<'a> {
        type Foo ;
        type Params;
        type Parsed;
        type Payment;
        type AtomicTransactionComposer ;
        //type ToVariant;
        type Sized ;//: u64; //= 7u64;
        type ToVariant = dyn ToVariant<Sized = usize>;
        type OwnedToVariant;
        //type NewTrait;
        

        fn _app_id(&self, x: u64) -> u64;
        //fn default() -> Option<String>{ None }
        //fn suggested_tx_params(&self) -> OtherSuggestedTransactionParams { OtherSuggestedTransactionParams::default() }
        fn arg1(withdrw_amt: u64) -> AbiArgValue { AbiArgValue::AbiValue(Int(withdrw_amt.into())) }
        fn arg2(withdrw_amt: u64) -> AbiArgValue { AbiArgValue::AbiValue(Int(withdrw_amt.into())) }

        fn get(&self) -> &'a AtomicTransactionComposer { todo!()}
        
        //fn to_variant(&self) -> dyn NewTrait <Sized = u64>{
        //    (**self).clone().into_shared().to_variant()
        //}
        fn to_variant(&'a self) -> &'a AtomicTransactionComposer {&AtomicTransactionComposer::default()}
  
    }

    pub trait ATC {
        type AtomicTransactionComposer  ;
        type AtomicTransactionComposerStatus = dyn AtomicTransactionComposerStatus;     
    }

    impl Display for dyn ATC<AtomicTransactionComposer = AtomicTransactionComposer, AtomicTransactionComposerStatus = dyn AtomicTransactionComposerStatus>{
        fn to_string(&self) -> String;

    }
    //code duplicate
    //impl Display for dyn ATC<AtomicTransactionComposerStatus = dyn AtomicTransactionComposerStatus>{
    //    fn to_string(&self) -> String;

    //}
    //trait NewTrait: ToVariant + Sized  {
       
    // fn static_foo<T:NewTrait + ?Sized>(b: &T) {todo!()}
        
    //}

    //Docs: https://godot-rust.github.io/docs/gdnative/prelude/struct.Variant.html

    //trait NewTrait: escrow::ToVariant + Sized {}
    pub trait OwnedToVariant{
        type Sized ;
        
        fn to_variant(&self) -> &AtomicTransactionComposer ;//{ todo!()}
    }

    pub trait ToVariant{
        type Sized ;
        
        fn to_variant(&self) -> &AtomicTransactionComposer ;//{ todo!()}
    }

    pub trait AtomicTransactionComposerStatus{
        fn status(&self) -> dyn AtomicTransactionComposerStatus ;
        fn to_string(&self) -> String;
    }
    /*Implements all traits for Foo Crate*/
    impl <'a, 'c> MyTrait <'a> for OtherFoo{//Foo <'a>{
        type Foo  = OtherFoo;
        type Parsed = Option<String>;
        type Payment = Option<Payment>;
        type Params = Option<OtherSuggestedTransactionParams>;
        type AtomicTransactionComposer =  AtomicTransactionComposer;
        
        type Sized = u32;//: u64; //= 7u64;
        type ToVariant = dyn ToVariant<Sized = usize>;
        type OwnedToVariant = dyn ToVariant<Sized = u32>; // = dyn OwnedToVariant<Sized = usize>;
        fn _app_id(&self, x: u64) -> u64 { x }
        
    }

    //impl Sized for  dyn ToString{
    //
    //
    // }

    impl AtomicTransactionComposerStatus for dyn ToString { 
        fn status(&self ) -> dyn ToString {
            <dyn AtomicTransactionComposerStatus>::to_string(&dyn escrow::AtomicTransactionComposerStatus)
            //"dfadfsdf".to_string()
        }
        
    }
    impl OwnedToVariant for AtomicTransactionComposer {
        type Sized = i32;
        
        fn to_variant(&self) -> &AtomicTransactionComposer {AtomicTransactionComposer::status(&AtomicTransactionComposer).to_string()}
    

    }
    impl ToVariant for AtomicTransactionComposer {
        type Sized = i32;
        
        fn to_variant(&self) -> &AtomicTransactionComposer { AtomicTransactionComposer::status(&AtomicTransactionComposer).to_string()}
    
      
    }

    impl ToVariant for &&AtomicTransactionComposer {
        type Sized = i32;
        
        fn to_variant(&self) -> &AtomicTransactionComposer { &AtomicTransactionComposer::default().status().to_string()}
    
      
    }
    impl<'a> ToVariant for &AtomicTransactionComposer {
        type Sized = i32;
        
        fn to_variant(&self) -> &AtomicTransactionComposer { todo!()}
    
      
    }
      
        impl <'a> MyTrait <'_> for &'a AtomicTransactionComposer {
            type Foo = OtherFoo;//Self::Foo;//Foo<'a>;
            type Parsed = Option<String>;
            type Payment= Option<Payment>;
            type Params = Option<OtherSuggestedTransactionParams>;
            type AtomicTransactionComposer = AtomicTransactionComposer;
            
            type Sized = u32;
            type ToVariant= dyn ToVariant<Sized = u32>;
            type OwnedToVariant = dyn ToVariant <Sized = u32>;
            
            //type NewTrait = dyn NewTrait<Sized = i32>;
         fn _app_id(&self, x: u64) -> u64{todo!()}
   
            
            //type ToVariant = dyn ToVariant<Sized = ?Sized>;//T: ?Sized
            //type Sized;
         //fn to_variant(&self) -> &'a AtomicTransactionComposer {todo!()}
  
         //fn to_variant(&self) -> dyn NewTrait {
         //   (**self).clone().into_shared().to_variant()
         //}

         fn to_variant(&self) -> &AtomicTransactionComposer {
            //(**self).len()
            //todo!()\
            &AtomicTransactionComposer::default()
         }
        }


    impl OtherFoo{
        // Adding method to create application call
        fn get_call(&self) -> Result<ApplicationCallOnComplete, ServiceError> {
            //let func_args = vec![self.arg1.clone(), self.arg2.clone()];
            
            todo!()
            
        }

        // Adding method to create pay transaction
        fn get_payment(&self) -> Result<Payment, ServiceError> {
            todo!()
           // tx
        }

        fn arg1(&self)-> AbiArgValue{ 
            todo!()
            
        }
        
        pub fn note(size : u32) -> Option <Vec<u8>>{
            Some(vec![size.try_into().unwrap()])

        }
    


        pub fn withdraw_amount(amount : u32) -> AbiArgValue {
            /*
            Converts a U64 int to Big Uint and returns an AbiArg Value
            
            Temporarily Disabling
            */
 

            todo!()

        }
            
    
        
        //pub fn withdraw(_acct1: Account ){
             /* 
            Withdraw Method Parameters for Escrow SmartContract
            
                Docs: https://docs.rs/num-bigint/0.4.3/num_bigint/struct.BigUint.html

                Does nothing
            */

        //}
        

        //use algonaut_core::Address;
        pub fn pay(to_address : algonaut::core::Address , acct1 : Account, _params : algonaut::core::SuggestedTransactionParams) -> algonaut::transaction::transaction::Transaction{
            /*
                Constructs a Payment Transaction to an Address
            */

             let _t = TxnBuilder::with(

                    &_params,

                    Pay::new(acct1.address(), to_address, MicroAlgos(123_456)).build(),

                )

                .build()
                .unwrap();
            
            return _t;
        }

        pub fn app_address (app_id : &u64) -> Address{
            to_app_address(*app_id)
        }
        
        //pub fn deposit(_algod : Algod , acct1_3 : Account ,  params : algonaut::core::SuggestedTransactionParams) -> algonaut::core::SuggestedTransactionParams {
            /*
            Deposit Method Parameters for Escrow SmartContract
            Unused and Depreciated
           
            Does
            */

        //    let _app_id = 161737986;

            
            //Get Escrow Address From App ID

        //    let _escrow_address = escrow::Foo::app_address(&_app_id); //to_app_address(_app_id.clone());
           
        //    println!(" building Pay transaction to Escrow Address: {}", &_escrow_address);

            //let _t = Foo::pay(_escrow_address, acct1_3.clone().into(), params.clone());                

            // create a transaction with signer with the current transaction

        //    let _signer ;//= TransactionSigner::BasicAccount(acct1_3);


            //let tx_with_signer = TransactionWithSigner { tx: _t, signer: _signer };


        //    let mut atc = AtomicTransactionComposer::default();  

            // Deposit
            // Add Payment Txn to 
            // Should Ideally Match To A Statemachine Behaviour Bloc
            
            //atc.add_transaction(tx_with_signer).unwrap();

        //  params

 
        //}

        pub fn new() -> AtomicTransactionComposer{
        /*
        Constructs a Default Atomic Transation Composer
        */
            AtomicTransactionComposer::default()
        
        }
     
        pub fn address_to_bytes(addr: String) -> [u8; 32]{ 
        /*
        Constructs a 32 Bit Byte Slice froma Given Address String
        */   
            let mut _to_addr: [u8; 32] = [0; 32];
            _to_addr.copy_from_slice(&addr.as_bytes()[..32]);

            _to_addr
            
        }

        //let arg2: AbiArgValue = AbiArgValue::AbiValue(algonaut_abi::abi_type::AbiValue::Address(OtherAddress::new(withdrw_to_addr)));
      
        pub fn address(addr : [u8; 32]) -> AbiArgValue {
            /* Returns an Address abi value from an Address as [u8,32]*/
            AbiArgValue::AbiValue(algonaut::abi::abi_type::AbiValue::Address(OtherAddress::new(addr)))

        } 

        
        pub fn basic_account(mnemonic : &str)  ->  algonaut::atomic_transaction_composer::transaction_signer::TransactionSigner{
            BasicAccount(Account::from_mnemonic(&mnemonic).unwrap())
        
        }

        pub fn fee(amount : u64) -> TxnFee{Fixed(MicroAlgos(amount))}

        //pub fn construct_app_call_method(
        /*
        Constructs an App Call Method as a Rust Module
        
        Depreciated
        */
        
 
        //&self,
        //_app_id: u64,
        //_method: AbiMethod,
        //_method_args: Vec<AbiArgValue>,
        //_fee: TxnFee,//Fixed(MicroAlgos(2500u64)), //make customizable
        //_sender: Address,
        //_on_complete: ApplicationCallOnComplete,
        //_clear_program: Option<CompiledTeal>,
        //_global_schema: Option<StateSchema>,
        //_local_schema: Option<StateSchema>,
        //_extra_pages: u32,
        //_note: Option<Vec<u8>>,
        //_lease: Option<HashDigest>,
        //_rekey_to: Option<Address>,
        //_signer: TransactionSigner,
      //  )
    
        // -> Result<Foo<'_>, ServiceError> {todo!()}
        

    } 

}







use algodot_core::*;
use algodot_macros::*;
use algonaut::algod::v2::Algod;
use algonaut::core::{MicroAlgos, Round};
use algonaut::model::algod::v2::{PendingTransaction, TransactionResponse};
use algonaut::transaction::transaction::{
    AssetAcceptTransaction,    AssetConfigurationTransaction, AssetParams, AssetTransferTransaction,
};
use algonaut::transaction::tx_group::TxGroup;
use algonaut::transaction::{Pay, TransactionType, TxnBuilder, builder::CallApplication, };
use gdnative::api::Engine;
use gdnative::prelude::*;
use gdnative::tasks::{Async, AsyncMethod, Spawner};
use std::rc::Rc;


use algonaut::atomic_transaction_composer::{AtomicTransactionComposerStatus, 
AddMethodCallParams //transaction_signer::TransactionSigner::BasicAccount, 
};

//cant get my mods
//use super::escrow::Foo;
use crate::algod::escrow::Foo;
use crate::algod::bar::Foo as OtherFoo;

//use bar::Foo as OtherFoo;
use algodot_core::Account;
use algonaut::transaction::transaction::ApplicationCallOnComplete::NoOp;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct Algodot {
    #[property(set = "Self::set_url")]
    url: String,

    #[property(set = "Self::set_token")]
    token: String,

    #[property(set = "Self::set_headers")]
    headers: StringArray,

    algod: Rc<Algod>,
}

impl Algodot {
    fn new(_base: &Node) -> Self {
        Algodot {
            url: String::new(),
            token: String::new(),
            headers: StringArray::new(),

            // algod will be initialised on _enter_tree()
            // leave these default values here for now
            algod: Rc::new(
                Algod::new(
                   "http://localhost:4001",
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                )
                .unwrap(),
            ),
        }
    }

    fn register(builder: &ClassBuilder<Algodot>) {
        Self::register_signals(builder);

        // made with asyncmethods! macro
        register_methods(builder);
    }

    fn register_signals(builder: &ClassBuilder<Algodot>) {
        builder
            .signal("transaction_confirmed")
            .with_param_custom(SignalParam {
                name: "transaction_info".into(),
                default: ().to_variant(),
                export_info: ExportInfo::new(VariantType::Dictionary),
                usage: PropertyUsage::DEFAULT,
            })
            .done();
    }

    async fn wait_for_transaction(
        algod: Rc<Algod>,
        tx: TransactionResponse,
    ) -> Result<PendingTransaction, AlgodotError> {
        let status = algod.status().await?;
        let mut round = status.last_round - 1;
        loop {
            algod.status_after_round(Round(round)).await?;
            let txn = algod.pending_transaction_with_id(&tx.tx_id).await?;
            if let Some(confirmed_round) = txn.confirmed_round {
                if confirmed_round != 0 {
                    return Ok(txn);
                }
            } else if !txn.pool_error.is_empty() {
                return Err(AlgodotError::PoolError(txn.pool_error));
            }
            round += 1;
        }
    }
}

#[methods]
impl Algodot {
    #[method]
    fn _enter_tree(&mut self, #[base] _base: TRef<Node>) {
        self.update_algod();
    }

    #[method]
    fn set_url(&mut self, #[base] _base: TRef<Node>, url: String) {
        self.url = url;
        self.update_algod();
    }

    #[method]
    fn set_token(&mut self, #[base] _base: TRef<Node>, token: String) {
        self.token = token;
        self.update_algod();
    }

    #[method]
    fn set_headers(&mut self, #[base] _base: TRef<Node>, headers: StringArray) {
        self.headers = headers;
        self.update_algod();
    }

    fn update_algod(&mut self) {
        // Do not update while in editor
        // e.g. editing properties in the inspector
        if Engine::godot_singleton().is_editor_hint() {
            return;
        }
        let algod: Algod;
        if self.token.is_empty() {
            let headers = self
                .headers
                .read()
                .iter()
                .map(|header| -> Result<(String, String), AlgodotError> {
                    let header = &header.to_string();
                    let mut split = header.split(": ");

                    let get_string = |split: &mut std::str::Split<&str>| {
                        split
                            .next()
                            .map(|str| str.to_string())
                            .ok_or(AlgodotError::HeaderParseError)
                    };

                    Ok((get_string(&mut split)?, get_string(&mut split)?))
                })
                .collect::<Result<Vec<(String, String)>, AlgodotError>>();

            if let Some(headers) = godot_unwrap!(headers) {
                let headers: Vec<(&str, &str)> = headers
                    .iter()
                    .map(|(str1, str2)| -> (&str, &str) { (str1, str2) })
                    .collect();

                algod = Algod::with_headers(&self.url, headers).unwrap();

                self.algod = Rc::new(algod);
            }
        } else {
            algod = Algod::new(&self.url, &self.token).unwrap();
            self.algod = Rc::new(algod);
        }
    }

    #[method]
    fn generate_key(&self, #[base] _base: &Node) -> (String, String) {
        let acc = Account::generate();
        (acc.address().to_string(), acc.mnemonic())
    }

    #[method]
    fn get_address(&self, #[base] _base: &Node, mnemonic: Account) -> Address {
        mnemonic.address().into()
    }

    #[method]
    fn sign_transaction(
        &self,
        #[base] _base: &Node,
        txn: Transaction,
        signer: Account,
    ) -> Option<SignedTransaction> {
        let stxn = signer.sign_transaction(txn.into());
        godot_unwrap!(stxn).map(SignedTransaction::from)
    }

    #[method]
    fn construct_payment(
        &self,
        #[base] _base: &Node,
        params: SuggestedTransactionParams,
        sender: Address,
        receiver: Address,
        amount: u64,
    ) -> Transaction {
        TxnBuilder::with(
            &params,
            Pay::new(*sender, *receiver, MicroAlgos(amount)).build(),
        )
        .build()
        .unwrap()
        .into()
    }

    #[method]
    #[allow(clippy::too_many_arguments)]
    fn construct_asset_xfer(
        &self,
        #[base] _base: &Node,
        params: SuggestedTransactionParams,
        sender: Address,
        receiver: Address,
        amount: u64,
        asset_id: u64,
        #[opt] close_to: Option<Address>,
    ) -> Transaction {
        TxnBuilder::with(
            &params,
            TransactionType::AssetTransferTransaction(AssetTransferTransaction {
                sender: *sender,
                xfer: asset_id,
                amount,
                receiver: *receiver,
                close_to: close_to.map(|x| *x),
            }),
        )
        .build()
        .unwrap()
        .into()
    }

    #[method]
    #[allow(clippy::too_many_arguments)]
    fn construct_asset_create(
        &self,
        #[base] _base: &Node,
        params: SuggestedTransactionParams,
        sender: Address,
        asset_name: String,
        decimals: u32,
        default_frozen: bool,
        total: u64,
        unit_name: String,
        #[opt] meta_data_hash: Option<ByteArray>,
        #[opt] url: Option<String>,
        #[opt] clawback: Option<Address>,
        #[opt] freeze: Option<Address>,
        #[opt] manager: Option<Address>,
        #[opt] reserve: Option<Address>,
    ) -> Transaction {
        let mdh = meta_data_hash.map(|mdh| mdh.read().iter().copied().collect::<Vec<u8>>());

        TxnBuilder::with(
            &params,
            TransactionType::AssetConfigurationTransaction(AssetConfigurationTransaction {
                sender: *sender,
                params: Some(AssetParams {
                    asset_name: Some(asset_name),
                    decimals: Some(decimals),
                    default_frozen: Some(default_frozen),
                    total: Some(total),
                    unit_name: Some(unit_name),
                    meta_data_hash: mdh,
                    url,
                    clawback: clawback.map(|x| *x),
                    freeze: freeze.map(|x| *x),
                    manager: manager.map(|x| *x),
                    reserve: reserve.map(|x| *x),
                }),
                config_asset: None,
            }),
        )
        .build()
        .unwrap()
        .into()
    }

    
    #[method]
    #[allow(clippy::too_many_arguments)]
    fn construct_app_call(
        &self,
        #[base] _base: &Node,
        params: SuggestedTransactionParams,
        sender: Address,
        app_id: u64,
        #[opt] app_arguments: Option<String>, 
        
   
    ) -> Transaction { 

        TxnBuilder::with(
            &params,
            CallApplication::new(*sender, app_id)
                .app_arguments( vec![app_arguments.expect("REASON").into_bytes()])
                .build(),
            )
            .build()
            .unwrap()
            .into()
    }

    #[method]
    #[allow(clippy::too_many_arguments)]
    async fn construct_atc : _(
        /* Atomic Transaction Composer*/
        &self,
        #[base] _base: &Node,
        params: SuggestedTransactionParams,
        sender: Address,
        mnemonic : String,
        app_id: u64,
        #[opt] app_arguments: Option<String>, 
        
   
    ) -> Result<(), Foo> { 

       
    let mut atc = escrow::Foo::new();  



    let mut _to_addr: [u8; 32] = Foo::address_to_bytes(sender.to_string());//[0; 32];

    let __app_id : u64 = 161737986 ;
    let pages: u32 = 0;
    
    godot_dbg!("retrieving suggested params");
    let params = self.algod.suggested_transaction_params().await.unwrap();
    //Txn Details As a Struct
    let details = <bar::Foo as Trait>::Foo{ //OtherFoo::Foo { 
            withdrw_amt : 0u32,//Foo::withdraw_amount(0u32),//BigUint::new(vec![0]),//BigUint { data: vec![0u64] },//BigUint = BigUint::new(vec![0]), 
            withdrw_to_addr: _to_addr.clone(), 
            arg1: Foo::withdraw_amount(0u32), 
            arg2: Foo::address(_to_addr),
            _app_id: __app_id.clone(), 
            _escrow_address: Foo::app_address(&__app_id),//to_app_address(__app_id), 
            atc: &atc };

    //println!("{:?}", &details);
    godot_dbg!(&details);
            //Add method Call     
    atc.add_method_call( &mut AddMethodCallParams {
                    app_id: details._app_id,
                    method: bar::Foo::withdraw(), //bar::Foo::withdraw() //for deposits //bar::Foo::deposit()
                    method_args: vec![details.arg1, details.arg2],
                    fee: escrow::Foo::fee(2500),
                    sender: *sender,
                    suggested_params: params,
                    on_complete: NoOp,
                    approval_program: None,
                    clear_program: None,
                    global_schema: None,
                    local_schema: None,
                    extra_pages: pages,
                    note: Foo::note(0u32),//_note,
                    lease: None,
                    rekey_to: None,
                    signer: Foo::basic_account(&mnemonic)
            
        }
    ).unwrap();


    atc.build_group().expect("Error");

    atc.execute(&self.algod).await.expect("Error");
    
    let status_str : &mut AtomicTransactionComposerStatus = &mut atc.status();
    godot_dbg!(status_str);

    Ok(())
    }

    #[method]
    fn construct_asset_opt_in(
        &self,
        #[base] _base: &Node,
        params: SuggestedTransactionParams,
        sender: Address,
        asset_id: u64,
    ) -> Transaction {
        TxnBuilder::with(
            &params,
            TransactionType::AssetAcceptTransaction(AssetAcceptTransaction {
                sender: *sender,
                xfer: asset_id,
            }),
        )
        .build()
        .unwrap()
        .into() //uses Core Traits
    }

    #[method]
    /// Give transactions same group id
    fn group_transactions(
        &self,
        #[base] _base: &Node,
        mut txns: Vec<Transaction>,
    ) -> Option<Vec<Transaction>> {
        let mut txns_mut_refs: Vec<&mut algonaut::transaction::Transaction> =
            txns.iter_mut().map(|tx| &mut tx.0).collect();
        let result = TxGroup::assign_group_id(txns_mut_refs.as_mut_slice());
        godot_unwrap!(result).map(|_| txns)
    }
}

asyncmethods!(algod, node, this,
    fn health(_ctx, _args) {
        async move {
            let status = algod.health().await;

            match status {
                Ok(_) => 0.to_variant(), // OK
                Err(_) => 1.to_variant(), // FAILED
            }
        }
    }

    fn suggested_transaction_params(_ctx, _args) {
        async move {
            let params = algod.suggested_transaction_params().await.map(SuggestedTransactionParams::from);
            godot_unwrap!(params).to_variant()
        }
    }

    fn status(_ctx, _args) {
        async move {
            let status = algod.status().await;
            godot_unwrap!(status).map(|status| to_json_dict(&status)).to_variant()
        }
    }

    fn account_information(_ctx, args) {
        let address = args.read::<Address>().get().unwrap();
        async move {
            let info = algod.account_information(&address).await;
            godot_unwrap!(info).map(|info| to_json_dict(&info)).to_variant()
        }
    }

    fn transaction_information(_ctx, args) {
        let txid = args.read::<String>().get().unwrap();

        async move {
            let info = algod.pending_transaction_with_id(txid.as_ref()).await;
            godot_unwrap!(info).map(|info| to_json_dict(&info)).to_variant()
        }
    }

    fn send_transaction(_ctx, args) {
        let txn = args.read::<SignedTransaction>().get().unwrap();

        async move {
            let txid = algod.broadcast_signed_transaction(&txn).await;
            godot_unwrap!(txid).map(|txid| txid.tx_id).to_variant()
        }
    }

    fn wait_for_transaction(_ctx, args) {
        let tx_id = args.read::<String>().get().unwrap();

        async move {
            let pending_tx = Algodot::wait_for_transaction(algod, TransactionResponse { tx_id }).await;
            godot_unwrap!(pending_tx).map(|tx| to_json_dict(&tx)).to_variant()
        }
    }

    fn send_transactions(_ctx, args) {
        let vartxns = args.read::<Vec<SignedTransaction>>().get().unwrap();
        let txns: Vec<algonaut::transaction::SignedTransaction> = vartxns.iter().map(|tx| tx.0.clone()).collect();

        async move {
            let txid = algod.broadcast_signed_transactions(txns.as_slice()).await;
            godot_unwrap!(txid).map(|txid| to_json_dict(&txid)).to_variant()
        }
    }

    fn compile_teal(_ctx, args) {
        let source_code = args.read::<String>().get().unwrap();

        async move {
            let compiled = algod.compile_teal(source_code.as_bytes()).await;
            godot_unwrap!(compiled).map(|c| (c.hash().0.to_vec().to_variant(), c.bytes_to_sign().to_variant())).to_variant()
        }
    }

    //fn execute(&algod) {atc.execute(&algod).await.expect("Error")
    //}
);
