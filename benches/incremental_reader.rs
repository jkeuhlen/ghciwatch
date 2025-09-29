use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::collections::VecDeque;

fn bench_stream_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("stream_processing");
    
    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);
    
    let data_sizes = vec![
        ("small", 1024),
        ("medium", 64 * 1024),
        ("large", 1024 * 1024),
    ];
    
    let prompts = vec!["ghci> ", "*Main> ", "Prelude> "];
    
    for (size_name, size) in &data_sizes {
        let test_data = {
            let mut data = String::new();
            for i in 0..10 {
                data.push_str(&format!("Line {}: Some output text here\n", i));
                if i % 3 == 0 {
                    data.push_str(prompts[i % prompts.len()]);
                }
            }
            data.repeat(size / data.len())
        };
        
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size_name),
            &test_data,
            |b, data| {
                b.iter(|| {
                    let mut buffer = String::new();
                    let mut chunks = Vec::new();
                    
                    for ch in data.chars() {
                        buffer.push(ch);
                        
                        for prompt in &prompts {
                            if buffer.ends_with(prompt) {
                                let chunk = buffer[..buffer.len() - prompt.len()].to_string();
                                chunks.push(chunk);
                                buffer.clear();
                                break;
                            }
                        }
                    }
                    
                    black_box(chunks.len())
                });
            },
        );
    }
    
    group.finish();
}

fn bench_buffer_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_operations");
    
    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);
    
    let test_sizes = vec![
        ("tiny", 64),
        ("small", 1024),
        ("medium", 16 * 1024),
        ("large", 256 * 1024),
    ];
    
    for (name, size) in test_sizes {
        let test_str = "x".repeat(size);
        
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::new("consume", name),
            &test_str,
            |b, input| {
                b.iter(|| {
                    let mut buffer = input.clone();
                    let consumed = if buffer.len() >= size {
                        buffer.drain(..size).collect::<String>()
                    } else {
                        String::new()
                    };
                    black_box(consumed)
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("ring_buffer", name),
            &test_str,
            |b, input| {
                b.iter(|| {
                    let mut ring = VecDeque::with_capacity(size);
                    
                    for ch in input.chars() {
                        if ring.len() >= size {
                            ring.pop_front();
                        }
                        ring.push_back(ch);
                    }
                    
                    black_box(ring.len())
                });
            },
        );
    }
    
    group.finish();
}

fn bench_utf8_handling(c: &mut Criterion) {
    let mut group = c.benchmark_group("utf8_handling");
    
    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);
    
    let test_cases = vec![
        ("ascii", "Hello, World! This is a test string.\n".repeat(1000)),
        ("mixed_utf8", "Hello 世界! Здравствуй мир! 🌍🚀\n".repeat(1000)),
        ("emoji_heavy", "🎉🎊🎈🎁🎀🎃🎄🎆🎇🧨✨🎐🎏🎎🏮\n".repeat(1000)),
    ];
    
    for (name, data) in test_cases {
        let bytes = data.as_bytes().to_vec();
        
        group.throughput(Throughput::Bytes(bytes.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &bytes,
            |b, input| {
                b.iter(|| {
                    let mut valid_chunks = Vec::new();
                    let mut buffer = Vec::new();
                    
                    for &byte in input {
                        buffer.push(byte);
                        
                        if let Ok(s) = std::str::from_utf8(&buffer) {
                            if s.ends_with('\n') {
                                valid_chunks.push(s.to_string());
                                buffer.clear();
                            }
                        }
                    }
                    
                    black_box(valid_chunks.len())
                });
            },
        );
    }
    
    group.finish();
}

fn bench_pattern_searching(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_searching");
    
    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);
    
    let test_text = {
        let mut text = String::new();
        for i in 0..1000 {
            text.push_str(&format!("Line {}: Processing data...\n", i));
            if i % 50 == 0 {
                text.push_str("ghci> ");
            }
        }
        text
    };
    
    let patterns = vec![
        vec!["ghci> "],
        vec!["ghci> ", "*Main> ", "Prelude> "],
        vec!["ghci> ", "*Main> ", "Prelude> ", "*Test> ", "*Module> "],
    ];
    
    for pattern_set in patterns {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("patterns_{}", pattern_set.len())),
            &(test_text.as_str(), &pattern_set),
            |b, (text, patterns)| {
                b.iter(|| {
                    let mut match_positions = Vec::new();
                    
                    for pattern in patterns.iter() {
                        let mut start = 0;
                        while let Some(pos) = text[start..].find(pattern) {
                            match_positions.push(start + pos);
                            start += pos + pattern.len();
                        }
                    }
                    
                    match_positions.sort_unstable();
                    black_box(match_positions.len())
                });
            },
        );
    }
    
    group.finish();
}

fn bench_real_world_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("incremental_reader_scenarios");
    
    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);
    
    group.bench_function("ghci_session", |b| {
        let session = generate_ghci_session(100);
        
        b.iter(|| {
            let mut buffer = String::new();
            let mut outputs = Vec::new();
            let prompts = ["ghci> ", "*Main> "];
            
            for ch in session.chars() {
                buffer.push(ch);
                
                for prompt in &prompts {
                    if buffer.ends_with(prompt) {
                        let output = buffer[..buffer.len() - prompt.len()].to_string();
                        outputs.push(output);
                        buffer.clear();
                        break;
                    }
                }
            }
            
            black_box(outputs.len())
        });
    });
    
    group.bench_function("streaming_output", |b| {
        let output = generate_compilation_output(50);
        
        b.iter(|| {
            let mut lines = Vec::new();
            let mut current = String::new();
            
            for ch in output.chars() {
                current.push(ch);
                if ch == '\n' {
                    lines.push(current.clone());
                    current.clear();
                }
            }
            
            black_box(lines.len())
        });
    });
    
    group.finish();
}

fn generate_ghci_session(commands: usize) -> String {
    let mut session = String::new();
    
    for i in 0..commands {
        session.push_str(&format!(":load Module{}\n", i));
        session.push_str(&format!("[1 of 3] Compiling Module{}.Types\n", i));
        session.push_str(&format!("[2 of 3] Compiling Module{}.Utils\n", i));
        session.push_str(&format!("[3 of 3] Compiling Module{}\n", i));
        session.push_str("Ok, 3 modules loaded.\n");
        
        if i % 2 == 0 {
            session.push_str("*Main> ");
        } else {
            session.push_str("ghci> ");
        }
        
        session.push_str(&format!("test{}\n", i));
        session.push_str(&format!("Result: {}\n", i * 42));
        session.push_str("ghci> ");
    }
    
    session
}

fn generate_compilation_output(modules: usize) -> String {
    let mut output = String::new();
    
    for i in 0..modules {
        output.push_str(&format!("[{} of {}] Compiling Module.Path.Component{} ", i + 1, modules, i));
        output.push_str(&format!("( src/Module/Path/Component{}.hs, interpreted )\n", i));
        
        if i % 5 == 0 {
            output.push_str(&format!("src/Module/Path/Component{}.hs:10:5: warning:\n", i));
            output.push_str("    Defined but not used: 'helper'\n");
        }
    }
    
    output.push_str(&format!("Ok, {} modules loaded.\n", modules));
    output
}

criterion_group!(
    benches,
    bench_stream_processing,
    bench_buffer_operations,
    bench_utf8_handling,
    bench_pattern_searching,
    bench_real_world_scenarios
);
criterion_main!(benches);