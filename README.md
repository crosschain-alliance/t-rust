# tRUST SDK

📚 [**Read the docs**](https://docs.safejunction.io)  
💬 [**Join our community**](https://docs.safejunction.io/meta/community)  
🛠️ [**Early Access**](https://forms.gle/YKwv47pLjKe3iYbk6)

---

You can install and use tRust as follows:

```
git clone https://github.com/crosschain-alliance/t-rust.git
cd t-rust/
./install.sh
source ~/.bashrc (or ~/.zshrc for macOS)

cd <your-project>

# Run locally
t-rust compile local [--verbose]
t-rust run local [--verbose]

# Run on sp1
t-rust compile sp1 [--verbose]
t-rust run sp1 [--verbose]

# Run on risc0
t-rust compile risc0 [--verbose]
t-rust run risc0 [--verbose]

# Run on jolt
t-rust compile jolt [--verbose]
t-rust run jolt [--verbose]
```

In order to get execution time:
```
# Locally
t-rust benchmark local [--verbose]

# sp1
t-rust benchmark sp1 [--verbose]

# risc0
t-rust benchmark risc0 [--verbose]

# jolt
t-rust benchmark jolt [--verbose]
```


The supported targets are:
- [x] local (no prover)
- [x] SP1 (zkVM)
- [x] RiscZero (zkVM)
- [x] Jolt (zkVM)
