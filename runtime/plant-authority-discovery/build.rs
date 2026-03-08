fn main() {
	#[cfg(feature = "std")]
	prost_build::compile_protos(
		&[
			"src/client/worker/schema/dht-v1.proto",
			"src/client/worker/schema/dht-v2.proto",
			"src/client/worker/schema/dht-v3.proto",
		],
		&["src/client/worker/schema"],
	)
	.unwrap();
}
