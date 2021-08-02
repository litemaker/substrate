mod primitives;

use sc_telemetry::TelemetryWorker;
use sp_runtime::traits::Block as BlockT;
use sp_timestamp::InherentDataProvider;
use std::sync::Arc;

use primitives::Block;

type RuntimeApi = ();

pub enum DatabaseType {}

pub struct EasyBuilder {
	pub database_type: DatabaseType,
	pub config: sc_service::Configuration,
}

impl EasyBuilder {
	pub fn build(&self, wasm_blob: Vec<u8>) -> Result<(), sc_service::error::Error> {
		let EasyBuilder { config, .. } = self;

		let executor = sc_executor::WasmExecutor::new(
			sc_executor::WasmExecutionMethod::Compiled,
			None,
			Vec::new(),
			1,
			None,
		);

		let telemetry = config
			.telemetry_endpoints
			.clone()
			.filter(|x| !x.is_empty())
			.map(|endpoints| -> Result<_, sc_telemetry::Error> {
				let worker = TelemetryWorker::new(16)?;
				let telemetry = worker.handle().new_telemetry(endpoints);
				Ok((worker, telemetry))
			})
			.transpose()?;

		let (client, backend, keystore_container, task_manager) =
			sc_service::new_full_parts_with_executor::<Block, RuntimeApi, _>(
				&config,
				telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
				executor,
			)?;
		let client = Arc::new(client);

		let telemetry = telemetry.map(|(worker, telemetry)| {
			task_manager.spawn_handle().spawn("telemetry", worker.run());
			telemetry
		});

		let select_chain = sc_consensus::LongestChain::new(backend.clone());

		let transaction_pool = sc_transaction_pool::BasicPool::new_full(
			config.transaction_pool.clone(),
			config.role.is_authority().into(),
			config.prometheus_registry(),
			task_manager.spawn_essential_handle(),
			client.clone(),
		);

		let (grandpa_block_import, grandpa_link) = grandpa::block_import(
			client.clone(),
			&(client.clone() as Arc<_>),
			select_chain.clone(),
			telemetry.as_ref().map(|x| x.handle()),
		)?;
		let justification_import = grandpa_block_import.clone();

		let (block_import, babe_link) = sc_consensus_babe::block_import(
			sc_consensus_babe::Config::get_or_compute(&*client)?,
			grandpa_block_import,
			client.clone(),
		)?;

		let slot_duration = babe_link.config().slot_duration();
		let import_queue = sc_consensus_babe::import_queue(
			babe_link.clone(),
			block_import.clone(),
			Some(Box::new(justification_import)),
			client.clone(),
			select_chain.clone(),
			move |_, ()| async move {
				let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

				let slot =
					sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_duration(
						*timestamp,
						slot_duration,
					);

				let uncles =
                    sp_authorship::InherentDataProvider::<<Block as BlockT>::Header>::check_inherents();

				Ok((timestamp, slot, uncles))
			},
			&task_manager.spawn_essential_handle(),
			config.prometheus_registry(),
			sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone()),
			telemetry.as_ref().map(|x| x.handle()),
		)?;

		Ok(())
	}
}
