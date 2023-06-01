use gstd::ActorId;

// Ideally, it has to be improved and hide the logic of working with send_for_reply.
// It could probably be achieved via using lambdas with pseudo-code like this
// let open_request_result: Result<Request, &'static str> = request_response_client.try_open_request(
//     contract_id,
//     request_data,
//     request_nonce // optional
// );
// let request = if let Ok(request) = open_request_result {
//    request
// }
// else
// { handle and return }
//
// request.execute(
//     |request_nonce, request_data| SomeTransaction { tx_id: request_nonce, tx_payload: request_data } // Or any other transoformation for building payload
//     |contract_execution_result: Result<Vec<u8>, ContractError>| { /*Some logic for handling response. Done in lambda to highlight it is a separate storage tx scope */}
// ).await;
//
// return;

pub(crate) struct TxManager<TxData, TxContext = ()>
where
    TxData: PartialEq,
{
    pending_tx: Option<(ActorId, TxData)>,
    pending_tx_context: Option<TxContext>,
    pending_tx_nonce: u64,
}

impl<TxData: PartialEq, TxContext> TxManager<TxData, TxContext> {
    pub fn new(tx_nonce: u64) -> Self {
        Self {
            pending_tx: None,
            pending_tx_context: None,
            pending_tx_nonce: tx_nonce,
        }
    }

    pub fn open_tx(
        &mut self,
        tx_target_id: ActorId,
        tx_data: TxData,
        tx_context: TxContext,
        new_only: bool,
    ) -> Result<u64, &'static str> {
        if let Some((pending_tx_target_id, pending_tx_data)) = &self.pending_tx {
            if new_only || pending_tx_target_id != &tx_target_id || pending_tx_data != &tx_data {
                return Err("Transaction is in progress.");
            }
            return Ok(self.pending_tx_nonce);
        }
        self.pending_tx = Some((tx_target_id, tx_data));
        self.pending_tx_context = Some(tx_context);
        self.pending_tx_nonce += 1;
        Ok(self.pending_tx_nonce)
    }

    pub fn close_tx(
        &mut self,
        tx_target_id: ActorId,
        tx_data: TxData,
        tx_nonce: u64,
    ) -> Result<(), &'static str> {
        if self.pending_tx == Some((tx_target_id, tx_data)) && self.pending_tx_nonce == tx_nonce {
            self.pending_tx = None;
            self.pending_tx_context = None;
            Ok(())
        } else {
            Err("Unexpected transaction closure")
        }
    }

    pub fn pending_tx_context(&self) -> &TxContext {
        self.pending_tx_context
            .as_ref()
            .expect("Transation is not open")
    }
}
