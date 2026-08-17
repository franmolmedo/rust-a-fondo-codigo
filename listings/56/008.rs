enum Command {
    Apply(Change),
    Snapshot(Sender<StateSnapshot>),
    Shutdown,
}
