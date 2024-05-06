//use algodot_abi::abi_smartcontract::*;
//use algodot_abi::abi_smartcontract::Foo as abiFoo;
//use algodot_abi::escrow::Foo as escrowFoo;
//use algodot_core::*; //temprarily disabled for refactoring
//use algodot_macros::*;

// All Algonaut Classes should be exposed in their sub modules, sharing only core algorand scripts required for txn signing

use algonaut::algod::v2::Algod;
use algonaut::error::ServiceError;

//use algonaut::core::{Address as OtherAddress, MicroAlgos, Round};
use algonaut::model::algod::v2::{PendingTransaction, TransactionResponse};

//use algonaut::transaction::transaction::{
//    ApplicationCallOnComplete::NoOp, AssetAcceptTransaction, AssetConfigurationTransaction,
//    AssetParams, AssetTransferTransaction,
//};
//use algonaut::transaction::tx_group::TxGroup;
//use algonaut::transaction::{
//    account::Account as OtherAccount, builder::CallApplication, Pay, TransactionType, TxnBuilder,
//};
// Gdnative is depreciated in godot 4.0 using gdext instead with tokio macros until Asyn is implemented in GExt
//use gdnative::tasks::{Async, AsyncMethod, Spawner};
use godot::builtin::meta::{ConvertError, FromGodot, GodotConvert, GodotType, ToGodot};
use godot::builtin::*;
//use godot::engine::Engine;
use godot::prelude::*;

use std::rc::Rc; //used for reference counting

//use algonaut::atomic_transaction_composer::{AddMethodCallParams, ExecuteResult};
//use paste::*;

#[derive(GodotClass, Clone, Debug)]
#[class(no_init,base=Node)]
//#[register_with(Self::register)] //depreciated macro

// disabled for refactoring
pub struct Algodot {
    //    //property macro?
    //#[property(set = "Self::set_url")]
    url: String,
    //
    //    #[property(set = "Self::set_token")]
    token: String,

    //    #[property(set = "Self::set_headers")]
    headers: PackedStringArray, //PoolArray<GodotString>,

    algod: Rc<Algod>,
}

// required traits From Godot and To Godot
/* Should unwrap algod node to a Godot Dictionary */
// try using algfonaut serder_json

impl GodotConvert for Algodot {
    type Via = Dictionary;
}

impl FromGodot for Algodot {
    fn from_godot(via: Self::Via) -> Self {
        todo!()
    }

    fn from_variant(variant: &Variant) -> Self {
        todo!()
    }
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        todo!()
    }
}

impl ToGodot for Algodot {
    fn into_godot(self) -> Self::Via {
        todo!()
    }

    fn to_variant(&self) -> Variant {
        todo!()
    }

    fn to_godot(&self) -> Self::Via {
        todo!()
    }
}

