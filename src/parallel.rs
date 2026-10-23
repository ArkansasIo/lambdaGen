use std::sync::Arc;
use std::thread;

pub fn parallel_map<T, R, F>(items: Vec<T>, workers: usize, function: F) -> Vec<R>
where T: Send + 'static, R: Send + 'static, F: Fn(T) -> R + Send + Sync + 'static {
    if items.is_empty() { return Vec::new(); }
    let workers = workers.max(1).min(items.len());
    let function = Arc::new(function);
    let chunks = {
        let size = (items.len() + workers - 1) / workers;
        items.into_iter().collect::<Vec<_>>().chunks(size).map(|c| c.to_vec()).collect::<Vec<_>>()
    };
    let mut handles = Vec::new();
    for chunk in chunks {
        let function = Arc::clone(&function);
        handles.push(thread::spawn(move || chunk.into_iter().map(|item| function(item)).collect::<Vec<_>>()));
    }
    let mut output = Vec::new();
    for handle in handles { output.extend(handle.join().expect("parallel worker panicked")); }
    output
}

pub fn parallel_for_each<T, F>(items: Vec<T>, workers: usize, function: F)
where T: Send + 'static, F: Fn(T) + Send + Sync + 'static {
    parallel_map(items, workers, move |item| { function(item); });
}
