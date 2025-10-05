# Benchmarking Guide

This project provides two complementary benchmarking suites to accommodate different platforms and use cases.

## Overview

| Benchmark Suite | Platforms | Precision | Use Case |
|-----------------|-----------|-----------|----------|
| iai-callgrind | Linux only | Very High (instruction-level) | Performance regression detection, CI/CD |
| Criterion | Windows/macOS/Linux | High (statistical) | Local development, cross-platform analysis |

## Quick Start

### Run all benchmarks (recommended)
```bash
cargo bench
```

### Platform-specific commands

#### Linux users (high-precision)
```bash
# iai-callgrind benchmarks (requires Valgrind)
cargo bench --bench read_file --bench create_tag
```

#### Windows/macOS users (or Linux users wanting statistical analysis)
```bash
# Criterion benchmarks (cross-platform)
cargo bench --bench read_file_criterion --bench create_tag_criterion
```

## Benchmark Coverage

Both suites provide comprehensive coverage:

### File Reading Benchmarks
Tests parsing performance across all supported audio formats:
- **AAC** (.aac)
- **AIFF** (.aiff, .aif) 
- **APE** (.ape)
- **FLAC** (.flac)
- **MP4** (.mp4, .m4a)
- **MP3** (.mp3)
- **MPC** (.mpc)
- **OPUS** (.opus)
- **RIFF** (.wav)
- **SPEEX** (.spx)
- **VORBIS** (.ogg)
- **WAVPACK** (.wv)

### Tag Creation Benchmarks  
Tests metadata writing performance for all supported tag formats:
- **AIFF Text Chunks**
- **APE Tags**
- **ID3v1**
- **ID3v2** (with picture embedding)
- **MP4 Ilst**
- **RIFF Info**
- **Vorbis Comments**

## Output and Reports

### iai-callgrind (Linux)
- Instruction counts and cycle estimations
- Cachegrind profiles for detailed analysis
- Regression detection with precise measurements
- Output in terminal with comparison to previous runs

### Criterion (Cross-platform)
- Statistical analysis with confidence intervals
- HTML reports in `target/criterion/`
- Performance plots and regression analysis  
- Detailed timing histograms

## Integration with Development Workflow

### Local Development
- **Windows/macOS**: Use Criterion benchmarks for performance testing
- **Linux**: Choose between iai-callgrind (precision) or Criterion (statistics)

### CI/CD
- **Linux CI**: Use iai-callgrind for regression detection
- **Cross-platform CI**: Use Criterion for broader platform coverage
- Both can run in parallel for comprehensive coverage

## Troubleshooting

### iai-callgrind issues
- **Error**: "valgrind: command not found"
  - **Solution**: Install Valgrind (`sudo apt install valgrind` on Ubuntu)
- **Platform**: Only works on Linux/Unix systems

### Criterion issues  
- **Slow performance**: First run includes compilation time
- **Missing reports**: Check `target/criterion/` directory
- **Platform**: Works on all platforms

## Adding New Benchmarks

When adding new functionality, update both benchmark suites:

1. **Add to iai-callgrind**: Update `benches/read_file.rs` or `benches/create_tag.rs`
2. **Add to Criterion**: Update `benches/read_file_criterion.rs` or `benches/create_tag_criterion.rs`

This ensures all developers can benchmark regardless of their platform.