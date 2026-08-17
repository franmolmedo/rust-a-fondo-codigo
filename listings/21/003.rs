struct DetailedReport {
    lines: [u64; 512],
}

enum Message {
    Ping,
    Close,
    Report(Box<DetailedReport>), // sin Box, cada Message mediría ~4 KB
}
