# :candle:feu

A minimal application launcher written in Rust.  
Currently works on Linux.

![sample](https://github.com/kyoheiu/feu/blob/develop/screenshot/sample.png)

## Installation

```
git clone https://github.com/kyoheiu/feu.git
cd feu
cargo install --path .
```

This app is designed to be used with a tiling window manager such as i3, so after the installation, configure as follows, for example:

```
# ~/.config/i3/config

bindsym $mod+d exec ~/.cargo/bin/feu
```

Then you can run feu with `$mod+d`.

## Usage

| key     | action                    |
| ------- | ------------------------- |
| Up/Down | Move cursor.              |
| Enter   | Launch the selected app and exit.  |
| Esc     | Exit.                     |
| \_      | Filter the list.          |

For example, with a list like the one in the image above, you can start `firefox` simply by pressing the Enter key. You can also start `code` by typing 'c' and pressing the Enter key.

## Binary list

feu reads $PATH and `~/.config/feu/config.json`, and lists all binaries there. If the config file looks like this:

```
{
  "paths": ["/home/kyohei/.cargo/bin/", "home/kyohei/.test/bin/"]
}
```

Then feu lists binaries in $PATH, `/home/kyohei/.cargo/bin` and `/home/kyohei/.test/bin`. If the config file doesn't exist, only $PATH will be read (and I think that's enough).

_Currently, `~` is not allowed in the config file, so you have to write like `/home/user/...`._

At the launch, feu sorts the binary list by the number of execution, so the top of the list should be the app you've called the most.  
The execution history will be automatically saved in `~/.config/feu/.history` (which is also JSON format).

```
# ~/.config/feu/.history

{"history_map":{"code":2,"firefox":3}}
```

If you want to reset it, just delete the `.history` file and everything will be new.
