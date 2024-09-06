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
t-rust compile local [--verbose]
t-rust run local [--verbose]
```

The supported targets are:
- [x] local (no prover)
- [ ] SP1 (zkVM)
- [ ] RiscZero (zkVM)
