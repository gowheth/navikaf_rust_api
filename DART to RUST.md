```protobuf
syntax = "proto3";

[RINF:DART-SIGNAL]
message RustMethodChannel {
  int32 id = 1;
  string method = 2;
  string data = 3;
}

[RINF:RUST-SIGNAL]
message RustMethodReciever {
  int32 id = 1;
  string response = 2;
}
```

Rust File for Communication,

import serde for Json parsing in cargo : serde_json = "1.0.117"

```rust
use serde_json::{json, Value};

use crate::messages::rust_channel::{RustMethodChannel, RustMethodReciever};

pub fn setup_rust_method_channel_handler() {
    tokio::spawn(async move {
        let mut receiver = RustMethodChannel::get_dart_signal_receiver();
        while let Some(dart_signal) = receiver.recv().await {
            let message: RustMethodChannel = dart_signal.message;
            let mut data: Option<Value> = None;
            if message.data != "" {
                data = serde_json::from_str(&message.data).expect("Failed to parse JSON");
            }
            handle(message.id, message.method, data);
        }
    });
}

fn handle(id: i32, method: String, data: Option<Value>) {
    let mut data: Value = json!({
"error":"not_implemented"
});

    if method == "test" {
        data = json!({
            "result": "success",
        });
    }

    // send result back to flutter
    RustMethodReciever { id, response: data.to_string() }.send_signal_to_dart();
}

```

Setup Dart side

```dart
class RustChannel {
  static int _methodId = 0;

  static final Stream<RustMethodReciever> _rustMethodStream =
  RustMethodReciever.rustSignalStream.map((event) => event.message);

  static Future<Map<String, dynamic>?> call(String method, {
    Map<String, dynamic>? data,
    Duration? timeout,
  }) async {
    _methodId++;

    RustMethodChannel(
      id: _methodId,
      method: method,
      data: data != null ? json.encode(data) : "",
    ).sendSignalToRust();

    RustMethodReciever reciever = await _rustMethodStream
        .firstWhere((e) => e.id == _methodId)
        .timeout(timeout ?? const Duration(seconds: 2));

    var response = json.decode(reciever.response);
    String? error = response["error"];

    if (error != null) throw error;
    return response;
  }
}
```

Now we just need to call RustChannel.init(); once, and communicate with Rust and get back response using

```dart

var response = RustChannel.call("test")
```