#[godot_api]
impl Algodot {
    #[func]
    fn new(_base: &Node) -> Self {
        Algodot {
            url: String::new(),
            token: String::new(),
            headers: PackedStringArray::new(),

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
    /* Depreciated method instead, call the #[derive(GodotClass)] macro instead*/
    //fn register(builder: &ClassBuilder<Algodot>) {
    //    Self::register_signals(builder);

    // made with asyncmethods! macro
    //    register_methods(builder);
    //}

    //fn register_signals(builder: &ClassBuilder<Algodot>) {
    //    builder
    //        .signal("transaction_confirmed")
    //        .with_param_custom(SignalParam {
    //            name: "transaction_info".into(),
    //            default: ().to_variant(),
    //            export_info: ExportInfo::new(VariantType::Dictionary),
    //            usage: PropertyUsage::DEFAULT,
    //        })
    //        .done();
    //}

    /* */

    /* */

    //#[method(async)]

    /* Should unwrap a tokio async to a Godot Dictionary */
    /* SHould return an Error */
    // Documentation : https://docs.rs/algonaut/latest/algonaut/algod/v2/struct.Algod.html
    #[tokio::main]
    async fn wait_for_transaction(
        algod: Rc<Algod>,
        tx: TransactionResponse,
    ) -> Result<PendingTransaction, ServiceError> {
        //AlgodotError> {
        let status = algod.status().await?;
        let mut round = status.last_round - 1;
        loop {
            // temporarily disabled//algod.status_after_round(Round(round)).await?;
            let txn = algod.pending_transaction_with_id(&tx.tx_id).await?;
            if let Some(confirmed_round) = txn.confirmed_round {
                if confirmed_round != 0 {
                    return Ok(txn);
                }
            } else if !txn.pool_error.is_empty() {
                return todo!(); //Err(AlgodotError::PoolError(txn.pool_error));
            }
            round += 1;
        }
    }
}

//#[godot_api] //#[methods]
impl Algodot {
    // Temporarily disabing for debugging
    //#[func]
    //fn _enter_tree(&mut self) {
    //    self.update_algod();
    //}

    //#[func]
    //fn set_url(&mut self) {
    //    self.url = url;
    //    self.update_algod();
    //}

    //#[func]
    //fn set_token(&mut self, token: String) {
    //    self.token = token;
    //    self.update_algod();
    //}

    //#[func]
    //fn set_headers(&mut self, headers: PackedStringArray) {
    //    self.headers = headers;
    //    self.update_algod();
    //}

    //#[func]
    //fn update_algod(&mut self) {
    // Do not update while in editor
    // e.g. editing properties in the inspector
    //   if Engine::godot_singleton().is_editor_hint() {
    //       return;
    //   }
    //    let algod: Algod;
    //    if self.token.is_empty() {
    //        let headers = self
    //            .headers
    //            .read()
    //            .iter()
    //            .map(|header| -> Result<(String, String), AlgodotError> {
    //                let header = &header.to_string();
    //                let mut split = header.split(": ");

    //                let get_string = |split: &mut std::str::Split<&str>| {
    //                    split
    //                        .next()
    //                        .map(|str| str.to_string())
    //                        .ok_or(AlgodotError::HeaderParseError)
    //               };

    //                Ok((get_string(&mut split)?, get_string(&mut split)?))
    //            })
    //            .collect::<Result<Vec<(String, String)>, AlgodotError>>();

    //        if let Some(headers) = godot_unwrap!(headers) {
    //            let headers: Vec<(&str, &str)> = headers
    //                .iter()
    //                .map(|(str1, str2)| -> (&str, &str) { (str1, str2) })
    //                .collect();

    //            algod = Algod::with_headers(&self.url, headers).unwrap();

    //            self.algod = Rc::new(algod);
    //        }
    //    } else {
    //        algod = Algod::new(&self.url, &self.token).unwrap();
    //        self.algod = Rc::new(algod);
    //    }
    //}
    /*
    #[func]
    fn generate_key(&self) -> (String, String) {
        let acc = Account::generate();
        (acc.address().to_string(), acc.mnemonic())
    }

    #[func]
    fn get_address(&self, #[base] _base: &Node, mnemonic: Account) -> Address {
        mnemonic.address().into()
    }

    #[func]
    fn sign_transaction(
        &self,
        //#[base] _base: &Node,
        txn: Transaction,
        signer: Account,
    ) -> Option<SignedTransaction> {
        let stxn = signer.sign_transaction(txn.into());
        godot_unwrap!(stxn).map(SignedTransaction::from)
    }

    #[func]
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

    #[func]
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

    #[func]
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
        #[opt] meta_data_hash: Option<PoolArray<u8>>,
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

    #[func]
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
                .app_arguments(vec![app_arguments.expect("REASON").into_bytes()])
                .build(),
        )
        .build()
        .unwrap()
        .into()
    }

    #[method(async)]
    #[allow(clippy::too_many_arguments)]
    async fn construct_atc(
        /*
        Atomic Transaction Composer

        An exported async method that returns a dictionary of the tx id or error code
        */
        //#[async_ctx]
        //#[opt] &self,
        params: SuggestedTransactionParams,
        sender: Address,
        mnemonic: String,
        app_id: u64,
        #[opt] _app_arguments: Option<String>,
    ) -> Dictionary {
        //Returns Opaque Type //Result<ExecuteResult, ServiceError> //Result<(), Foo>

        let mut atc = escrowFoo::new_atc();

        let mut _to_addr: [u8; 32] = escrowFoo::address_to_bytes(sender.to_string()); //[0; 32];

        let _acct: OtherAccount = OtherAccount::from_mnemonic(&mnemonic).unwrap();
        //let _addr_as_string: String = _acct.address().to_string();

        let mut _my_addr: OtherAddress =
            escrowFoo::address_to_address(&_acct.address().to_string());

        let pages: u32 = 0;

        //Txn Details As a Struct
        let details = escrowFoo {
            withdrw_amt: 0u32,
            withdrw_to_addr: _to_addr,
            arg1: escrowFoo::withdraw_amount(5000u32),
            arg2: escrowFoo::address(_my_addr),
            _app_id: app_id, //__app_id.clone(),
            _escrow_address: escrowFoo::app_address(&app_id),
            atc: &atc,
        };

        let k = params.into();

        //godot_dbg!(" Params Debug: {}", &k);

        atc.add_method_call(&mut AddMethodCallParams {
            app_id: details._app_id,
            method: abiFoo::withdraw(), //bar::Foo::withdraw() //for deposits //bar::Foo::deposit()
            method_args: vec![details.arg1, details.arg2],
            fee: escrowFoo::fee(2500),
            sender: *sender,
            suggested_params: k, //params.into(),
            on_complete: NoOp,
            approval_program: None,
            clear_program: None,
            global_schema: None,
            local_schema: None,
            extra_pages: pages,
            note: escrowFoo::note(0u32), //_note,
            lease: None,
            rekey_to: None,
            signer: escrowFoo::basic_account(&mnemonic),
        })
        .unwrap();

        atc.build_group().expect("Error");

        //godot_dbg!(" ATC Debug: {}", &atc);

        //Testnet
        // Should ideally get initialization code from Algodot Type but
        // That would require editting the init variables to global variables with lifetimes

        let url = String::from("https://node.testnet.algoexplorerapi.io");

        let user = String::from("User-Agent");
        let pass = String::from("DoYouLoveMe?");
        let headers: Vec<(&str, &str)> = vec![(&user, &pass)];
        let po = Algod::with_headers(&url, headers).unwrap();
        let result: ExecuteResult = atc.execute(&po).await.expect("Error");
        godot_dbg!(atc.status());

        //Ok(())
        // implement To and From Variant traits in ATC for Easy executing and Parsing
        // implement traits for ExecuteResult and Service Error
        // Parse ATC.build group() to .json
        // Rewrite this method as async
        // figure out async macro in core.rs
        let dict = Dictionary::new();
        dict.insert("confirmed round", result.confirmed_round);
        dict.insert("tx_ids", result.tx_ids);
        //dict.insert("status ", atc.status());
        dict.into()
    }

    #[func]
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

    #[func]
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
    */
}

/*ASync Methods*/
// Uses macros/lib.rs for implementation
// temporarily depreciated for debugging
/* Impelements Async Methods using Algodot Macros, Handles Error and maps the results to algod and node objects*/
//asyncmethods!(algod, node, this,
//    fn health(_ctx, _args) {
//        async move {
//            let status = algod.health().await;
//
//            match status {
//                Ok(_) => 0.to_variant(), // OK
//                Err(_) => 1.to_variant(), // FAILED
//            }
//        }
//    }

//    fn suggested_transaction_params(_ctx, _args) {
//        async move {
//let params = algod.suggested_transaction_params().await.map(SuggestedTransactionParams::from);
//godot_unwrap!(params).to_variant()
//            todo!()
//        }
//    }
/*
fn execute (_ctx, _args){
    /*
    async move {
        atc.execute(algod);//.await.expect("Error");
    }; */
    //todo!()
}
*/
//    fn status(_ctx, _args) {
//        async move {
//            let status = algod.status().await;
//            godot_unwrap!(status).map(|status| to_json_dict(&status)).to_variant()
//        }
//    }

