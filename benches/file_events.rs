use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::collections::BTreeSet;
use std::path::PathBuf;

fn create_test_paths(count: usize, depth: usize) -> Vec<PathBuf> {
    let mut paths = Vec::with_capacity(count);
    
    for i in 0..count {
        let mut path = PathBuf::from("/project/src");
        for d in 0..depth {
            path.push(format!("module_{}", d));
        }
        path.push(format!("file_{}.hs", i));
        paths.push(path);
    }
    
    paths
}

fn bench_file_event_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_event_processing");
    
    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);
    
    let test_cases = vec![
        ("single_file", 1, 1),
        ("small_batch", 10, 2),
        ("medium_batch", 100, 3),
        ("large_batch", 1000, 4),
        ("deep_hierarchy", 100, 10),
    ];
    
    for (name, count, depth) in test_cases {
        let paths = create_test_paths(count, depth);
        
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &paths,
            |b, paths| {
                b.iter(|| {
                    let mut events = BTreeSet::new();
                    
                    for path in paths {
                        events.insert(("modify", path.clone()));
                    }
                    
                    black_box(events.len())
                });
            },
        );
    }
    
    group.finish();
}

fn bench_path_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("path_operations");
    
    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);
    
    group.bench_function("path_comparison", |b| {
        let paths1 = create_test_paths(100, 3);
        let paths2 = create_test_paths(100, 3);
        
        b.iter(|| {
            let mut matches = 0;
            for p1 in &paths1 {
                for p2 in &paths2 {
                    if p1 == p2 {
                        matches += 1;
                    }
                }
            }
            black_box(matches)
        });
    });
    
    group.bench_function("path_extension_check", |b| {
        let paths = create_test_paths(1000, 3);
        
        b.iter(|| {
            let mut haskell_files = 0;
            for path in &paths {
                if path.extension().map_or(false, |ext| ext == "hs" || ext == "lhs") {
                    haskell_files += 1;
                }
            }
            black_box(haskell_files)
        });
    });
    
    group.bench_function("path_parent_extraction", |b| {
        let paths = create_test_paths(500, 5);
        
        b.iter(|| {
            let mut parents = BTreeSet::new();
            for path in &paths {
                if let Some(parent) = path.parent() {
                    parents.insert(parent.to_path_buf());
                }
            }
            black_box(parents.len())
        });
    });
    
    group.finish();
}

fn bench_btreeset_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("btreeset_operations");
    
    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);
    
    let sizes = vec![10, 100, 1000, 5000];
    
    for size in sizes {
        group.bench_with_input(
            BenchmarkId::new("insert", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut set = BTreeSet::new();
                    for i in 0..size {
                        let path = PathBuf::from(format!("/src/file_{}.hs", i));
                        set.insert(("modify", path));
                    }
                    black_box(set)
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("contains", size),
            &size,
            |b, &size| {
                let mut set = BTreeSet::new();
                let paths: Vec<_> = (0..size)
                    .map(|i| PathBuf::from(format!("/src/file_{}.hs", i)))
                    .collect();
                
                for path in &paths {
                    set.insert(("modify", path.clone()));
                }
                
                b.iter(|| {
                    let mut found = 0;
                    for path in &paths {
                        if set.contains(&("modify", path.clone())) {
                            found += 1;
                        }
                    }
                    black_box(found)
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("merge", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut set1 = BTreeSet::new();
                    let mut set2 = BTreeSet::new();
                    
                    for i in 0..size/2 {
                        let path = PathBuf::from(format!("/src/file_{}.hs", i));
                        set1.insert(("modify", path));
                    }
                    
                    for i in size/4..3*size/4 {
                        let path = PathBuf::from(format!("/src/file_{}.hs", i));
                        set2.insert(("create", path));
                    }
                    
                    set1.extend(set2);
                    black_box(set1)
                });
            },
        );
    }
    
    group.finish();
}

fn bench_event_filtering(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_filtering");
    
    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);
    
    group.bench_function("filter_haskell_files", |b| {
        let mut events = BTreeSet::new();
        for i in 0..1000 {
            let ext = match i % 5 {
                0 => "hs",
                1 => "lhs",
                2 => "cabal",
                3 => "yaml",
                _ => "txt",
            };
            let path = PathBuf::from(format!("/src/file_{}.{}", i, ext));
            events.insert(("modify", path));
        }
        
        b.iter(|| {
            let haskell_events: BTreeSet<_> = events
                .iter()
                .filter(|(_, path)| {
                    path.extension().map_or(false, |ext| ext == "hs" || ext == "lhs")
                })
                .cloned()
                .collect();
            black_box(haskell_events)
        });
    });
    
    group.bench_function("deduplicate_events", |b| {
        let mut events = Vec::new();
        for i in 0..1000 {
            let path = PathBuf::from(format!("/src/file_{}.hs", i % 100));
            events.push(("modify", path.clone()));
            if i % 3 == 0 {
                events.push(("create", path));
            }
        }
        
        b.iter(|| {
            let unique: BTreeSet<_> = events.iter().cloned().collect();
            black_box(unique)
        });
    });
    
    group.finish();
}

fn bench_real_world_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_events_real_world");
    
    // Configure for consistent results
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(100);
    
    group.bench_function("ide_save_burst", |b| {
        let files: Vec<_> = (0..20)
            .map(|i| PathBuf::from(format!("/project/src/Module{}.hs", i)))
            .collect();
        
        b.iter(|| {
            let mut all_events = BTreeSet::new();
            
            for _ in 0..5 {
                for file in &files {
                    all_events.insert(("modify", file.clone()));
                }
            }
            
            black_box(all_events)
        });
    });
    
    group.bench_function("git_checkout", |b| {
        let modified: Vec<_> = (0..50)
            .map(|i| PathBuf::from(format!("/project/src/modified_{}.hs", i)))
            .collect();
        let created: Vec<_> = (0..20)
            .map(|i| PathBuf::from(format!("/project/src/new_{}.hs", i)))
            .collect();
        let removed: Vec<_> = (0..10)
            .map(|i| PathBuf::from(format!("/project/src/old_{}.hs", i)))
            .collect();
        
        b.iter(|| {
            let mut events = BTreeSet::new();
            
            for path in &modified {
                events.insert(("modify", path.clone()));
            }
            for path in &created {
                events.insert(("create", path.clone()));
            }
            for path in &removed {
                events.insert(("remove", path.clone()));
            }
            
            black_box(events)
        });
    });
    
    group.bench_function("build_artifacts_ignore", |b| {
        let mut all_paths = Vec::new();
        
        for i in 0..100 {
            all_paths.push(PathBuf::from(format!("/project/src/Module{}.hs", i)));
            all_paths.push(PathBuf::from(format!("/project/.stack-work/dist/Module{}.o", i)));
            all_paths.push(PathBuf::from(format!("/project/dist-newstyle/build/Module{}.hi", i)));
        }
        
        b.iter(|| {
            let mut events = BTreeSet::new();
            for path in &all_paths {
                events.insert(("modify", path.clone()));
            }
            
            let filtered: BTreeSet<_> = events
                .into_iter()
                .filter(|(_, path)| {
                    !path.components().any(|c| {
                        matches!(c.as_os_str().to_str(), Some(".stack-work") | Some("dist-newstyle"))
                    })
                })
                .collect();
            
            black_box(filtered)
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_file_event_processing,
    bench_path_operations,
    bench_btreeset_operations,
    bench_event_filtering,
    bench_real_world_scenarios
);
criterion_main!(benches);