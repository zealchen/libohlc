
# libohlc

A Rust library crate that provides a way of computing rolling OHLC (open-high-low-close) for a stream of numeric 
prices and timestamps for a given time window.

E.g., if the window is 5 minutes, once a new price/timestamp is given, return the rolling 5-minute OHLC over the last 
5 minutes - the earliest price in the time period, the highest/lowest prices and the latest price.

## Usage

In this crate, we have two solutions implemented in mod ohlc_maker.
One is single thread solution ohlc_maker::OHLCMaker::make().
```sh
pub fn make(&self, 
    tick_path: String, // input dataset path
    window_length: u64, // input window length in ms
    ohlc_path: String // output ohlc path
)
```

And the other is multi-thread solution ohlc_maker::OHLCMaker::parallel_make().
```sh
pub fn parallel_make(&self, 
    tick_path: String, // input dataset path
    window_length: u64, // input window length in ms
    ohlc_path: String // output ohlc path
)
```
The thread count is set to the local machine cpu count.


demo:
```sh
crate libohlc;
use libohlc::ohlc_maker::OHLCMaker;

fn main() {
    let maker = OHLCMaker::new();
    let tick_path = "../data/dataset-b.txt";
    let ohlc_path = "predictions.txt";
    let window = 60 * 5 * 1000; // 5m
    maker.parallel_make(tick_path.to_string(), window, ohlc_path.to_string());
    println!("ohlc is dump to {}", ohlc_path)
}
```

test:
```sh
cargo test
```

benchmark:
```sh
cargo bench
```

We can switch criterion_benchmark and criterion_benchmark_parallel to see the performance diff. The multi-thread one is awesome.