//   fn account_information(_ctx, args) {
//       let address = args.read::<Address>().get().unwrap();
//       async move {
//           let info = algod.account_information(&address).await;
//           godot_unwrap!(info).map(|info| to_json_dict(&info)).to_variant()
//      }
//  }

//    fn transaction_information(_ctx, args) {
//        let txid = args.read::<String>().get().unwrap();

//        async move {
//            let info = algod.pending_transaction_with_id(txid.as_ref()).await;
//            godot_unwrap!(info).map(|info| to_json_dict(&info)).to_variant()
//        }
//    }

//    fn send_transaction(_ctx, args) {
//        let txn = args.read::<SignedTransaction>().get().unwrap();

//        async move {
//            let txid = algod.broadcast_signed_transaction(&txn).await;
//            godot_unwrap!(txid).map(|txid| txid.tx_id).to_variant()
//        }
//    }

//   fn wait_for_transaction(_ctx, args) {
//       let tx_id = args.read::<String>().get().unwrap();

//        async move {
//            let pending_tx = Algodot::wait_for_transaction(algod, TransactionResponse { tx_id }).await;
//            godot_unwrap!(pending_tx).map(|tx| to_json_dict(&tx)).to_variant()
//        }
//    }

//    fn send_transactions(_ctx, args) {
//        let vartxns = args.read::<Vec<SignedTransaction>>().get().unwrap();
//        let txns: Vec<algonaut::transaction::SignedTransaction> = vartxns.iter().map(|tx| tx.0.clone()).collect();

//        async move {
//            let txid = algod.broadcast_signed_transactions(txns.as_slice()).await;
//            godot_unwrap!(txid).map(|txid| to_json_dict(&txid)).to_variant()
//        }
//    }

//    fn compile_teal(_ctx, args) {
//        let source_code = args.read::<String>().get().unwrap();
//
//        async move {
//            let compiled = algod.compile_teal(source_code.as_bytes()).await;
//            godot_unwrap!(compiled).map(|c| (c.hash().0.to_vec().to_variant(), c.bytes_to_sign().to_variant())).to_variant()
//        }
//    }

//);
