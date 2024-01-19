use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::{BigUintApi, ErrorApi, SendApi};
use crate::io::EndpointResult;
use crate::types::{Address, ArgBuffer, BoxedBytes};
use alloc::string::String;
use alloc::vec::Vec;

#[must_use]
pub struct TransferMoaExecute<BigUint: BigUintApi> {
	pub(super) to: Address,
	pub(super) moa_payment: BigUint,
	pub(super) endpoint_name: BoxedBytes,
	pub(super) arg_buffer: ArgBuffer,
	pub(super) gas_limit: u64,
}

impl<BigUint> TransferMoaExecute<BigUint>
where
	BigUint: BigUintApi + 'static,
{
	pub fn with_gas_limit(self, gas_limit: u64) -> Self {
		TransferMoaExecute { gas_limit, ..self }
	}
}

impl<FA, BigUint> EndpointResult<FA> for TransferMoaExecute<BigUint>
where
	BigUint: BigUintApi + 'static,
	FA: SendApi<BigUint> + ErrorApi + Clone + 'static,
{
	#[inline]
	fn finish(&self, api: FA) {
		api.direct_moa_execute(
			&self.to,
			&self.moa_payment,
			self.gas_limit,
			self.endpoint_name.as_slice(),
			&self.arg_buffer,
		);
	}
}

impl<BigUint: BigUintApi> TypeAbi for TransferMoaExecute<BigUint> {
	fn type_name() -> String {
		"TransferMoaExecute".into()
	}

	/// No ABI output.
	fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
		Vec::new()
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
