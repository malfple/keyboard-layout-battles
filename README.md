# Keyboard Layout Battles

Deployment branch for shuttle

# Changes from main

1. main.rs: function name change, runtime changed
2. settings changed, no longer uses settings but uses shuttle's proprietary Secrets.toml file. The settings mod just reads from it.
3. Using postgres instead of mysql

# Deploy

```
cargo install shuttle // to install shuttle in your comp

cd backend
cargo shuttle run // run locally
cargo shuttle deploy
```