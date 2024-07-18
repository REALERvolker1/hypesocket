# WIP Hyprland IPC bindings

I kind of got bored of waiting for hyprland-rs to implement all the newer Hyprland features, so I made this library

It is a lower-level wrapper around the sockets. In fact, it is entirely possible to initialize a listener with a custom path, and parse any/all output data yourself.

Convenience functions are provided with the json_commands feature. However, they could easily become outdated. Be mindful of this.

It is compatible with tokio (tokio feature) and async-net/futures-lite (async-lite feature). The default configuration uses just the std library alone.

This is very WIP, so expect some bugs. **Do your duty as a beta tester and report them!!!**

