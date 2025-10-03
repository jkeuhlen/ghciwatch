use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::collections::VecDeque;

fn generate_compilation_output(lines: usize, with_ansi: bool) -> String {
    let mut output = String::new();

    for i in 0..lines {
        if with_ansi {
            match i % 4 {
                0 => output.push_str(&format!(
                    "\x1b[32m[{} of {}] Compiling Module{}\x1b[0m\n",
                    i, lines, i
                )),
                1 => output.push_str(&format!(
                    "\x1b[33mWarning: Unused import at line {}\x1b[0m\n",
                    i
                )),
                2 => output.push_str(&format!(
                    "\x1b[31mError: Type mismatch at line {}\x1b[0m\n",
                    i
                )),
                _ => output.push_str(&format!("Normal output line {}\n", i)),
            }
        } else {
            output.push_str(&format!("[{} of {}] Compiling Module{}\n", i, lines, i));
        }
    }

    output
}

fn bench_ansi_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("ansi_processing");

    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);

    let test_cases = vec![
        (
            "no_ansi",
            "Simple text without any ANSI codes\n".repeat(100),
        ),
        (
            "light_ansi",
            "\x1b[32mGreen\x1b[0m normal \x1b[31mRed\x1b[0m\n".repeat(100),
        ),
        (
            "heavy_ansi",
            "\x1b[1;32;44mBold green on blue\x1b[0m\x1b[4mUnderline\x1b[0m\n".repeat(100),
        ),
        ("mixed_content", generate_compilation_output(100, true)),
    ];

    for (name, input) in test_cases {
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), &input, |b, input| {
            b.iter(|| {
                // Simulate ANSI stripping and parsing
                let stripped = strip_ansi_escapes::strip_str(input);
                black_box(stripped)
            });
        });
    }

    group.finish();
}

fn bench_scrollback_buffer(c: &mut Criterion) {
    let mut group = c.benchmark_group("scrollback_buffer");

    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);

    let buffer_sizes = vec![("small", 100), ("medium", 1000), ("large", 10000)];

    for (name, size) in buffer_sizes {
        let lines: Vec<String> = (0..size)
            .map(|i| format!("Line {}: Some output text here with various content", i))
            .collect();

        group.bench_with_input(BenchmarkId::new("append_line", name), &lines, |b, lines| {
            b.iter(|| {
                let mut buffer = VecDeque::with_capacity(10000);
                for line in lines {
                    buffer.push_back(line.clone());
                    if buffer.len() > 10000 {
                        buffer.pop_front();
                    }
                }
                black_box(buffer.len())
            });
        });

        group.bench_with_input(
            BenchmarkId::new("scroll_viewport", name),
            &size,
            |b, &size: &usize| {
                let mut offset = 0;
                let viewport_height = 50;

                b.iter(|| {
                    for _ in 0..100 {
                        offset = (offset + 10) % (size.saturating_sub(viewport_height));
                        let visible_range = offset..std::cmp::min(offset + viewport_height, size);
                        black_box(&visible_range);
                    }
                });
            },
        );
    }

    group.finish();
}

