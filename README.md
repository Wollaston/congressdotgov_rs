# congressdotgov_rs

Rust bindings to the [congress.gov](https://api.congress.gov/) REST API. Inspired by [Designing Rust bindings for REST APIs](https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is).

These are unofficial bindings and are not affiliated with congress.gov

## Example

```rust,no_run
use serde::Deserialize;
use congressdotgov_rs::Cdg;
use congressdotgov_rs::api::Query;
use congressdotgov_rs::api::bill;
use congressdotgov_rs::api::common::Format;
use congressdotgov_rs::Auth;
use tokio_test::block_on;

// The return type of a `Bill`. Note that a Bill may contain more information, but you can
// define your structure to only deserialize what is needed as the return value is a
// serde_json::Value.
#[derive(Debug, Deserialize)]
struct Bills {
    bills: Vec<Bill>,
}

#[derive(Debug, Deserialize)]
struct Bill{
    title: String,
    number: String,
}

// Create the client.
let auth = Auth::Token("API_KEY".into());
let req_client = reqwest::Client::new();
let client = Cdg::new(auth, req_client, Format::Json).unwrap();

// Create a simple endpoint. This one gets recent Bills from the 118th Congress.
let endpoint = bill::Congress::builder().congress(118_u8).build().unwrap();

// Call the endpoint. The return type decides how to represent the value.
# tokio_test::block_on(async {
    let bills: Bills = endpoint.query(&client).await.unwrap();
# })
```

## Coverage

All resources, endpoints, and their respective query parameters are covered by these bindings. Many parameters are defined by Rust types, and the library strives to be idiomatic.

These resources are:

- bill
- amendments
- summaries
- congress
- member
- committee
- committee-report
- committee-print
- committee-meeting
- hearing
- congressional-record
- daily-congressional-record
- bound-congressional-record
- house-communication
- house-requirement
- senate-communication
- nomination
- crsreport
- treaty

### Motivation

These bindings were created to make it easier to query the congress.gov API from Rust web servers for use in NLP and related tasks. I hope others may also find them useful.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in congressdotgov_rs by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
