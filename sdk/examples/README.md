# T-Rust Benchmark Runner

A Python script to automate benchmarking of zkVM applications across different targets.

## Prerequisites

- Python 3.6+
- T-Rust SDK installed
- Access to the target backends (risc0, sp1, etc.)

## Configuration

Create a `bench_config.json` file in your working directory:

```json
{
    "app_name": {
        "input": "value",
        "input_type": "uint32|bytearray|file",
        "targets": ["risc0", "sp1"]
    }
}
```

Example configuration:
```json
{
    "fibonacci": {
        "input": 10,
        "input_type": "uint32",
        "targets": ["risc0", "sp1"]
    },
    "hash": {
        "input": "0x1234",
        "input_type": "bytearray",
        "targets": ["risc0"]
    }
}
```

## Usage

1. Add your application
2. Create the configuration file
3. Run the benchmark:

```bash
python run_benchmark.py
```

## Output

The script generates a CSV file named `benchmark_result.csv` with the following format:

```csv
app,target,time_ns
fibonacci,risc0,1234567
fibonacci,sp1,2345678
```

## Directory Structure

```
.
├── run_benchmark.py
├── bench_config.json
├── benchmark_result.csv
├── app1/
│   └── ...
└── app2/
    └── ...
```

## Example

1. Create configuration:
```json
{
    "fibonacci": {
        "input": 10,
        "input_type": "uint32",
        "targets": ["risc0", "sp1"]
    }
}
```

2. Run benchmark:
```bash
python run_benchmark.py
```

3. Check results in `benchmark_result.csv`