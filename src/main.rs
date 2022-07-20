use std::{thread,time};
use indicatif::{ProgressBar,ProgressStyle};

fn main() {
    let hundred_ms = time::Duration::from_millis(100);

    for i in 0..30 {
        let pb = ProgressBar::new(40).with_message("Breath In");
        pb.set_style(ProgressStyle::default_bar()
                     .template("{msg} {bar:40.cyan/blue} [{elapsed_precise}]")
                     .progress_chars("##-"));

        for _ in 0..40 {
            thread::sleep(hundred_ms);
            pb.inc(1);
        }
        pb.finish_with_message("Done");
        let out = ProgressBar::new(60).with_message("Breath Out");
        out.set_style(ProgressStyle::default_bar()
                      .template("{msg} {bar:40.cyan/blue} [{elapsed_precise}]")
                      .progress_chars("##-"));

        for _ in 0..60 {
            thread::sleep(hundred_ms);
            out.inc(1);
        }
        out.finish_with_message(format!("Done {}", i + 1));
    }
}
