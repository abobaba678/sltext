# sltext — Still Life Text

**Transform ordinary text into distorted typography.**

`sltext` (Still Life Text) is a lightweight command-line utility written in Rust that transforms text by randomly replacing selected characters with visually similar, distorted Unicode characters.

The idea is simple: preserve the original structure of a text while giving individual letters a slightly corrupted appearance.

## ✨ Features

- **Random character transformation** — randomly selects characters to transform.
- **Custom dictionary** — define your own character transformations.
- **Adjustable intensity** — control the percentage of characters affected.
- **Unicode support** — works with Unicode characters, including Cyrillic.
- **Lightweight** — built with Rust and minimal dependencies.

## 📸 Example

```text
Input:
Still Life Text

Output:
Sȶill Lɨfe Tɇxt
```

*Output is illustrative. Actual transformations depend on your dictionary and random selection.*

## ⚙️ Installation

### Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo

### Build from source

Clone the repository:

```bash
git clone https://github.com/YOUR_USERNAME/sltext.git
cd sltext
```

Build the project:

```bash
cargo build --release
```

The compiled binary will be located at:

```text
target/release/sltext
```

## 🚀 Usage

```bash
sltext "<text>" <percentage> <dictionary>
```

### Arguments

| Argument | Description |
|---|---|
| `text` | Text to transform |
| `percentage` | Percentage of characters to select (0–100) |
| `dictionary` | Path to the transformation dictionary |

### Examples

Transform 50% of the characters:

```bash
sltext "Still Life Text" 50 from-to.txt
```

Transform all characters:

```bash
sltext "Hello World" 100 from-to.txt
```

Transform 10% of the characters:

```bash
sltext "Rust is awesome" 10 from-to.txt
```

Rust run:
```bash
cargo run "Hello, world!" 100 from-to.txt
```


## 📖 Dictionary Format

`sltext` uses a simple text-based dictionary to define character transformations.

Each line contains two characters:

```text
aȺ
bɃ
cȻ
dĐ
eɆ
fƑ
```

The first character represents the original letter, while the second represents its transformed version.

For example:

```text
aȺ
eɆ
oØ
sƧ
tŦ
```

When `sltext` encounters a selected character, it searches the dictionary for a matching original character and replaces it with the corresponding transformed character.

You can create your own dictionaries for different visual styles and alphabets.

## 🦀 Built With

- **[Rust](https://www.rust-lang.org/)** — main programming language.
- **[rand](https://crates.io/crates/rand)** — random character selection.

## 🛠️ How It Works

1. Reads the input text and converts it into a vector of Unicode characters.
2. Loads transformation rules from the specified dictionary.
3. Calculates how many characters should be selected based on the requested percentage.
4. Randomly selects unique character positions.
5. Searches for corresponding transformations in the dictionary.
6. Reconstructs the resulting text and prints it to the terminal.

## 📄 License

This project is open-source. 

GPL 3.0 license

---

<p align="center">
  <b>sltext — Give your text a different shape.</b>
</p>