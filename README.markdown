# TIME OXIDE :gear: :hourglass:

![GitHub CI](https://github.com/angeldollface/time-oxide/actions/workflows/yew.yml/badge.svg)

***A small web clock written in Rust. :gear: :hourglass:***

## ABOUT :books:

Some time ago I wrote [this project](https://github.com/angeldollface/dolly-clock) in React.js. Because I like aesthetic clocks, I thought I'd re-implement this clock in Rust using Yew. This repository contains the source code for that re-implementation.

## DEPLOYED PROJECT ON GITHUB PAGES :rocket:

To view a live deployed version of this project, click here: [VIEW](https://angeldollface.art/time-oxide)

## USAGE :hammer:

- 1.) Visit the link above.
- 2.) Enjoy.

## TRY THE CODE FOR YOURSELF! :inbox_tray:

You should have the following tools installed and available from the command line:

- Rust
- Git

To try *Time Oxide* on your own machine, follow these steps:

- 1.) Install `trunk` from [crates.io](https://crates.io/crates/trunk):

```bash
cargo install trunk
```

- 2.) Clone this project's source code:

```bash
git clone https://github.com/angeldollface/time-oxide
```

- 3.) Change directory into the source code's root directory:

```bash
cd time-oxide
```

- 4.) Serve the app locally (This will serve the app locally on [`http://127.0.0.1:8080/time-oxide/`](http://127.0.0.1:8080/time-oxide/).):

```bash
trunk --config trunk.toml serve --release
```

- 5.) If you want to build the app into a bundle to deploy to a server, run the command below. This will produce a directory called `dist` with the bundle inside it.

```bash
trunk --config trunk.toml build --release
```

- 5.) Enjoy! :heart_on_fire:


## CHANGELOG :black_nib:

### Version 1.0.0

- Initial release.
- Upload to GitHub.
- Deployment to GitHub Pages.

## NOTE :scroll:

- *Time Oxide :gear: :hourglass:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.