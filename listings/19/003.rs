#[derive(Clone, Copy)]
struct Event {
    kind: &'static str,
    payload: &'static str,
}

trait Sink {
    fn write(&mut self, event: Event) -> Result<Option<String>, &'static str>;
}

struct AllEvents;

impl Sink for AllEvents {
    fn write(&mut self, event: Event) -> Result<Option<String>, &'static str> {
        Ok(Some(format!("all:{}", event.payload)))
    }
}

struct AuditOnly;

impl Sink for AuditOnly {
    fn write(&mut self, event: Event) -> Result<Option<String>, &'static str> {
        Ok((event.kind == "audit").then(|| format!("audit:{}", event.payload)))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct DispatchReport {
    records: Vec<String>,
    failures: usize,
}

struct Pipeline {
    sinks: Vec<Box<dyn Sink>>,
}

impl Pipeline {
    fn from_config(audit_enabled: bool) -> Self {
        let mut sinks: Vec<Box<dyn Sink>> = vec![Box::new(AllEvents)];
        if audit_enabled {
            sinks.push(Box::new(AuditOnly));
        }
        Self { sinks }
    }

    fn dispatch(&mut self, event: Event) -> DispatchReport {
        let mut report = DispatchReport { records: Vec::new(), failures: 0 };
        for sink in &mut self.sinks {
            match sink.write(event) {
                Ok(Some(record)) => report.records.push(record),
                Ok(None) => {}
                Err(_) => report.failures += 1,
            }
        }
        report
    }
}

fn main() {
    let event = Event { kind: "audit", payload: "login" };
    let mut basic = Pipeline::from_config(false);
    let mut audited = Pipeline::from_config(true);

    assert_eq!(basic.dispatch(event).records, ["all:login"]);
    assert_eq!(audited.dispatch(event).records, ["all:login", "audit:login"]);
}
