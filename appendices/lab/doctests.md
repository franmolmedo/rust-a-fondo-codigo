# Supplementary appendix examples

## AP-A-B009

```rust
fn remaining(capacity: u32, occupied: u32) -> Option<u32> {
    capacity.checked_sub(occupied)
}

fn main() {
    let capacity = 12;
    let occupied = 5;
    match remaining(capacity, occupied) {
        Some(free) => println!("Free spaces: {free}"),
        None => eprintln!("Occupied spaces exceed capacity"),
    }
}
```

## AP-B-B001

```rust
use std::collections::HashMap;

fn main() {
    let mut totals = HashMap::<&str, u64>::new();
    for (name, quantity) in [("bolt", 2_u64), ("nut", 3), ("bolt", 4)] {
        let total = totals.entry(name).or_insert(0);
        *total = total.checked_add(quantity).expect("small example totals");
    }
    assert_eq!(totals.get("bolt"), Some(&6));
    assert_eq!(totals.get("missing"), None);
}
```

## AP-B-B002

```rust
use std::collections::HashMap;

fn main() {
    let users = HashMap::from([(String::from("ada"), 7)]);
    assert_eq!(users.get("ada"), Some(&7));
}
```

## AP-B-B003

```rust
use std::collections::BTreeMap;

fn main() {
    let totals = BTreeMap::from([("washer", 6), ("bolt", 2), ("nut", 3)]);
    let names: Vec<_> = totals.keys().copied().collect();
    assert_eq!(names, ["bolt", "nut", "washer"]);
    assert_eq!(totals.range("bolt".."washer").count(), 2);
}
```

## AP-B-B004

```rust
use std::collections::HashSet;

fn main() {
    let mut seen = HashSet::new();
    assert!(seen.insert("job-1"));
    assert!(!seen.insert("job-1"));
    assert_eq!(seen.len(), 1);
}
```

## AP-B-B005

```rust
use std::collections::VecDeque;

fn main() {
    let mut pending = VecDeque::from([10, 20]);
    pending.push_back(30);
    assert_eq!(pending.pop_front(), Some(10));
    assert_eq!(pending.into_iter().collect::<Vec<_>>(), [20, 30]);
}
```

## AP-B-B006

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut jobs = BinaryHeap::new();
    jobs.push(Reverse((3_u32, 20_u64)));
    jobs.push(Reverse((1, 30)));
    jobs.push(Reverse((1, 10)));
    assert_eq!(jobs.pop(), Some(Reverse((1, 10))));
    assert_eq!(jobs.pop(), Some(Reverse((1, 30))));
}
```

## AP-B-B007

```rust
use std::borrow::Cow;

fn normalize_label(input: &str) -> Cow<'_, str> {
    let trimmed = input.trim();
    if trimmed.bytes().any(|byte| byte.is_ascii_uppercase()) {
        Cow::Owned(trimmed.to_ascii_lowercase())
    } else {
        Cow::Borrowed(trimmed)
    }
}

fn main() {
    assert!(matches!(normalize_label(" bolt "), Cow::Borrowed("bolt")));
    assert_eq!(normalize_label(" BOLT "), "bolt");
}
```

## AP-B-B008

```rust
use std::ffi::OsStr;
use std::path::Path;

fn main() {
    let path = Path::new("data").join("inventory.csv");
    assert_eq!(path.file_name(), Some(OsStr::new("inventory.csv")));
    assert_eq!(path.extension(), Some(OsStr::new("csv")));
}
```

## AP-B-B009

```rust
use std::io::{Cursor, Read, Seek, SeekFrom};

fn main() -> std::io::Result<()> {
    let mut input = Cursor::new(b"ABCD");
    input.seek(SeekFrom::Start(1))?;
    let mut pair = [0_u8; 2];
    input.read_exact(&mut pair)?;
    assert_eq!(&pair, b"BC");
    Ok(())
}
```

## AP-B-B010

```rust
use std::io::{BufRead, Cursor};

fn main() -> std::io::Result<()> {
    let mut reader = Cursor::new(b"one\n\ntwo\n");
    let mut line = String::new();
    let mut count = 0;
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        if !line.trim().is_empty() {
            count += 1;
        }
    }
    assert_eq!(count, 2);
    Ok(())
}
```

## AP-B-B012

```rust
use rust_appendix_lab::data::{Limits, summarize, write_report};
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Cursor::new(b"bolt,2\nnut,3\nbolt,4\n");
    let report = summarize(input, Limits::default())?;
    let mut output = Vec::new();
    write_report(&mut output, &report)?;
    assert_eq!(output, b"bolt,6\nnut,3\n");
    Ok(())
}
```

## AP-C-B001

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Port(u16);

impl Port {
    const fn new(value: u16) -> Option<Self> {
        if value == 0 { None } else { Some(Self(value)) }
    }
}

const DEFAULT_PORT: Port = match Port::new(8042) {
    Some(port) => port,
    None => panic!("the default port must be nonzero"),
};

let supplied = "8042".parse::<u16>().unwrap();
assert_eq!(Port::new(supplied), Some(DEFAULT_PORT));
assert_eq!(Port::new(0), None);
```

