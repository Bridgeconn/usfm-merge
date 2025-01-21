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