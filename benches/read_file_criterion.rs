#![allow(missing_docs)]
//! Cross-platform benchmarks using Criterion
//! These complement the existing iai-callgrind benchmarks for Linux
//! and provide Windows/macOS developers the ability to run benchmarks locally

use lofty::config::ParseOptions;
use lofty::probe::Probe;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::io::Cursor;

// Include test data - same files as the iai benchmarks
const AAC_DATA: &[u8] = include_bytes!("./assets/01 TempleOS Hymn Risen (Remix).aac");
const AIFF_DATA: &[u8] = include_bytes!("./assets/01 TempleOS Hymn Risen (Remix).aiff");
const APE_DATA: &[u8] = include_bytes!("./assets/01 TempleOS Hymn Risen (Remix).ape");
const FLAC_DATA: &[u8] = include_bytes!("./assets/01 TempleOS Hymn Risen (Remix).flac");
const MP4_DATA: &[u8] = include_bytes!("./assets/01 TempleOS Hymn Risen (Remix).m4a");
const MP3_DATA: &[u8] = include_bytes!("./assets/01 TempleOS Hymn Risen (Remix).mp3");
const MPC_DATA: &[u8] = include_bytes!("./assets/01 TempleOS Hymn Risen (Remix).mpc");
const OPUS_DATA: &[u8] = include_bytes!("./assets/01 TempleOS Hymn Risen (Remix).opus");
const RIFF_DATA: &[u8] = include_bytes!("./assets/01 TempleOS Hymn Risen (Remix).wav");
const SPEEX_DATA: &[u8] = include_bytes!("./assets/01 TempleOS Hymn Risen (Remix).spx");
const VORBIS_DATA: &[u8] = include_bytes!("./assets/01 TempleOS Hymn Risen (Remix).ogg");
const WAVPACK_DATA: &[u8] = include_bytes!("./assets/01 TempleOS Hymn Risen (Remix).wv");

fn benchmark_read_files(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_reading");
    
    group.bench_function("aac", |b| {
        b.iter(|| {
            black_box(Probe::new(Cursor::new(AAC_DATA))
                .options(ParseOptions::new())
                .guess_file_type()
                .unwrap()
                .read()
                .unwrap())
        })
    });
    
    group.bench_function("aiff", |b| {
        b.iter(|| {
            black_box(Probe::new(Cursor::new(AIFF_DATA))
                .options(ParseOptions::new())
                .guess_file_type()
                .unwrap()
                .read()
                .unwrap())
        })
    });
    
    group.bench_function("ape", |b| {
        b.iter(|| {
            black_box(Probe::new(Cursor::new(APE_DATA))
                .options(ParseOptions::new())
                .guess_file_type()
                .unwrap()
                .read()
                .unwrap())
        })
    });
    
    group.bench_function("flac", |b| {
        b.iter(|| {
            black_box(Probe::new(Cursor::new(FLAC_DATA))
                .options(ParseOptions::new())
                .guess_file_type()
                .unwrap()
                .read()
                .unwrap())
        })
    });
    
    group.bench_function("mp4", |b| {
        b.iter(|| {
            black_box(Probe::new(Cursor::new(MP4_DATA))
                .options(ParseOptions::new())
                .guess_file_type()
                .unwrap()
                .read()
                .unwrap())
        })
    });
    
    group.bench_function("mp3", |b| {
        b.iter(|| {
            black_box(Probe::new(Cursor::new(MP3_DATA))
                .options(ParseOptions::new())
                .guess_file_type()
                .unwrap()
                .read()
                .unwrap())
        })
    });
    
    group.bench_function("mpc", |b| {
        b.iter(|| {
            black_box(Probe::new(Cursor::new(MPC_DATA))
                .options(ParseOptions::new())
                .guess_file_type()
                .unwrap()
                .read()
                .unwrap())
        })
    });
    
    group.bench_function("opus", |b| {
        b.iter(|| {
            black_box(Probe::new(Cursor::new(OPUS_DATA))
                .options(ParseOptions::new())
                .guess_file_type()
                .unwrap()
                .read()
                .unwrap())
        })
    });
    
    group.bench_function("riff", |b| {
        b.iter(|| {
            black_box(Probe::new(Cursor::new(RIFF_DATA))
                .options(ParseOptions::new())
                .guess_file_type()
                .unwrap()
                .read()
                .unwrap())
        })
    });
    
    group.bench_function("speex", |b| {
        b.iter(|| {
            black_box(Probe::new(Cursor::new(SPEEX_DATA))
                .options(ParseOptions::new())
                .guess_file_type()
                .unwrap()
                .read()
                .unwrap())
        })
    });
    
    group.bench_function("vorbis", |b| {
        b.iter(|| {
            black_box(Probe::new(Cursor::new(VORBIS_DATA))
                .options(ParseOptions::new())
                .guess_file_type()
                .unwrap()
                .read()
                .unwrap())
        })
    });
    
    group.bench_function("wavpack", |b| {
        b.iter(|| {
            black_box(Probe::new(Cursor::new(WAVPACK_DATA))
                .options(ParseOptions::new())
                .guess_file_type()
                .unwrap()
                .read()
                .unwrap())
        })
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_read_files);
criterion_main!(benches);