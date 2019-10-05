# html-to-seed
Tool for converting Html to [Rust](https://www.rust-lang.org/) for use with the [Seed](https://seed-rs.org/) web framework
Seed can be used to build web applications in WebAssembly inspired by [Elm](https://elm-lang.org/)).

Use html-to-seed to:
- converting an existing HTML UI to a [Seed](https://seed-rs.org/) based application.
- check your UI layout in Html before committing to building an application, then generate the view code with little to no effort.

Try it [here](https://github.com).

---

## Main parts:

- **html-to-seed-web** - The tool as a webassembly [SPA (single page application)](https://en.wikipedia.org/wiki/Single-page_application).
- **html-to-seed-bin** - Command line version of the tool.
- **html-to-seed-lib** - The library that does the actual conversions.

## Acknowledgements

- **html-to-seed** is inspired by [html-to-elmish](https://mangelmaxime.github.io/html-to-elmish/) for [F#/Fable](https://fable.io/) by **Maxime Mangel**.
- Some Html samples were borrowed from [html-to-elmish](https://mangelmaxime.github.io/html-to-elmish/).
- This project was kick started using **Martin Kavik's** [Seed Quickstart with Webpack](https://github.com/MartinKavik/seed-quickstart-webpack).
- Icons from [Zondicons](http://www.zondicons.com/) by **Steve Schoger**.

## To build

1. You need to install [Rust](https://rust-lang.org/tools/install), [NodeJs](https://nodejs.org/en/download/) and [Yarn](https://yarnpkg.com/lang/en/docs/install).
1. These tools are required by some commands:

   - [cargo-make](https://sagiegurari.github.io/cargo-make/)

     - Check: `$ cargo make -V` => `cargo-make 0.22.1`
     - Install: `$ cargo install --force cargo-make`

   - [nightly rustfmt](https://github.com/rust-lang/rustfmt#on-the-nightly-toolchain)
     - Check: `$ cargo +nightly fmt -- -V` => `rustfmt 1.4.8-nightly (afb1ee1c 2019-09-08)`
     - Install:
       1. `$ rustup toolchain install nightly`
       2. `$ rustup component add rustfmt --toolchain nightly`
1. Clone or download this repo.
1. Run command `yarn` in the project root.


## Commands

- **`yarn build`** and **`yarn build:release`**

  - Bundle app and save it into `/dist`.

* **`yarn start`**

  - Build project and start developer server on `127.0.0.1:8000`.
  - Server auto-reloads browser tabs on changes.
  - It doesn't destroy state if I change only styles (hot reload).

* **`yarn test`**

  - Run tests in NodeJs.

* **`yarn test:{browser}`**

  - Run tests in headless browser.
  - `{browser}` should be `firefox`, `chrome` or `safari`.


##  Build Pipeline

  - Remove previous build from `/dist/`.
  - Generate styles from `/css/styles.css` (see `/configs/postcss.config.js`).
  - Generate `/html-to-seed-web/src/generated/css_classes.rs` from styles.
  - _[only release]_ Filter out unused CSS classes from styles.
  - Process styles with [autoprefixer](https://github.com/postcss/autoprefixer).
  - Compile Rust:
    - `/html-to-seeed-web/` will be built into `/html-to-seeed-web/pkg/`.
  - Bundle assets (css, images, ts, js...) and _[only release]_ uglify/minify them. (See `/configs/webpack.config.js` and `/configs/tsconfig.json`)
  - Compile template `/entries/index.html` (i.e. Add bundle link into template and save result into `/dist/`).
  - Copy files from `/static/` into `/dist/static/`.
  - _[only release]_ Optimize `/dist/[name].wasm` for size (see `/scripts/optimize_wasm.js`).


# Browser and platform support

Should work with most modern browsers.
Guaranteed _not_ to work in IE.


# Contributing

- create Issue or PR.
- Ideas, bugs, questions, ... - create Issue.

_Note_: Please squash commits and rebase before creating PR. Thanks!

