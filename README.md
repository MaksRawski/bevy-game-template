# Bevy game template

# Usage
`cargo generate --git https://github.com/MaksRawski/bevy-game-template.git --subfolder template --name my-game`

This could be simplified by adding:
```
[favorites.bevy]
description = "Generate a template for a 2d bevy game with wasm suppport"
git = "https://gitlab.com/MaksRawski/bevy-game-template.git"
subfolder = "template"
```
to `$CARGO_HOME/cargo-generate.toml`.
Which allows to just `cargo generate bevy --name my-game`.
