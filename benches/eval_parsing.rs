use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn generate_source_with_eval_commands(commands: usize, lines_between: usize) -> String {
    let mut source = String::new();

    source.push_str("module TestModule where\n\n");
    source.push_str("import Data.List\n");
    source.push_str("import Control.Monad\n\n");

    for i in 0..commands {
        for _ in 0..lines_between {
            source.push_str(&format!(
                "-- Comment line\nhelperFunction{} x = x + {}\n",
                i, i
            ));
        }

        match i % 4 {
            0 => {
                source.push_str(&format!("-- $> print \"Test {}\"\n", i));
            }
            1 => {
                source.push_str(&format!("-- >>> putStrLn \"Doctest {}\"\n", i));
            }
            2 => {
                source.push_str(&format!(
                    "-- | Multi-line eval\n-- $> do\n-- $>   x <- return {}\n-- $>   print x\n",
                    i
                ));
            }
            _ => {
                source.push_str(&format!(
                    "{{- ORMOLU_DISABLE -}}\n-- $> :type helperFunction{}\n{{- ORMOLU_ENABLE -}}\n",
                    i
                ));
            }
        }
    }

    source.push_str("\nmain :: IO ()\n");
    source.push_str("main = putStrLn \"Done\"\n");

    source
}

fn bench_eval_command_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("eval_command_detection");

    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);

    let test_cases = vec![
        ("no_commands", generate_source_with_eval_commands(0, 100)),
        ("few_commands", generate_source_with_eval_commands(5, 20)),
        ("many_commands", generate_source_with_eval_commands(50, 5)),
        ("dense_commands", generate_source_with_eval_commands(100, 1)),
    ];

    for (name, source) in test_cases {
        let byte_len = source.len();

        group.throughput(Throughput::Bytes(byte_len as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), &source, |b, source| {
            b.iter(|| {
                let mut commands = Vec::new();
                let markers = ["-- $>", "-- >>>"];

                for (line_num, line) in source.lines().enumerate() {
                    for marker in &markers {
                        if line.trim_start().starts_with(marker) {
                            commands.push((line_num + 1, line));
                        }
                    }
                }

                black_box(commands.len())
            });
        });
    }

    group.finish();
}

fn bench_line_column_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("line_column_calc");

    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);

    let sizes = vec![
        ("small_file", 100, 80),
        ("medium_file", 1000, 120),
        ("large_file", 10000, 150),
        ("very_long_lines", 100, 1000),
    ];

    for (name, lines, line_length) in sizes {
        let source = {
            let mut s = String::new();
            for i in 0..lines {
                if i % 20 == 0 {
                    s.push_str(&format!("-- $> test command {}\n", i));
                } else {
                    s.push_str(&"x".repeat(line_length));
                    s.push('\n');
                }
            }
            s
        };

        group.throughput(Throughput::Bytes(source.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), &source, |b, source| {
            b.iter(|| {
                let mut byte_to_line = Vec::new();
                let mut current_line = 1;
                let mut current_col = 1;

                for (_byte_idx, ch) in source.char_indices() {
                    byte_to_line.push((current_line, current_col));

                    if ch == '\n' {
                        current_line += 1;
                        current_col = 1;
                    } else {
                        current_col += 1;
                    }
                }

                black_box(byte_to_line.len())
            });
        });
    }

    group.finish();
}

fn bench_multiline_command_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiline_commands");

    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);

    group.bench_function("parse_continuation_lines", |b| {
        let source = r#"
-- $> do
-- $>   x <- getLine
-- $>   putStrLn x
-- $>   return ()

-- >>> let y = 10
-- >>>     z = 20
-- >>> in y + z
"#
        .repeat(50);

        b.iter(|| {
            let mut in_command = false;
            let mut command_groups = Vec::new();
            let mut current_group = Vec::new();

            for line in source.lines() {
                if line.trim_start().starts_with("-- $>") || line.trim_start().starts_with("-- >>>")
                {
                    if !in_command {
                        in_command = true;
                        current_group = Vec::new();
                    }
                    current_group.push(line);
                } else if in_command {
                    in_command = false;
                    if !current_group.is_empty() {
                        command_groups.push(current_group.clone());
                    }
                }
            }

            black_box(command_groups.len())
        });
    });

    group.bench_function("extract_command_content", |b| {
        let lines = vec![
            "-- $> print 42",
            "-- >>> putStrLn \"test\"",
            "-- $>   continuation",
            "{- $> block comment eval -}",
        ];

        b.iter(|| {
            let mut contents = Vec::new();

            for line in &lines {
                if let Some(pos) = line.find("$>") {
                    let trimmed = line[pos + 2..].trim();
                    contents.push(trimmed);
                } else if let Some(pos) = line.find(">>>") {
                    let trimmed = line[pos + 3..].trim();
                    contents.push(trimmed);
                }
            }

            black_box(contents)
        });
    });

    group.finish();
}

