// Transición NO atómica aunque cada operación lo sea:
let current = counter.load(Ordering::Relaxed);
counter.store(current + 1, Ordering::Relaxed);
