<div align="center">

# @dataconstruct/data-faking

Generate massive amounts of fake (but realistic) data for testing and development.

[![Docs Status](https://docs.rs/data-faking/badge.svg)](https://docs.rs/data-faking)
[![Latest Version](https://img.shields.io/crates/v/data-faking.svg)](https://crates.io/crates/data-faking)

Try using our [playground](https://www.dataconstruct.io/organizations/playground/schemas) for your data gen needs, it supports code gen for much more than rust and javascript.

</div>

## Features
* Defaults data types - numbers, lorem ipsum, bools, uuids
* People - generate names, emails, jobs
* Locations - generate addresses for north america (more coming soon), and coordinates
* Various media - games, show, and books from across the globe
* API data - generate data resembling real apis (ex. stripe)

> Note: We try to generate realistic data. The generated names, addresses, emails, phone numbers, and/or other data might be coincidentally valid information. Please do not send any of your messages / calls to them from your test setup.

## Usage Rust

```bash
cargo add data-faking
```

```rust
use data_faking as faking;

fn main() {
  println!("{}", faking::defaults::uuids::uuid_v4());
}
```

## Usage Javascript / Typescript

```bash
npm i --save-dev data-faking
```

```typescript
import * as faking from "data-faking";

console.log(faking.uuid_v4());
```
