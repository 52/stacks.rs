use crate::clarity::impl_display_generic;
use crate::transaction::args::UContractCallOptions;
use crate::transaction::base::impl_wrapped_transaction;
use crate::transaction::ContractCallOptions;
use crate::transaction::Error;
use crate::transaction::StacksTransaction;
use crate::transaction::Transaction;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractCall(StacksTransaction);

impl_display_generic!(ContractCall);
impl_wrapped_transaction!(ContractCall, Error);

impl Transaction for ContractCall {
    type Args = ContractCallOptions;
    type UArgs = UContractCallOptions;

    fn new(args: Self::Args) -> Result<StacksTransaction, Error> {
        todo!()
    }

    fn new_unsigned(args: Self::UArgs) -> Result<StacksTransaction, Error> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractCallMultiSig(StacksTransaction);

impl_display_generic!(ContractCallMultiSig);
impl_wrapped_transaction!(ContractCallMultiSig, Error);

impl Transaction for ContractCallMultiSig {
    type Args = ContractCallOptions;
    type UArgs = UContractCallOptions;

    fn new(args: Self::Args) -> Result<StacksTransaction, Error> {
        todo!()
    }

    fn new_unsigned(args: Self::UArgs) -> Result<StacksTransaction, Error> {
        todo!()
    }
}
