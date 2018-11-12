# reqwest-pretty-json

`reqwest` provides an easy way to send a JSON formatted `HTTP body` of the request.
However, it gives you zero control of how the serialization is done.
Whatever `serde_json::to_vec()` emits [single condensed string] is going to be used.
In vast majority of the case it is good and does the job very well.
However, sometimes you may want to have it "prettified".
For example, when talking to KV stores (that just keep the bytes you've sent them and do not interpret it in any way)
it may be desirable to have you JSON text more human-readable.
There is no way to do it directly with `reqwest`.
Of course you can always manually serialize the data structure into anything you want
(pretty JSON) and manually set it as request body as well as adding `Content-Type: application/json` header.
This, however, is less nice than the just letting `reqwest::RequestBuilder` do the right thing.

Exactly for cases like that this crate augments `reqwest::RequestBuilder` with `pretty_json()` method.

```rust
use reqwest::Client;
use reqwest_pretty_json::PrettyJson;

let data = vec![1, 2, 3];
let client = Client::new();
client
    .post("http://httpbin.org/post")
    .pretty_json(&data)
    .send()
    .unwrap();
```

Under the hood it uses `serde_json::to_vec_pretty()` to serialize the data.
