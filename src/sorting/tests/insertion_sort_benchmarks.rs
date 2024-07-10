#[cfg(test)]
mod insertion_sort_with_tracking_tests {
    use std::error::Error;
    use crate::sorting::sort_tracker::SortTracker;
    use crate::sorting::insertion_sort_with_tracking::insertion_sort;
    use crate::util::data_generation::generate_random_vec;
    use crate::util::reporting::create_plot;

    #[test]
    fn benchmark_insertion_sort() -> Result<(), Box<dyn Error>> {
        let sizes: Vec<usize> = vec![500, 1000, 2000, 3000];
        let mut results = Vec::new();

        for &size in &sizes {
            let mut total_time: u128 = 0;
            for _ in 0..5 {
                let mut vec = generate_random_vec(size);
                let mut tracker = SortTracker::new();
                insertion_sort(&mut vec, &mut tracker);
                total_time += tracker.get_duration().as_millis();
            }
            let avg_time: u128 = total_time / 5;
            results.push((size, avg_time));
        }
        create_plot(&results, "insertion_sort_benchmark.png", "Insertion Sort Benchmark", "Insertion Sort", "Input Size", "Time (ms)")?;

        Ok(())
    }

}