fn bench_real_world_eval_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("eval_real_world");

    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);

    group.bench_function("doctest_module", |b| {
        let source = generate_doctest_module(50);

        b.iter(|| {
            let mut doctest_count = 0;
            let mut property_count = 0;

            for line in source.lines() {
                if line.contains(">>>") {
                    doctest_count += 1;
                } else if line.contains("prop>") {
                    property_count += 1;
                }
            }

            black_box((doctest_count, property_count))
        });
    });

    group.bench_function("mixed_eval_styles", |b| {
        let source = generate_mixed_eval_file(30);

        b.iter(|| {
            let mut eval_types = std::collections::HashMap::new();

            for line in source.lines() {
                if line.contains("$>") {
                    *eval_types.entry("dollar").or_insert(0) += 1;
                } else if line.contains(">>>") {
                    *eval_types.entry("doctest").or_insert(0) += 1;
                } else if line.contains("prop>") {
                    *eval_types.entry("property").or_insert(0) += 1;
                } else if line.contains("$setup") {
                    *eval_types.entry("setup").or_insert(0) += 1;
                }
            }

            black_box(eval_types)
        });
    });

    group.finish();
}

fn generate_doctest_module(functions: usize) -> String {
    let mut source = String::new();

    source.push_str("-- | A module with extensive doctest examples\n");
    source.push_str("module DocTestExample where\n\n");

    for i in 0..functions {
        source.push_str(&format!(
            r#"
-- | Function {} with doctests
-- 
-- >>> func{} 10 20
-- 30
-- 
-- >>> func{} 0 0
-- 0
-- 
-- >>> let x = func{} 5 5
-- >>> x * 2
-- 20
func{} :: Int -> Int -> Int
func{} x y = x + y

"#,
            i, i, i, i, i, i
        ));
    }

    source
}

fn generate_mixed_eval_file(examples: usize) -> String {
    let mut source = String::new();

    source.push_str("{-# LANGUAGE OverloadedStrings #-}\n");
    source.push_str("module MixedExamples where\n\n");
    source.push_str("import qualified Data.Text as T\n\n");

    for i in 0..examples {
        match i % 5 {
            0 => {
                source.push_str("-- Property test example\n");
                source.push_str("-- prop> \\x -> reverse (reverse x) == (x :: [Int])\n");
            }
            1 => {
                source.push_str("-- REPL command\n");
                source.push_str("-- $> :set -XTypeApplications\n");
                source.push_str("-- $> :type @Int read\n");
            }
            2 => {
                source.push_str("-- Multi-line do block\n");
                source.push_str("-- >>> do\n");
                source.push_str("-- >>>   putStr \"Enter: \"\n");
                source.push_str(&format!("-- >>>   x <- pure {}\n", i));
                source.push_str("-- >>>   print (x * 2)\n");
            }
            3 => {
                source.push_str("-- Setup code\n");
                source.push_str("-- $setup\n");
                source.push_str("-- >>> import Control.Monad\n");
                source.push_str(&format!("-- >>> let helper{} = {}\n", i, i));
            }
            _ => {
                source.push_str("-- Simple expression\n");
                source.push_str(&format!("-- $> {} + {}\n", i, i));
                source.push_str(&format!("-- {}\n", i * 2));
            }
        }

        source.push_str(&format!("\nfunction{} :: Int -> Int\n", i));
        source.push_str(&format!("function{} = (* {})\n\n", i, i));
    }

    source
}

criterion_group!(
    benches,
    bench_eval_command_detection,
    bench_line_column_calculation,
    bench_multiline_command_parsing,
    bench_real_world_eval_scenarios
);
criterion_main!(benches);
