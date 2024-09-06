# tRUST SDK

You can install and use tRust as follows:

```
git clone https://github.com/crosschain-alliance/t-rust.git
cd t-rust/
./install.sh
source ~/.bashrc (or ~/.zshrc for macOS)

cd <your-project>
t-rust compile local [--verbose]
t-rust run local [--verbose]
```

The supported targets are:
- [x] local (no prover)
- [ ] SP1 (zkVM)
- [ ] RiscZero (zkVM)
