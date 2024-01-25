# websocket_ping_pong

Test WebSockets latency on localhost.

## Usage

```
# First terminal
cargo run --release --bin server

# Second terminal
cargo run --release --bin client
```

## Example

TLDR: 20-30µs round-trip to ping/pong on a 12th Gen i7.

My machine:

```
$ lshw -short
...
/0/0                            memory         66GiB System memory
/0/1                            processor      12th Gen Intel(R) Core(TM) i7-1280P
...
```

My results:

```
Round-trip time avg: 23.656µs over 100 trials
Connection closed
Round-trip time avg: 16.384µs over 100 trials
Connection closed
Round-trip time avg: 28.854µs over 100 trials
Connection closed
Round-trip time avg: 18.674µs over 100 trials
Connection closed
Round-trip time avg: 15.705µs over 100 trials
Connection closed
Round-trip time avg: 27.504µs over 100 trials
Connection closed
Round-trip time avg: 21.087µs over 100 trials
Connection closed
Round-trip time avg: 34.202µs over 100 trials
Connection closed
Round-trip time avg: 22.076µs over 100 trials
Connection closed
Round-trip time avg: 30.811µs over 100 trials
Connection closed
Round-trip time avg: 22.59µs over 100 trials
Connection closed
Round-trip time avg: 17.906µs over 100 trials
Connection closed
Round-trip time avg: 33.276µs over 100 trials
Connection closed
```
