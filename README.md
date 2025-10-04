# Kitten Popup Renderer

A not so elegant, but somewhat aesthetical solution to the problem

## The Problem

Linux desktop environments mostly provide complete experience, except those that don't. One of the common lacking pieces are the popups when changing brightness and audio volume.

## The Solution

Since I am dumb and don't know how to render popups, I decided to use [kitty](https://github.com/kovidgoyal/kitty)'s `kitten panel` feature to render a terminal window like a status bar. Which is stupid, but rather in a funny way, so probably usable.

## Installation

Like any other Rust project, Installation is pretty simple:

```sh
cargo install --git https://github.com/rdsq/kitty-popup-renderer
```

### Dependencies

In order to work in runtime, it needs:

- [**kitty**](https://github.com/kovidgoyal/kitty) for the `kitten panel` command
- [**brightnessctl**](https://github.com/Hummer12007/brightnessctl) for getting current brightness level
- [**ALSA**](https://github.com/alsa-project/alsa-lib) for getting current audio volume

## Usage

Call `kitten-popup-renderer brightness` to display the brightness popup, and `kitten-popup-renderer volume` to display the volume popup. You probably want to put those after your brightness/volume changing bindings in your config

[Hyprland](https://hypr.land/) example:

```hl
bindel = ,XF86AudioRaiseVolume, exec, wpctl set-volume -l 1 @DEFAULT_AUDIO_SINK@ 5%+ && ~/.cargo/bin/kitten-popup-renderer volume
bindel = ,XF86AudioLowerVolume, exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%- && ~/.cargo/bin/kitten-popup-renderer volume
bindel = ,XF86AudioMute, exec, wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle && ~/.cargo/bin/kitten-popup-renderer volume
bindel = ,XF86MonBrightnessUp, exec, brightnessctl -e4 -n2 set 5%+ && ~/.cargo/bin/kitten-popup-renderer brightness
bindel = ,XF86MonBrightnessDown, exec, brightnessctl -e4 -n2 set 5%- && ~/.cargo/bin/kitten-popup-renderer brightness
```
