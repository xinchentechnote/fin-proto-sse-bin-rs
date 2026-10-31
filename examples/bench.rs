//! Encode/decode benchmarks for the SSE binary library, mirroring
//! bench/bench.zig in fin-proto-sse-bin-zig: same methodology (10k warmup
//! iterations, then 10 batches of 100k operations, fastest batch reported),
//! so the numbers are directly comparable.
//!
//! Output is one TSV line per case on stdout:
//!   lib \t lang \t op \t ns_per_op \t frame_bytes
//!
//! Run with: cargo run --release --example bench

use std::hint::black_box;
use std::time::Instant;

use binary_codec::BinaryCodec;
use bytes::{Bytes, BytesMut};
use sse_binary::new_order_single::{NewOrderSingle, NewOrderSingleRef};

const LIB: &str = "sse";
const LANG: &str = "rust";
const ITERS: usize = 100_000;
const BATCHES: usize = 10;
const WARMUP: usize = 10_000;

fn msg() -> NewOrderSingle {
    NewOrderSingle {
        biz_id: 123456,
        biz_pbu: "PBU0001".to_string(),
        cl_ord_id: "ORD000001".to_string(),
        security_id: "600000".to_string(),
        account: "ACC000000001".to_string(),
        owner_type: 1,
        side: "1".to_string(),
        price: 10500,
        order_qty: 100,
        ord_type: "2".to_string(),
        time_in_force: "0".to_string(),
        transact_time: 20_260_109_093_000_123,
        credit_tag: "XY".to_string(),
        clearing_firm: "01".to_string(),
        branch_id: "0001".to_string(),
        user_info: "user01".to_string(),
    }
}

fn main() {
    // --- encode -------------------------------------------------------------
    let msg = msg();
    let mut buf = BytesMut::with_capacity(1024);
    for _ in 0..WARMUP {
        buf.clear();
        msg.encode(&mut buf);
    }
    let frame_bytes = buf.len();
    let mut best: u128 = u128::MAX;
    for _ in 0..BATCHES {
        let start = Instant::now();
        for _ in 0..ITERS {
            buf.clear();
            msg.encode(&mut buf);
        }
        best = best.min(start.elapsed().as_nanos());
    }
    black_box(&buf);
    println!(
        "{}\t{}\tencode\t{:.1}\t{}",
        LIB,
        LANG,
        best as f64 / ITERS as f64,
        frame_bytes
    );

    // --- decode -------------------------------------------------------------
    let raw: &'static [u8] = Box::leak(buf.to_vec().into_boxed_slice());
    for _ in 0..WARMUP {
        let mut b = Bytes::from_static(raw);
        let m = NewOrderSingle::decode(&mut b).unwrap();
        black_box(m);
    }
    let mut best: u128 = u128::MAX;
    for _ in 0..BATCHES {
        let start = Instant::now();
        for _ in 0..ITERS {
            let mut b = Bytes::from_static(raw);
            let m = NewOrderSingle::decode(&mut b).unwrap();
            black_box(m);
        }
        best = best.min(start.elapsed().as_nanos());
    }
    println!(
        "{}\t{}\tdecode\t{:.1}\t{}",
        LIB,
        LANG,
        best as f64 / ITERS as f64,
        raw.len()
    );

    // --- owned decode with reused String capacity --------------------------
    let mut reused = msg.clone();
    for _ in 0..WARMUP {
        let mut b = Bytes::from_static(raw);
        NewOrderSingle::decode_into(&mut b, &mut reused).unwrap();
        black_box(&reused);
    }
    let mut best: u128 = u128::MAX;
    for _ in 0..BATCHES {
        let start = Instant::now();
        for _ in 0..ITERS {
            let mut b = Bytes::from_static(raw);
            NewOrderSingle::decode_into(&mut b, &mut reused).unwrap();
            black_box(&reused);
        }
        best = best.min(start.elapsed().as_nanos());
    }
    println!(
        "{}\t{}\tdecode_reuse\t{:.1}\t{}",
        LIB,
        LANG,
        best as f64 / ITERS as f64,
        raw.len()
    );

    // --- zero-copy borrowed decode -----------------------------------------
    for _ in 0..WARMUP {
        let m = NewOrderSingleRef::from_slice(raw).unwrap();
        black_box(m);
    }
    let mut best: u128 = u128::MAX;
    for _ in 0..BATCHES {
        let start = Instant::now();
        for _ in 0..ITERS {
            let m = NewOrderSingleRef::from_slice(raw).unwrap();
            black_box(m);
        }
        best = best.min(start.elapsed().as_nanos());
    }
    println!(
        "{}\t{}\tdecode_borrowed\t{:.1}\t{}",
        LIB,
        LANG,
        best as f64 / ITERS as f64,
        raw.len()
    );
}