fn bench_text_formatting(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_formatting");

    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);

    group.bench_function("format_error_lines", |b| {
        let error_output = r#"
src/Main.hs:42:10: error:
    • No instance for (Show MyType)
        arising from a use of 'show'
    • In the expression: show myValue
      In an equation for 'main': main = show myValue
   |
42 | main = show myValue
   |          ^^^^^^^^^^^^
"#
        .repeat(20);

        b.iter(|| {
            let mut formatted_lines = Vec::new();

            for line in error_output.lines() {
                let formatted = if line.contains("error:") {
                    format!("ERROR: {}", line)
                } else if line.trim().starts_with('|') || line.trim().starts_with('^') {
                    format!("  {}", line)
                } else {
                    line.to_string()
                };

                formatted_lines.push(formatted);
            }

            black_box(formatted_lines)
        });
    });

    group.bench_function("wrap_long_lines", |b| {
        let long_text = "This is a very long line that needs to be wrapped ".repeat(20);
        let width = 80;

        b.iter(|| {
            let mut wrapped = Vec::new();
            let mut current = String::new();

            for word in long_text.split_whitespace() {
                if current.len() + word.len() + 1 > width {
                    wrapped.push(current.clone());
                    current.clear();
                }
                if !current.is_empty() {
                    current.push(' ');
                }
                current.push_str(word);
            }
            if !current.is_empty() {
                wrapped.push(current);
            }

            black_box(wrapped)
        });
    });

    group.bench_function("highlight_search", |b| {
        let text = "The quick brown fox jumps over the lazy dog\n".repeat(100);
        let search_term = "fox";

        b.iter(|| {
            let mut highlighted = Vec::new();

            for line in text.lines() {
                let mut result = String::new();
                let mut last_end = 0;

                for (start, _) in line.match_indices(search_term) {
                    result.push_str(&line[last_end..start]);
                    result.push_str("[[");
                    result.push_str(search_term);
                    result.push_str("]]");
                    last_end = start + search_term.len();
                }

                result.push_str(&line[last_end..]);
                highlighted.push(result);
            }

            black_box(highlighted)
        });
    });

    group.finish();
}

fn bench_compilation_status(c: &mut Criterion) {
    let mut group = c.benchmark_group("compilation_status");

    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);

    group.bench_function("progress_tracking", |b| {
        let total_modules = 100;

        b.iter(|| {
            let mut status_lines = Vec::new();

            for current in 0..total_modules {
                let progress = (current as f32 / total_modules as f32 * 100.0) as u8;
                let status = format!("[{}/{}] Compiling... {}%", current, total_modules, progress);
                status_lines.push(status);
            }

            black_box(status_lines)
        });
    });

    group.bench_function("format_durations", |b| {
        let durations: Vec<u64> = (0..100).map(|i| i * 10).collect();

        b.iter(|| {
            let formatted: Vec<String> = durations
                .iter()
                .map(|ms| {
                    let secs = ms / 1000;
                    let millis = ms % 1000;
                    format!("{}.{:03}s", secs, millis)
                })
                .collect();
            black_box(formatted)
        });
    });

    group.finish();
}

fn bench_real_world_tui_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("tui_real_world");

    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);

    group.bench_function("full_compilation_render", |b| {
        let output = generate_compilation_output(100, true);

        b.iter(|| {
            // Strip ANSI codes
            let stripped = strip_ansi_escapes::strip_str(&output);

            // Split into lines and manage scrollback
            let mut displayed_lines = VecDeque::with_capacity(50);
            for line in stripped.lines() {
                if displayed_lines.len() >= 50 {
                    displayed_lines.pop_front();
                }
                displayed_lines.push_back(line.to_string());
            }

            black_box(displayed_lines)
        });
    });

    group.bench_function("error_categorization", |b| {
        let mixed_output = generate_compilation_output(50, false);

        b.iter(|| {
            let mut errors = Vec::new();
            let mut warnings = Vec::new();
            let mut info = Vec::new();

            for line in mixed_output.lines() {
                if line.contains("Error:") || line.contains("error:") {
                    errors.push(line);
                } else if line.contains("Warning:") || line.contains("warning:") {
                    warnings.push(line);
                } else {
                    info.push(line);
                }
            }

            black_box((errors.len(), warnings.len(), info.len()))
        });
    });

    group.bench_function("rapid_updates", |b| {
        let update_batches: Vec<String> = (0..50)
            .map(|i| format!("Update batch {}: Processing...\n", i))
            .collect();

        b.iter(|| {
            let mut display_buffer = VecDeque::with_capacity(1000);

            for batch in &update_batches {
                for line in batch.lines() {
                    display_buffer.push_back(line.to_string());
                    if display_buffer.len() > 1000 {
                        // Remove oldest 10% when full
                        for _ in 0..100 {
                            display_buffer.pop_front();
                        }
                    }
                }
            }

            black_box(display_buffer.len())
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_ansi_processing,
    bench_scrollback_buffer,
    bench_text_formatting,
    bench_compilation_status,
    bench_real_world_tui_scenarios
);
criterion_main!(benches);
