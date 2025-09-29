use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, BatchSize};

fn create_sample_ghc_outputs() -> Vec<(&'static str, String)> {
    vec![
        ("simple_error", include_str!("./fixtures/simple_error.txt").to_string()),
        ("multiple_errors", include_str!("./fixtures/multiple_errors.txt").to_string()),
        ("warnings", include_str!("./fixtures/warnings.txt").to_string()),
        ("mixed_output", include_str!("./fixtures/mixed_output.txt").to_string()),
        ("large_output", {
            let base_error = r#"
src/Main.hs:42:10: error:
    • No instance for (Show MyType)
        arising from a use of 'show'
    • In the expression: show myValue
      In an equation for 'main': main = show myValue
   |
42 | main = show myValue
   |          ^^^^^^^^^^^^
"#;
            base_error.repeat(100)
        }),
        ("ansi_heavy", {
            let base = "\x1b[1;31msrc/Main.hs:1:1: \x1b[0m\x1b[1merror:\x1b[0m parse error on input\n";
            base.repeat(50)
        }),
    ]
}

fn bench_parse_ghc_messages_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_ghc_messages");
    
    // Configure for more consistent results
    group.warm_up_time(std::time::Duration::from_secs(3));
    group.measurement_time(std::time::Duration::from_secs(5));
    group.sample_size(200);
    
    for (name, input) in create_sample_ghc_outputs() {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &input,
            |b, input| {
                b.iter(|| {
                    // Simulate parsing by processing each line
                    let mut error_count = 0;
                    let mut warning_count = 0;
                    
                    for line in input.lines() {
                        if line.contains("error:") {
                            error_count += 1;
                        } else if line.contains("warning:") {
                            warning_count += 1;
                        } else if line.trim().starts_with('|') {
                            // Context line
                        } else if line.trim().is_empty() {
                            // Empty line
                        }
                    }
                    
                    black_box((error_count, warning_count))
                });
            },
        );
    }
    
    group.finish();
}

fn bench_ansi_stripping(c: &mut Criterion) {
    let mut group = c.benchmark_group("ansi_stripping");
    
    // Configure for more consistent results
    group.warm_up_time(std::time::Duration::from_secs(3));
    group.measurement_time(std::time::Duration::from_secs(5));
    group.sample_size(200);
    
    // Pre-generate all test data outside the benchmark
    let test_cases = vec![
        ("no_ansi", "src/Main.hs:1:1: error: parse error".repeat(100)),
        ("light_ansi", "\x1b[31merror:\x1b[0m parse error".repeat(100)),
        ("heavy_ansi", "\x1b[1;31m\x1b[47msrc/Main.hs:1:1:\x1b[0m \x1b[1merror:\x1b[0m \x1b[4mparse error\x1b[0m".repeat(100)),
    ];
    
    for (name, input) in test_cases {
        group.bench_function(name, |b| {
            // Use iter_batched to ensure fresh allocation for each iteration
            b.iter_batched(
                || input.clone(),
                |input| {
                    let stripped = strip_ansi_escapes::strip_str(&input);
                    black_box(stripped)
                },
                BatchSize::SmallInput
            );
        });
    }
    
    group.finish();
}

fn bench_line_parsing_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("line_parsing");
    
    // Configure for consistency
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    
    group.bench_function("parse_location", |b| {
        let lines = vec![
            "src/Main.hs:10:5: error:",
            "src/Types.hs:25:10-30: warning:",
            "app/Main.hs:1:1: error:",
            "/absolute/path/Module.hs:100:50: note:",
        ];
        
        b.iter(|| {
            for line in &lines {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 3 {
                    let _file = parts[0];
                    let _line = parts[1].parse::<usize>().ok();
                    let _col = parts[2].split(' ').next().and_then(|s| s.parse::<usize>().ok());
                }
            }
        });
    });
    
    group.bench_function("extract_context", |b| {
        let context = r#"
   |
10 | main = print myValue
   |        ^^^^^^^^^^^^^
"#;
        
        b.iter(|| {
            let mut lines = context.lines();
            while let Some(line) = lines.next() {
                if line.trim().starts_with('|') {
                    let _code_line = lines.next();
                    let _marker_line = lines.next();
                }
            }
        });
    });
    
    group.finish();
}

fn bench_real_world_parsing_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("real_world_parsing");
    
    // Configure for consistency
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    
    // Pre-generate test data
    let typecheck_input = generate_typecheck_cascade(20);
    let compilation_input = generate_compilation_output(50);
    
    group.bench_function("typecheck_failure_cascade", |b| {
        b.iter_batched(
            || typecheck_input.clone(),
            |input| {
                let mut contexts = Vec::new();
                
                for line in input.lines() {
                    if line.contains("error:") {
                        contexts.push(Vec::new());
                    } else if let Some(last) = contexts.last_mut() {
                        last.push(line);
                    }
                }
                
                black_box(contexts.len())
            },
            BatchSize::SmallInput
        );
    });
    
    group.bench_function("module_compilation_progress", |b| {
        b.iter_batched(
            || compilation_input.clone(),
            |input| {
                let mut compiled = 0;
                let mut total = 0;
                
                for line in input.lines() {
                    if line.starts_with('[') && line.contains("] Compiling") {
                        if let Some(progress) = extract_progress(line) {
                            compiled = progress.0;
                            total = progress.1;
                        }
                    }
                }
                
                black_box((compiled, total))
            },
            BatchSize::SmallInput
        );
    });
    
    group.finish();
}

fn generate_typecheck_cascade(count: usize) -> String {
    let mut output = String::new();
    for i in 1..=count {
        output.push_str(&format!(
            r#"src/Module{}.hs:{}:1: error:
    • Couldn't match type 'Int' with 'String'
      Expected: String -> String
        Actual: Int -> String
    • In the expression: func{}
      In an equation for 'test{}': test{} = func{}
   |
{} | test{} = func{}
   |          ^^^^^^

"#,
            i, i * 10, i, i, i, i, i * 10, i, i
        ));
    }
    output
}

fn generate_compilation_output(modules: usize) -> String {
    let mut output = String::new();
    
    for i in 1..=modules {
        output.push_str(&format!(
            "[{} of {}] Compiling Module.Path.Component{} ( src/Module/Path/Component{}.hs, interpreted )\n",
            i, modules, i, i
        ));
        
        if i % 5 == 0 {
            output.push_str(&format!("src/Module/Path/Component{}.hs:10:5: warning:\n", i));
            output.push_str("    Defined but not used: 'helper'\n");
        }
    }
    
    output.push_str(&format!("Ok, {} modules loaded.\n", modules));
    output
}

fn extract_progress(line: &str) -> Option<(usize, usize)> {
    let start = line.find('[')?;
    let end = line.find(']')?;
    let progress_str = &line[start + 1..end];
    let parts: Vec<&str> = progress_str.split(" of ").collect();
    
    if parts.len() == 2 {
        let current = parts[0].parse().ok()?;
        let total = parts[1].parse().ok()?;
        Some((current, total))
    } else {
        None
    }
}

criterion_group!(
    benches,
    bench_parse_ghc_messages_simulation,
    bench_ansi_stripping,
    bench_line_parsing_patterns,
    bench_real_world_parsing_scenarios
);
criterion_main!(benches);