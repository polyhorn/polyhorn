<p align="center">
    <img src="assets/emoji-polyhorn.png?raw=true" width="64" />
</p>

<h1 align="center">
    <a href="https://polyhorn.com/">
        Polyhorn
    </a>
</h1>

<p align="center">
    <strong>
        A library for rapidly building cross-platform apps in Rust 🦀.
    </strong>
</p>

<p align="center">
    <a href="https://crates.io/crates/polyhorn">
        <img src="https://img.shields.io/crates/v/polyhorn" />
    </a>
    <a href="https://spectrum.chat/polyhorn">
        <img src="https://withspectrum.github.io/badge/badge.svg" />
    </a>
</p>

__Polyhorn__ is a Rust library for building user interfaces. For users familiar
with React or React Native, this library will be very similar in purpose.

It also comes with its own command line interface that makes it easy to start a
new project and to build and run existing projects.

Colloquially, both are called Polyhorn.

---

🚧 __Warning:__ Polyhorn is still very much a work-in-progress, which means that
there will probably be many breaking changes and missing features before its
official launch (sorry for this).

---

## Get Started

### Installation

Installation of Polyhorn is easy and takes just a few seconds. If you're new to
Rust, you also need to install Rust first with step 1 below. If you're already
using Rust, you can skip step 1 and install Polyhorn with step 2.

1. __If you don't already have Rust installed__, install Rust with rustup.

   ```
   $ curl -sSf https://sh.rustup.rs | sh
   ```

2. Now, install Polyhorn with cargo, the Rust-provided package manager.

   ```
   $ cargo install polyhorn
   ```

### Start a Project

Starting a new project from one of our built-in templates is easy. For an almost
blank project, run:

```
$ polyhorn new hello-world
````

You can replace `hello-world` with the name of your app. Make sure it contains
only alphanumerical characters and dashes.

This will generate a directory with the following structure:

```
hello-world/
├── .gitignore
├── assets
│   └── .gitkeep
└── src
    └── lib.rs
```

This is the content of `src/lib.rs`:

```rust
use polyhorn::prelude::*;

pub struct App {}

impl Component for App {
    fn render(&self, _manager: &mut Manager) -> Element {
        poly!(<Window ...>
            <View style={ style! {
                align-items: center;
                justify-content: center;
                background-color: red;
                height: 100%;
            } } ...>
                <Text style={ style! { color: white; } }>
                    "Welcome to your Polyhorn app!"
                </Text>
            </View>
        </Window>)
    }
}

polyhorn::render!(<App />);
```

### Run a Project

In your command line, navigate to your project. For example, if you followed the
instructions above, you should now be in `hello-world/`. Then, decide on which
platform you want to run. Note that all platforms share the same codebase, so
you don't need to plan ahead.

##### Running on iOS

If you want to run your app on an iOS simulator, run this command:

```
$ polyhorn run ios
```

It will ask you to select an available simulator.
