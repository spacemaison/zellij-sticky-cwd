## About

This is a plugin for the [Zellij][zellij] terminal multiplexer that allows for keeping the current working directory when splitting panes. In order to run this, you need to use my fork of Zellij, as I had to make extensions to the core API's. This repositories Cargo.toml points to the fork as a relative path in an adjacent directory to this one, so make sure you compile Zellij and this plugin from the same root directory. 

## Usage

Once you've built both projects you need to modify your Zellij config.yaml to add this plugin and configure it like so:

```yaml
plugins:
  - path: /absolute/path/to/target/wasm32-wasi/debug/zellij-sticky-cwd.wasm
    tag: sticky-cwd
    run: service

keybinds:
    unbind: [ Ctrl: 'r', ]
    normal:
        - action: [ SwitchToMode: resize, ]
          key: [ Ctrl: 'e', ]
    plugins:
        sticky-cwd:
            pane:
                - action: [ NewPane: Right, ]
                  key: [ Char: 'm', ]
```

Note that you can bind NewPane instructions to be dispatched into the sticky-cwd plugin.

[zellij]: https://github.com/zellij-org/zellij
