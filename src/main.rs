use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::thread::sleep;
use std::time::Duration;

const STEPS_PER_TASK: u64 = 10;

fn main() {
    let multi_progress = MultiProgress::new();
    let mut handles = vec![];

    for thread_index in 0..2 {
        let bar = multi_progress.add(ProgressBar::new(STEPS_PER_TASK));
        // Formata a barra de progresso
        bar.set_style(
            ProgressStyle::default_bar()
                .template(&format!("Task {}: [{{bar:30}}] {{pos}}/{{len}} {{msg}}", thread_index + 1))
                .expect("Failed to set progress style.")
                .progress_chars("=> "),
        );

        // Dispara a Thread
        let h = std::thread::spawn(move || {
            for _ in 0..STEPS_PER_TASK {

                //////////// Faz alguma coisa
                sleep(Duration::from_millis((thread_index+1)*100)); // A pausa depende do índice da thread
                ////////////

                bar.inc(1); // Incrementa a barra de progresso (considerar o limite máximo disposto em ProgressBar::new(X))
            }
            bar.finish_with_message("Done!"); // Finaliza a barra de progresso
        });
        handles.push(h);
    }

    // Aguarda as Threads terminarem
    for h in handles {
        h.join().unwrap();
    }
}