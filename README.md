# :candle:feu
A minimal application launcher written in Rust(iced).  
Currently works on Linux well.

![sample](https://github.com/kyoheiu/feu/blob/develop/screenshot/sample.jpg)

## Installation
```
git clone https://github.com/kyoheiu/feu.git
cd feu
cargo install --path .
```

Since this app is designed to be used with a tiling window manager like i3, the post-installation setting should look like this:

```
# ~/.config/i3/config

bindsym $mod+d exec ~/.cargo/bin/feu
```

Then you can run feu with `$mod+d`.

## Usage

| key | action|
|---|---|
| Up/Down | selects.|
| Enter | launches selected app.|
| Esc | exits program.|
| _ | filters the list.|


feu accepts text-input and Up/Down/Enter/Esc keys simultaneously: You can enter words to filter the list and move the cursor without using the mouse.
In other words, if you have a list shown in the one above, you can launch `firefox` just by pressing the Enter key. Or type 'c' and press Enter to launch `code`.

## Binary list
feu reads `~/.config/feu/config`  (RON format) and lists all binaries in `paths`. For example, if the config file looks like this:

```
Config(
    paths: [
    "/usr/bin",
    "/home/username/.cargo/bin"
    ]
)
```

Then feu lists binaries in `/usr/bin` and `/home/username/.cargo/bin`. If the config file doesn't exist, only `/usr/bin` is read.

*Currently, `~` is not allowed in the config file, so you have to write like `/home/username/...`, which is not smart IMO.*

And, remembering the number of execution, feu sorts the binary list by the number, so the top of the list should be the app you use the most.  
The number of execution is automatically saved as `HashMap<(String, usize)>` in `~/.config/feu/.history` (which is also RON format). 

```
# ~/.config/feu/.history

(history_map:{"code":2,"firefox":3})
```

If you want to reset it, just delete `.history` and everything will be new. 

## Todo
- [x] support other PATHs, such as `~/.cargo/bin`
- [ ] support macOS
