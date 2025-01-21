## Clone the Repository with Submodule

```bash
git clone --recurse-submodules <your-repo-url>
```

If the repository is already cloned, initialize the submodule manually:

```bash
git submodule update --init --recursive
```

## Update the Submodule

When Mergiraf updates, you need to update the submodule in your repository:

1. Fetch Updates:
```bash
cd mergiraf
git pull origin main
```
2. Commit the Updated Submodule:
```bash
cd ..
git add mergiraf
git commit -m "Updated Mergiraf submodule"
```

## Edit it to support USFM

1. Add tree-sitter-usfm3 to dependencies in `Cargo.toml`.
2. Add language details to `src/supported_langs.rs`
	```rust
	        LangProfile {
            name: "USFM3", // only used for logging purposes so far
            extensions: vec![".usfm", ".sfm"], // all file extensions for this language
            language: tree_sitter_usfm3::language().into(), // the tree-sitter parser
            // optional settings, explained below
            atomic_nodes: vec![],
            commutative_parents: vec![],
            signatures: vec![],
        },
    ```