## AP-C-B002

```rust,compile_fail
use rust_appendix_lab::const_eval::Port;

const INVALID_PORT: Port = match Port::new(0) {
    Some(port) => port,
    None => panic!("the default port must be nonzero"),
};

fn main() {
    let _ = INVALID_PORT;
}
```

## AP-C-B003

```rust
const fn squares() -> [u16; 16] {
    let mut table = [0; 16];
    let mut index = 0;
    while index < table.len() {
        table[index] = (index as u16) * (index as u16);
        index += 1;
    }
    table
}

const SQUARES: [u16; 16] = squares();
assert_eq!(SQUARES[0], 0);
assert_eq!(SQUARES[15], 225);
```

## AP-C-B004

```rust
use rust_appendix_lab::const_eval::byte_sum;

const EXPECTED: u32 = byte_sum(b"Rust");
let received = Vec::from(b"Rust".as_slice());
assert_eq!(byte_sum(&received), EXPECTED);
assert_eq!(EXPECTED, 430);
```

## AP-C-B005

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

static REQUESTS: AtomicUsize = AtomicUsize::new(0);

REQUESTS.fetch_add(1, Ordering::Relaxed);
assert_eq!(REQUESTS.load(Ordering::Relaxed), 1);
```

## AP-C-B006

```rust
use std::collections::HashMap;
use std::sync::LazyLock;

static LABELS: LazyLock<HashMap<u8, &'static str>> = LazyLock::new(|| {
    HashMap::from([(1, "queued"), (2, "finished")])
});

assert_eq!(LABELS.get(&2), Some(&"finished"));
```

## AP-C-B007

```rust
use rust_appendix_lab::const_eval::Frame;

const HEADER: Option<Frame<4>> = Frame::new([1, 2, 3, 4]);
assert_eq!(HEADER.unwrap().bytes(), &[1, 2, 3, 4]);
assert!(Frame::new([0_u8; 65]).is_none());
```

## AP-C-B008

```rust,compile_fail
struct Packet<const N: usize> {
    bytes: [u8; N + 1],
}

fn main() {}
```

## AP-D-B001

```rust,should_panic
let text = "a€z";
let limit = 2;
let preview = &text[..limit];
println!("{preview}");
```

## AP-D-B005

```rust
fn preview(text: &str, max_scalars: usize) -> &str {
    let end = text.char_indices()
        .nth(max_scalars)
        .map_or(text.len(), |(offset, _)| offset);
    &text[..end]
}

assert_eq!(preview("a€z", 2), "a€");
assert_eq!(preview("東京", 1), "東");
assert_eq!(preview("Rust", 0), "");
assert_eq!(preview("Rust", 20), "Rust");
```

## AP-D-B007

```rust
use std::collections::HashSet;
use rust_appendix_lab::performance::{SortedIndex, count_hashed, count_linear};

let values = [8, 3, 3];
let queries = [3, 3, 7];
let hashed: HashSet<_> = values.into_iter().collect();
assert_eq!(count_linear(&values, &queries), 2);
assert_eq!(SortedIndex::new(&values).count(&queries), 2);
assert_eq!(count_hashed(&hashed, &queries), 2);
```

## AP-E-B001

```rust
fn read_u32_le(bytes: &[u8]) -> Option<u32> {
    let prefix = bytes.get(..4)?;
    // SAFETY: four initialized bytes in one live allocation; u32 permits
    // every bit pattern, and read_unaligned does not require alignment.
    let native = unsafe { prefix.as_ptr().cast::<u32>().read_unaligned() };
    Some(u32::from_le(native))
}

assert_eq!(read_u32_le(&[0x78, 0x56, 0x34, 0x12]), Some(0x1234_5678));
assert_eq!(read_u32_le(&[1, 2, 3]), None);
```

## AP-E-B002

```rust
fn read_u32_le(bytes: &[u8]) -> Option<u32> {
    let array: [u8; 4] = bytes.get(..4)?.try_into().ok()?;
    Some(u32::from_le_bytes(array))
}

assert_eq!(read_u32_le(&[0x78, 0x56, 0x34, 0x12]), Some(0x1234_5678));
```

## AP-E-B006

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let counter = AtomicUsize::new(0);
let first = counter.load(Ordering::SeqCst);
let second = counter.load(Ordering::SeqCst);
counter.store(first + 1, Ordering::SeqCst);
counter.store(second + 1, Ordering::SeqCst);
assert_eq!(counter.load(Ordering::SeqCst), 1);
```

## AP-E-B008

```rust
use rust_appendix_lab::data::parse_record;

let record = parse_record(" cable , 0007 ").unwrap();
let serialized = record.to_line();
assert_eq!(serialized, "cable,7\n");
assert_eq!(parse_record(&serialized).unwrap(), record);
```

## AP-F-B007

```rust
use rust_appendix_lab::protocol::{Reply, evaluate};

assert_eq!(evaluate(b"SUM 19 23\n"), Reply::Value(42));
assert_eq!(evaluate(b"SUM 18446744073709551615 1\n"), Reply::Overflow);
assert_eq!(evaluate(b"SUM -1 2\n"), Reply::InvalidRequest);
```
