extern crate pbr;

use pbr::{ProgressBar, PbIter};
use std::time::Duration;
use std::thread;

#[test]
fn simple_example() {
    let count = 5000;
    let mut pb = ProgressBar::new(count);
    pb.format("╢▌▌░╟");
    for _ in 0..count {
        pb.inc();
        thread::sleep(Duration::from_millis(5));
    }
    println!("done!");
}

#[test]
fn simple_iter_example(){
    for _ in PbIter::new(0..20000) {
        thread::sleep(Duration::from_millis(1));
    }
    println!("done!");
}

#[test]
fn timeout_example() {
    let count = 10;
    let mut pb = ProgressBar::new(count*20);
    pb.tick_format("▏▎▍▌▋▊▉██▉▊▋▌▍▎▏");
    pb.show_message = true;
    pb.inc();
    for _ in 0..count {
        for _ in 0..20 {
            pb.message("Waiting  : ");
            thread::sleep(Duration::from_millis(50));
            pb.tick();
        }
        for _ in 0..20 {
            pb.message("Connected: ");
            thread::sleep(Duration::from_millis(50));
            pb.inc();
        }
    }
    for _ in 0..10 {
        pb.message("Cleaning :");
        thread::sleep(Duration::from_millis(50));
        pb.tick();
    }
    pb.finish();
    println!("done!");
}

#[test]
fn npm_bar() {
   let count = 30;
    let mut pb = ProgressBar::new(count*5);
    pb.tick_format("\\|/-");
    pb.format("|#--|");
    pb.show_tick = true;
    pb.show_speed = false;
    pb.show_percent = false;
    pb.show_counter = false;
    pb.show_time_left = false;
    pb.inc();
    for _ in 0..count {
        for _ in 0..5 {
            pb.message("normalize -> thing ");
            thread::sleep(Duration::from_millis(80));
            pb.tick();
        }
        for _ in 0..5 {
            pb.message("fuzz -> tree       ");
            thread::sleep(Duration::from_millis(80));
            pb.inc();
        }
    }
    pb.finish();
    println!("done!");
}
