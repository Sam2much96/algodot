/*
This Module Exposes Algonaut Algod Node To Godot, handles Errors and Connects Algonaut_crypto,
Algonaut ABI, and My Custom Escrow Smart Contract Through Here To Godot


This code base is split into 2
Unrigistered Async Functions
Restesitered Async Methods that implement the unregistered code
THe Registered methods implement the unreigistered class via functional and OOP code structure
Where Algod node is the obj and the unregistered async methods are the functional code
I Hope That makes sense
It Doesn't necessarily require the gdnative async macro with this implementation

TO DO:
(1) Implement Block Storage in via Algod Node
(2) Transaction Params Shluld Be a shared struct with proper traits btw algodot core, abi and algodot node
*/

//use algodot_abi::abi_smartcontract::*;
use algodot_abi::abi_smartcontract::Foo as abiFoo;
use algodot_abi::escrow::Foo as escrowFoo;
//use algodot_abi::escrow::MyTransactionParams; // Get Custom transation Params that implements all traits and is reusable

use algodot_core::*; // Godot Class Exposing Algonaut Core
use algodot_macros::*; // Plugin Custom Macros
use algonaut::algod::v2::Algod;
use algonaut::core::{Address as OtherAddress, MicroAlgos}; //Round
use algonaut::transaction::transaction::{
    ApplicationCallOnComplete::NoOp, AssetAcceptTransaction, AssetConfigurationTransaction,
    AssetParams, AssetTransferTransaction,
};
use algonaut::transaction::tx_group::TxGroup;
use algonaut::transaction::{
    account::Account as OtherAccount, builder::CallApplication, Pay, TransactionType, TxnBuilder,
};
use algonaut_algod::models::{PendingTransactionResponse, RawTransaction200Response};
use gdnative::api::Engine;
use gdnative::prelude::*;
use gdnative::tasks::{Async, AsyncMethod, Spawner};

use algonaut::atomic_transaction_composer::{AddMethodCallParams, ExecuteResult};
use std::rc::Rc;
//use std::sync::Arc;

#[derive(NativeClass, Clone)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct Algodot {
    #[property(set = "Self::set_url")]
    url: String,

    #[property(set = "Self::set_token")]
    token: String,

    //#[property(set = "Self::set_headers")]
    headers: PoolArray<GodotString>, // Depreciated Method
    algod: Rc<Algod>,
}

impl Algodot {
    /// Build a v2 client for Algorand protocol daemon.
    fn new(_base: &Node) -> Self {
        Algodot {
            url: String::new(),
            token: String::new(),
            headers: PoolArray::<GodotString>::new(), // Depreciated headers method in 0.4.2 algod

            /* It Is Rc Reference Counted So Rust Holds is in memory instead of deallocating it */
            // algod will be initialised on _enter_tree()
            // These are temporary default values
            // To DO: Change Default To Testnet
            algod: Rc::new(
                Algod::new(
                    "http://localhost:4001",
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                )
                .unwrap(),
            ),
        }
    }

    /* */
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

    /*
       Async Functions Writen, No Registered,
       Only Exposed To Rust. They Are Called and Implement
       Below in an Async Macro That's Registered and Accessible Via Godot

    */

