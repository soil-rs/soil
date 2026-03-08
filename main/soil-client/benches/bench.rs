// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use criterion::{criterion_group, criterion_main, Criterion};
use tracing_subscriber::fmt::{
	format,
	time::{ChronoLocal, FormatTime},
};

fn bench_fast_local_time(c: &mut Criterion) {
	c.bench_function("fast_local_time", |b| {
		let mut buffer = String::new();
		let t = soil_client::tracing::logging::FastLocalTime { with_fractional: true };
		b.iter(|| {
			buffer.clear();
			let mut writer = format::Writer::new(&mut buffer);
			t.format_time(&mut writer).unwrap();
		})
	});
}

// This is here just as a point of comparison.
fn bench_chrono_local(c: &mut Criterion) {
	c.bench_function("chrono_local", |b| {
		let mut buffer = String::new();
		let t = ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_string());
		b.iter(|| {
			buffer.clear();
			let mut writer: format::Writer<'_> = format::Writer::new(&mut buffer);
			t.format_time(&mut writer).unwrap();
		})
	});
}

criterion_group! {
	name = benches;
	config = Criterion::default();
	targets = bench_fast_local_time, bench_chrono_local
}
criterion_main!(benches);
