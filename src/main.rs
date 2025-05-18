use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::thread::sleep;
use std::time::Duration;

fn create_bars(multi_progress: &MultiProgress, num_bars: u64, total_steps: u64) -> Vec<ProgressBar> {
    let mut bars = Vec::with_capacity(num_bars as usize);

    for i in 0..num_bars {
        let bar = multi_progress.add(ProgressBar::new(total_steps));
        bar.set_style(
            ProgressStyle::default_bar()
                .template(&format!("Task {}: [{{bar:30}}] {{pos}}/{{len}} {{msg}}", i + 1))
                .expect("Failed to set progress style.")
                .progress_chars("=> "),
        );
        bars.push(bar);
    }

    bars
}

fn main() {
    const STEPS_PER_TASK: u64 = 60;

    let multi_progress = MultiProgress::new();
    let bars = create_bars(&multi_progress, 2, STEPS_PER_TASK);

    for i in 0..STEPS_PER_TASK {
        sleep(Duration::from_millis(100));
        bars[0].inc(1);
        bars[1].set_position(STEPS_PER_TASK - i);
    }
}