    /// Waits for a block to appear after round {round} and returns the node's status at the time.
    /// DOCs: https://github.com/manuelmauro/algonaut/blob/e7702a51c7cf7ece220293fc6a723b4d10b8040a/src/algod/v2/mod.rs#L447
    async fn wait_for_block(
        algod: Rc<Algod>,
        tx: RawTransaction200Response,
    ) -> Result<PendingTransactionResponse, AlgodotError> {
        let status = algod.status().await?;
        let mut round = status.last_round - 1;

        // Recursively
        loop {
            //let given_round = Round(round)
            algod.status_after_block(round).await?; // Gets the node status after waiting for the given round.
            let txn = algod.pending_txn(&tx.tx_id).await?; //Get a specific pending transaction.
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

/*
Non Async Functions Registered With The Method Macro

*/

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

    // Requires with_header method which is depreciated in bleeding edge algonaut build
    #[method]
    fn set_headers(&mut self, #[base] _base: TRef<Node>, headers: PoolArray<GodotString>) {
        self.headers = headers;
        //self.update_algod();
        // Headers Functinality Is Depreciated In ALgonaut 0.4.2
    }

    fn update_algod(&mut self) {
        // Do not update while in editor
        // e.g. editing properties in the inspector
        if Engine::godot_singleton().is_editor_hint() {
            return;
        }
        let algod: Algod;
        //This code checks if the token is empty and, if so, processes HTTP headers stored in a shared,
        //thread-safe manner. It uses the read() method to access the headers and iterates over them,
        //It attempts to retrieve both the key and value of each header, converting them into strings. If parsing fails, it returns an AlgodotError::HeaderParseError.
        //It collects the results into a vector of header tuples ((String, String)) or returns an error.
        // Error Catcher for If token parameters is empty, it maps the headers to the token
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

            // Depreciated with_headers method in algonaut 0.4.2
            // if let Some(headers) = godot_unwrap!(headers) {
            //   let headers: Vec<(&str, &str)> = headers
            //       .iter()
            //       .map(|(str1, str2)| -> (&str, &str) { (str1, str2) })
            //        .collect();

            //algod = Algod::with_headers(&self.url, headers).unwrap(); // Deprecaiated Method
            //    algod = Algod::with_headers(&self.url, headers).unwrap();
            //    self.algod = Rc::new(algod);
            //}
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
                .app_arguments(vec![app_arguments.expect("REASON").into_bytes()])
                .build(),
        )
        .build()
        .unwrap()
        .into()
    }
    /*
    #[method(async)]
    #[allow(clippy::too_many_arguments)]
    async fn construct_atc(
        /*
        Atomic Transaction Composer

        An exported async method that returns a dictionary of the tx id or error code
        */
        //#[async_ctx]
        &self,
        params: SuggestedTransactionParams,
        sender: Address,
        mnemonic: String,
        app_id: u64,
        #[opt] _app_arguments: Option<String>,
    ) -> Dictionary {
        let dict = Dictionary::new();

    //
                let mut atc = escrowFoo::new_atc();
                let mut _to_addr: [u8; 32] = escrowFoo::address_to_bytes(sender.to_string());

                let _acct: OtherAccount = OtherAccount::from_mnemonic(&mnemonic).unwrap();
                let mut _my_addr: OtherAddress = escrowFoo::address_to_address(&_acct.address().to_string());

                let pages: u32 = 0;
                let details = escrowFoo {
                    withdrw_amt: 0u32,
                    withdrw_to_addr: _to_addr,
                    arg1: escrowFoo::withdraw_amount(5000u32),
                    arg2: escrowFoo::address(_my_addr),
                    _app_id: app_id,
                    _escrow_address: escrowFoo::app_address(&app_id),
                    atc: &atc,
                };

                let k = params.into();
                atc.add_method_call(&mut AddMethodCallParams {
                    app_id: details._app_id,
                    method: abiFoo::withdraw(),
                    method_args: vec![details.arg1, details.arg2],
                    fee: escrowFoo::fee(2500),
                    sender: *sender,
                    suggested_params: k,
                    on_complete: NoOp,
                    approval_program: None,
                    clear_program: None,
                    global_schema: None,
                    local_schema: None,
                    extra_pages: pages,
                    note: escrowFoo::note(0u32),
                    lease: None,
                    rekey_to: None,
                    signer: escrowFoo::basic_account(&mnemonic),
                }).unwrap();

                atc.build_group().expect("Error");
                let result: ExecuteResult = atc.execute(&self.algod).await.expect("Error");
                dict.insert("confirmed round", result.confirmed_round);
                dict.insert("tx_ids", result.tx_ids);
                dict
            })
            .unwrap()
    });

    dict
    }
    */

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

