<div align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/spejamchr/chbsrs/main/preview-dark.png">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/spejamchr/chbsrs/main/preview-light.png">
        <img width="40%" alt="Representing 123.45 as a base-pi number: 10220.00012120..." src="https://raw.githubusercontent.com/spejamchr/chbsrs/main/preview-light.png">
    </picture>
</div>

# ChangeBase

Convert numbers from one [positional notation](https://en.wikipedia.org/wiki/Positional_notation) to
another. Either base can be a
[non-integer](https://en.wikipedia.org/wiki/Non-integer_base_of_numeration), such as `pi` or `e`.

Built with [Leptos](https://github.com/leptos-rs/leptos).

## Why?

I read that [base-`e`](https://en.wikipedia.org/wiki/Non-integer_base_of_numeration#Base_e) has the
lowest [radix economy](https://en.wikipedia.org/wiki/Radix_economy#e_has_the_lowest_radix_economy),
and I wanted to see what numbers would look like in base-`e`. Also, I wanted to try out Leptos.

## Running Locally

This site uses Rust `nightly` and requires that you've installed the `wasm` compilation target for
your toolchain. See [getting started with
Leptos](https://book.leptos.dev/getting_started/index.html).

If you don't have Rust nightly, you can install it with

```sh
rustup toolchain install nightly --allow-downgrade
```

You can add the `wasm` compilation target to rust using

```sh
rustup target add wasm32-unknown-unknown
```

Then clone the project

```sh
git clone https://github.com/spejamchr/chbsrs.git
```

## Developing

To run the app use [Trunk](https://github.com/trunk-rs/trunk):

```sh
trunk serve --port 3000 --open
```

## Deploying

All pushes to `main` are automatically deployed by GitHub Actions.
