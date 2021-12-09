# :candle:feu
A minimal application launcher written in Rust(iced).

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
| Up/Down | moves cursor.|
| Enter | launches selected app.|
| Esc | exits program.|
| _ | filters the list.|


feu accepts text-input and Up/Down/Enter/Esc keys simultaneously: You can enter words to filter the list and move the cursor without using the mouse.
In other words, if you have a list shown in the one above, you can launch `firefox` just by pressing the Enter key. Or type 'c' and press Enter to launch `code`.

## Binary list
Currently feu reads `/usr/bin` directory and lists all binaries in it. And, remembering the number of execution, it sorts the binary list by the number, so the top of the list should be the app you use the most.
The number of execution is automatically saved as `HashMap<(String, usize)>` in `~/.config/feu/.history` (RON format). 

```
# ~/.config/feu/.history

(history_map:{"code":2,"firefox":3})
```

If you want to reset it, just delete `.history` and everything will be new. 