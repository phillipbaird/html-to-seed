# html-to-seed
![Build status](https://github.com/phillipbaird/html-to-seed/actions/workflows/build.yml/badge.svg) [![Netlify Status](https://api.netlify.com/api/v1/badges/2c65e870-70db-4ce9-a562-614589831a75/deploy-status)](https://app.netlify.com/sites/html-to-seed/deploys)

Tool for converting Html to [Rust](https://www.rust-lang.org/) for use with the [Seed](https://seed-rs.org/) web framework
Seed can be used to build web applications in WebAssembly inspired by [Elm](https://elm-lang.org/).

Use html-to-seed to:
- converting an existing HTML UI to a [Seed](https://seed-rs.org/) based application.
- check your UI layout in Html before committing to building an application, then generate the view code with little to no effort.

Try it [here](https://html-to-seed.netlify.app).

---

## Contents:

- **html-to-seed-web** - The tool as a webassembly [SPA (single page application)](https://en.wikipedia.org/wiki/Single-page_application).
- **html-to-seed-bin** - Command line version of the tool.
- **html-to-seed-lib** - The library that does the actual conversions.

## Acknowledgements

- **html-to-seed** is inspired by [html-to-elmish](https://mangelmaxime.github.io/html-to-elmish/) for [F#/Fable](https://fable.io/) by **Maxime Mangel**.
- Some Html samples were borrowed from [html-to-elmish](https://mangelmaxime.github.io/html-to-elmish/).
- Icons from [Zondicons](http://www.zondicons.com/) by **Steve Schoger**.

## To build

1. You need to install [Rust](https://rust-lang.org/tools/install), [NodeJs](https://nodejs.org/en/download/) and NPM.
2. Ensure you have WASM support enabled using
   - `$ rustup target add wasm32-unknown-unknown`
4. Trunkrs is is required to build the SPA:
   - [trunkrs](https://trunkrs.dev/) install: `$ cargo install trunkrs`
5. Clone or download this repo.
6. Install TailwindCSS using `cd html-to-seed-web` & `npm ci`

## Commands

- **`cargo build`** and **`cargo build --release`**

  - Build the library and cli tool and output to `/target`.

* **`cargo test`**
  - Run the unit tests for the library.

__From within the html-to-seed-web directory__

- **`trunk serve`**
  - Build project and start developer server on `127.0.0.1:8080`.
  - Server auto-reloads browser on changes.

* **`trunk build --release`**

  - Build project and output artifacts ready to deploy to `../dist` directory.


# Browser and platform support

Should work with most modern browsers.
Guaranteed _not_ to work in IE.


# Contributing

- create Issue or PR.
- Ideas, bugs, questions, ... - create Issue.

_Note_: Please squash commits and rebase before creating PR. Thanks!

