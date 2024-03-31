<div align="center">

# @dataconstruct/data-faking

Generate massive amounts of fake (but realistic) data for testing and development.

[![Docs Status](https://docs.rs/data-faking/badge.svg)](https://docs.rs/data-faking)
[![Latest Version](https://img.shields.io/crates/v/data-faking.svg)](https://crates.io/crates/data-faking)
[![npm version](https://badgen.net/npm/v/data-faking)](https://www.npmjs.com/package/data-faking)

Try using our [playground](https://www.dataconstruct.io/organizations/playground/schemas) for your data gen needs, it supports code gen for much more than rust and javascript.

[Docs (WIP)](https://data-construct.github.io/faking)
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
  println!("{}", faking::defaults::types::f64());
}
```

## Usage Javascript / Typescript

```bash
npm i --save-dev data-faking
```

```typescript
import * as faking from "data-faking";

console.log(faking.f64());
```

## Randomness seed

If you want consistent results, you can set your own seed:

```rust
faking::utils::seeder::set_seed(2);
println!("{}", faking::defaults::types::f64());
```

```typescript
faking.set_seed(BigInt(2));
console.log(faking.f64());
```

### Unsupported Seeded Generation
The following data generators do not support seeded generation:

- DateTime: Naive Date ([Before Today](src/data/datetime/date_naive.rs#L57) | [After Today](src/data/datetime/date_naive.rs#L83)).
