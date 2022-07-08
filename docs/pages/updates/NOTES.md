# Notes

## Minimal Config

Since this is a multi target kind of thing in rust, we get a whole bunch of issues if we try to do it the standard way. Recommended to disable any language servers since they can spasm really hard. Maybe theres a way to configure it nicely but Idk I dont really wanna to configure VSCode too much.

- This means things like `.cargo/config.toml` should be very minimal. Mostly for cool things like aliases and stuff. Dont specify any main configs. You can do `[dependencies.X]` for X if you want but I rather leave it mostly vanila and rely on `arcboot` and `arcutils` for more complex config and functionality
- Mostly using rust, the language itself and the cargo package management and test suite. I dont really care about the other stuff, at least for now.
