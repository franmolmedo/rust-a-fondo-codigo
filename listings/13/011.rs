#[derive(Debug, PartialEq)]
struct Record {
    id: u64,
    name: String,
}

#[derive(Debug, PartialEq)]
enum ImportError {
    MissingField { line: usize },
    InvalidId { line: usize },
}

#[derive(Debug, PartialEq)]
struct ImportReport {
    accepted: Vec<Record>,
    rejected: Vec<ImportError>,
}

fn parse_line(line_number: usize, line: &str) -> Result<Record, ImportError> {
    let (raw_id, name) = line
        .split_once(',')
        .ok_or(ImportError::MissingField { line: line_number })?;
    let id = raw_id
        .trim()
        .parse()
        .map_err(|_| ImportError::InvalidId { line: line_number })?;
    Ok(Record { id, name: name.trim().to_owned() })
}

fn import_all(input: &str) -> ImportReport {
    let mut report = ImportReport { accepted: Vec::new(), rejected: Vec::new() };

    for (index, line) in input.lines().enumerate() {
        match parse_line(index + 1, line) {
            Ok(record) => report.accepted.push(record),
            Err(error) => report.rejected.push(error),
        }
    }
    report
}

fn main() {
    let report = import_all("1,Ada\nbad,Grace\n3,Linus");
    assert_eq!(report.accepted.len(), 2);
    assert_eq!(report.rejected, [ImportError::InvalidId { line: 2 }]);
}