/*
ASync Methods Implementation + Macros

It Exposes Algonaut's Algod Node's Methods to Godot Engine
It mostly async cuz it mainly netcode which is asynchronus
e.g javascript async await
The asyncmethods macro simplifies the process of registering an
async method with Godot by automatically generating the required
AsyncMethod struct and boilerplate code for method registration.

arg functions like the kwargs** in argument python

It Also Creates A Context with algod node so you can directly reference it
*/

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
    // Gets a Suggested Transaction Parameter Algorand Protocol and Convert it to A Trait supported Type from (Core.rs) using our Custom macro
    fn suggested_transaction_params(_ctx, _args) {
        async move {
            let params = algod.txn_params().await.map(SuggestedTransactionParams::from);
            godot_unwrap!(params).to_variant()
        }
    }

    fn status(_ctx, _args) {
        async move {
            let status = algod.status().await;
            godot_unwrap!(status).map(|status| to_json_dict(&status)).to_variant()
        }
    }

    // Given a specific account public key, this call returns the accounts status, balance and spendable amounts
    fn account_information(_ctx, args) {
        let address = args.read::<String>().get().unwrap();
        async move {
            let info = algod.account(&address).await;

            //maps code output to a dictionay with custom godot unwrap macro and to_json_dict() method?
            godot_unwrap!(info).map(|info| to_json_dict(&info)).to_variant()
        }
    }

    fn transaction_information(_ctx, args) {
        let txid = args.read::<String>().get().unwrap();

        async move {
            let info = algod.pending_txn(txid.as_ref()).await;
            godot_unwrap!(info).map(|info| to_json_dict(&info)).to_variant()
        }
    }

    fn send_transaction(_ctx, args) {
        let txn = args.read::<SignedTransaction>().get().unwrap();

        async move {
            let txid = algod.send_txn(&txn).await;
            godot_unwrap!(txid).map(|txid| txid.tx_id).to_variant()
        }
    }

    fn wait_for_block(_ctx, args) {
        let tx_id = args.read::<String>().get().unwrap();

        // It implements the async function as within the class and calls it within this macro.
        // clean and efficient
        async move {
            let pending_tx = Algodot::wait_for_block(algod, RawTransaction200Response { tx_id }).await;
            godot_unwrap!(pending_tx).map(|tx| to_json_dict(&tx)).to_variant()
        }
    }

    fn send_transactions(_ctx, args) {
        let vartxns = args.read::<Vec<SignedTransaction>>().get().unwrap();
        let txns: Vec<algonaut::transaction::SignedTransaction> = vartxns.iter().map(|tx| tx.0.clone()).collect();

        async move {
            let txid = algod.send_txns(txns.as_slice()).await;
            godot_unwrap!(txid).map(|txid| to_json_dict(&txid)).to_variant()
        }
    }

    fn compile_teal(_ctx, args) {
        let source_code = args.read::<String>().get().unwrap();

        async move {
            let compiled = algod.teal_compile(source_code.as_bytes(), None).await;
            godot_unwrap!(compiled).map(|c| (c.hash().0.to_vec().to_variant(), c.bytes_to_sign().to_variant())).to_variant()
        }
    }

    fn construct_atc(ctx, args) {
        // Get Params
        // Extract the arguments from `args` as needed
        let params: SuggestedTransactionParams = args.read::<SuggestedTransactionParams>().get().unwrap();

        // TO DO : Send String and Convert string to address for this params
        let mut sender: String = args.read::<String>().get().unwrap(); // sender address

        // secret key
        let mnemonic: String = args.read::<String>().get().unwrap();

        // app id
        let app_id: u64 = args.read::<u64>().get().unwrap();

        // app args
        let _app_arguments: Option<String> = args.read::<Option<String>>().get().unwrap(); // Should ideally be Some()

        // create atc no async
        let mut atc = escrowFoo::new_atc();
        //let mut s

        // convert address to bytes
        let mut _to_addr: [u8; 32] = escrowFoo::address_to_bytes(sender.clone());

        // i probably couldnt clone account address thats why i wrote this sloppy code
        //let _acct: OtherAccount = OtherAccount::from_mnemonic(&mnemonic).unwrap(); // Generate Account From Mnenomic

        // sender address transmuted to  address from string and Address to Address
        let mut _my_addr: OtherAddress = escrowFoo::address_to_address(&sender); //Generate Address String from address
        let _arg2 = escrowFoo::address(_my_addr.clone());
        let pages: u32 = 0; // no pages

        // Transaction details
        let details = escrowFoo {
            withdrw_amt: 0u32,
            withdrw_to_addr: _to_addr,
            arg1: escrowFoo::withdraw_amount(5000u32),
            arg2: _arg2,
            _app_id: app_id,
            _escrow_address: escrowFoo::app_address(&app_id),
            atc: &atc,
        };

        let k = params.into();

        atc.add_method_call(&mut AddMethodCallParams {
            app_id: details._app_id,
            method: abiFoo::withdraw(), //abiFoo::deposit() to deposit
            method_args: vec![details.arg1, details.arg2],
            fee: escrowFoo::fee(2500),
            sender: _my_addr, // Covert String Address TO Address Address
            suggested_params: k,
            on_complete: NoOp,
            approval_program: None,
            clear_program: None,
            global_schema: None,
            local_schema: None,
            extra_pages: pages,
            note: escrowFoo::note(0u32),
            lease: None,
            rekey_to: None,
            signer: escrowFoo::basic_account(&mnemonic),// Create A Txn Signer
        })
        .unwrap();

        atc.build_group().expect("Error Building ATC Group");

        let dict = Dictionary::new()
        async move{
        // Execute the ATC
        let result: ExecuteResult = atc.execute(&algod).await.expect("ATC Execute Error");
        dict.insert("confirmed round", result.confirmed_round);
        dict.insert("tx_ids", result.tx_ids);

        //let p = to_json_dict(&result);
        godot_unwrap!(dict).to_variant()


    }}


);
