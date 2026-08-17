let workers = std::thread::available_parallelism()
    .map(|n| n.get())
    .unwrap_or(4);
