use std::{
    cmp, env,
    fmt::Display,
    hint::black_box,
    io::{stdout, Write},
    process::{self, Output},
    time::{Duration, Instant},
};

use super::{aoc_cli, Day};
use crate::template::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};

pub fn run_part<I: Clone, T: Display>(func: impl Fn(I) -> Option<T>, input: I, day: Day, part: u8) {
    let part_str = format!("Part {part}");

    let (result, duration, samples) =
        run_timed(func, input, |result| print_result(result, &part_str, ""));

    print_result(&result, &part_str, &format_duration(&duration, samples));

    if let Some(result) = result {
        submit_result(result, day, part);
    }
}

fn run_timed<I: Clone, T>(
    func: impl Fn(I) -> T,
    input: I,
    hook: impl Fn(&T),
) -> (T, Duration, u128) {
    let timer = Instant::now();
    let result = {
        let input = input.clone();

        #[cfg(feature = "dhat-heap")]
        let _profiler = dhat::Profiler::new_heap();

        func(input)
    };

    let base_time = timer.elapsed();

    hook(&result);

    let run = if std::env::args().any(|x| x == "--time") {
        bench(func, input, &base_time)
    } else {
        (base_time, 1)
    };

    (result, run.0, run.1)
}

fn bench<I: Clone, T>(func: impl Fn(I) -> T, input: I, base_time: &Duration) -> (Duration, u128) {
    let mut stdout = stdout();

    print!(" > {ANSI_ITALIC}benching{ANSI_RESET}");
    let _ = stdout.flush();

    let bench_iterations = cmp::min(
        10000,
        cmp::max(
            Duration::from_secs(1).as_nanos() / cmp::max(base_time.as_micros(), 10),
            10,
        ),
    );

    let mut timers: Vec<Duration> = vec![];

    for _ in 0..bench_iterations {
        let cloned = input.clone();
        let timer = Instant::now();
        black_box(func(black_box(cloned)));
        timers.push(timer.elapsed());
    }

    (
        #[allow(clippy::cast_possible_truncation)]
        Duration::from_nanos(average_duration(&timers) as u64),
        bench_iterations,
    )
}

fn average_duration(numbers: &[Duration]) -> u128 {
    numbers.iter().map(Duration::as_nanos).sum::<u128>() / numbers.len() as u128
}

fn format_duration(duration: &Duration, samples: u128) -> String {
    if samples == 1 {
        format!("   ({duration:.1?})")
    } else {
        format!("   ({duration:.1?}) @ {samples} samples")
    }
}

fn print_result<T: Display>(result: &Option<T>, part: &str, duration_str: &str) {
    let is_intermediate_result = duration_str.is_empty();

    match result {
        Some(result) => {
            if result.to_string().contains('\n') {
                let str = format!("{part}: {duration_str}");
                if is_intermediate_result {
                    println!("{str}");
                } else {
                    print!("\r");
                    println!("{str}");
                    println!("{result}");
                }
            } else {
                let str = format!("{part}: {ANSI_BOLD}{result}{ANSI_RESET}{duration_str}");
                if is_intermediate_result {
                    println! {"{str}"};
                } else {
                    print!("\r");
                    println!("{str}");
                }
            }
        }
        None => {
            if is_intermediate_result {
                print!("{part}: X");
            } else {
                print!("\r");
                println!("{part}: X         ");
            }
        }
    }
}

fn submit_result<T: Display>(
    result: T,
    day: Day,
    part: u8,
) -> Option<Result<Output, aoc_cli::AocCommandError>> {
    let args: Vec<String> = env::args().collect();

    if !args.contains(&"--submit".into()) {
        return None;
    }

    if args.len() < 3 {
        eprint!("Unexpected command-line input. Format: cargo solve 1 --submit 1");
        process::exit(1);
    }

    let part_index = args.iter().position(|x| x == "--submit").unwrap() + 1;

    let Ok(part_submit) = args[part_index].parse::<u8>() else {
        eprint!("Unexpected command-line output. Format: cargo solve 1 --submit 1");
        process::exit(1);
    };

    if part_submit != part {
        return None;
    }

    if aoc_cli::check().is_err() {
        eprint!("Command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
        process::exit(1);
    }

    println!("Submitting result via aoc-cli..");

    Some(aoc_cli::submit(day, part, &result.to_string()))
}
