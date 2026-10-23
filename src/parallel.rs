use std::sync::Arc;
use std::thread;

pub fn parallel_map<T, R, F>(items: Vec<T>, workers: usize, function: F) -> Vec<R>
where
    T: Send + 'static,
    R: Send + 'static,
    F: Fn(T) -> R + Send + Sync + 'static,
{
    if items.is_empty() { return Vec::new(); }
    let workers = workers.max(1).min(items.len());
    let function = Arc::new(function);
    let mut chunks: Vec<Vec<T>> = (0..workers).map(|_| Vec::new()).collect();
    for (index, item) in items.into_iter().enumerate() {
        chunks[index % workers].push(item);
    }

    let mut handles = Vec::with_capacity(chunks.len());
    for chunk in chunks {
        let function = Arc::clone(&function);
        handles.push(thread::spawn(move || {
            chunk.into_iter().map(|item| function(item)).collect::<Vec<_>>()
        }));
    }

    let mut output = Vec::new();
    for handle in handles {
        output.extend(handle.join().expect("parallel worker panicked"));
    }
    output
}

pub fn parallel_for_each<T, F>(items: Vec<T>, workers: usize, function: F)
where
    T: Send + 'static,
    F: Fn(T) + Send + Sync + 'static,
{
    parallel_map(items, workers, move |item| { function(item); });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_without_clone_bound() {
        let values = parallel_map(vec![1, 2, 3, 4], 2, |n| n * 2);
        assert_eq!(values, vec![2, 4, 6, 8]);
    }
}
