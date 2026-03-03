# fahhh

> A minimal, blazingly fast auditory disappointment for failed commands

Makes your shell go "fahhh" on non-zero exit codes.

No logs. No analysis. Just shame.

## 🚀 Installation

### Step 1. Install fahhh

<details>
<summary>Arch</summary>

```sh
yay -S fahhh-git
```

</details>

<details>
<summary>Other Linux</summary>

```sh
cargo install --locked --git https://github.com/arghyadipchak/fahhh.git
```

</details>

### Step 2. Set up your shell to use fahhh

<details>
<summary>Bash</summary>

Add the following to the end of `~/.bashrc`:

```sh
eval "$(fahhh init bash)"
```

</details>

<details>
<summary>Fish</summary>

Add the following to the end of `~/.config/fish/config.fish`:

```fish
fahhh init fish | source
```

</details>

<details>
<summary>Zsh</summary>

Add the following to the end of `~/.zshrc`:

```sh
eval "$(fahhh init zsh)"
```

</details>

## 📝 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
