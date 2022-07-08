# VSCode

## Settings

I keep having issues with the language server since I'm using multiple targets and specific configs. So basically you should just create a `.vscode/settings.json` with:

```json
{
    "rust-analyzer.cargo.target": "aarch64-unknown-none",
}
```

since the main arch we're writing for is arm64.
