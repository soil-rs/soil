// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0 OR GPL-3.0-or-later WITH Classpath-exception-2.0

use criterion::{criterion_group, criterion_main, Criterion};
use subsoil::api::ProvideRuntimeApi;
use soil_test_node_runtime_client::{
	runtime::TestAPI, DefaultTestClientBuilderExt, TestClientBuilder, TestClientBuilderExt,
};

fn sp_api_benchmark(c: &mut Criterion) {
	c.bench_function("add one with same runtime api", |b| {
		let client = soil_test_node_runtime_client::new();
		let runtime_api = client.runtime_api();
		let best_hash = client.chain_info().best_hash;

		b.iter(|| runtime_api.benchmark_add_one(best_hash, &1))
	});

	c.bench_function("add one with recreating runtime api", |b| {
		let client = soil_test_node_runtime_client::new();
		let best_hash = client.chain_info().best_hash;

		b.iter(|| client.runtime_api().benchmark_add_one(best_hash, &1))
	});

	c.bench_function("vector add one with same runtime api", |b| {
		let client = soil_test_node_runtime_client::new();
		let runtime_api = client.runtime_api();
		let best_hash = client.chain_info().best_hash;
		let data = vec![0; 1000];

		b.iter_with_large_drop(|| runtime_api.benchmark_vector_add_one(best_hash, &data))
	});

	c.bench_function("vector add one with recreating runtime api", |b| {
		let client = soil_test_node_runtime_client::new();
		let best_hash = client.chain_info().best_hash;
		let data = vec![0; 1000];

		b.iter_with_large_drop(|| client.runtime_api().benchmark_vector_add_one(best_hash, &data))
	});

	c.bench_function("calling function by function pointer in wasm", |b| {
		let client = TestClientBuilder::new().build();
		let best_hash = client.chain_info().best_hash;
		b.iter(|| client.runtime_api().benchmark_indirect_call(best_hash).unwrap())
	});

	c.bench_function("calling function", |b| {
		let client = TestClientBuilder::new().build();
		let best_hash = client.chain_info().best_hash;
		b.iter(|| client.runtime_api().benchmark_direct_call(best_hash).unwrap())
	});
}

criterion_group!(benches, sp_api_benchmark);
criterion_main!(benches);
