use test_harness::test;
use test_harness::BaseMatcher;
use test_harness::GhciWatchBuilder;

/// Test that the `--output` flag writes GHCi stdout to a file.
#[test]
async fn output_flag_writes_to_file() {
    let output_path = "ghci-output.txt";
    let mut session = GhciWatchBuilder::new("tests/data/simple")
        .with_args(["--output", output_path])
        .start()
        .await
        .expect("ghciwatch starts");

    let output_path = session.path(output_path);

    session.wait_until_ready().await.expect("ghciwatch loads");

    // Wait for file to be written using the fs helper
    session
        .fs()
        .wait_for_path(std::time::Duration::from_secs(10), &output_path)
        .await
        .expect("output file created");

    // Read the output file
    let output_contents = session
        .fs()
        .read(&output_path)
        .await
        .expect("output file exists");

    // Verify the output file contains expected GHCi output
    assert!(
        output_contents.contains("GHCi, version"),
        "Output file should contain GHCi version"
    );
    assert!(
        output_contents.contains("modules loaded") || output_contents.contains("Ok,"),
        "Output file should contain module loading info"
    );
}

/// Test that the `-o` short flag works the same as `--output`.
#[test]
async fn output_short_flag_works() {
    let output_path = "ghci-output-short.txt";
    let mut session = GhciWatchBuilder::new("tests/data/simple")
        .with_args(["-o", output_path])
        .start()
        .await
        .expect("ghciwatch starts");

    let output_path = session.path(output_path);

    session.wait_until_ready().await.expect("ghciwatch loads");

    session
        .fs()
        .wait_for_path(std::time::Duration::from_secs(10), &output_path)
        .await
        .expect("output file created");

    let output_contents = session
        .fs()
        .read(&output_path)
        .await
        .expect("output file exists");

    assert!(
        output_contents.contains("GHCi, version"),
        "Output file should contain GHCi version"
    );
}

/// Test that output file is updated on reload.
#[test]
async fn output_file_updated_on_reload() {
    let output_path = "ghci-reload-output.txt";
    let mut session = GhciWatchBuilder::new("tests/data/simple")
        .with_args(["--output", output_path])
        .start()
        .await
        .expect("ghciwatch starts");

    let output_path = session.path(output_path);

    session.wait_until_ready().await.expect("ghciwatch loads");

    session
        .fs()
        .wait_for_path(std::time::Duration::from_secs(10), &output_path)
        .await
        .expect("output file created");

    let initial_contents = session
        .fs()
        .read(&output_path)
        .await
        .expect("output file exists");

    // Trigger a reload by touching a file
    let module_path = session.path("src/MyLib.hs");
    session.fs().touch(&module_path).await.unwrap();

    session
        .wait_for_log(BaseMatcher::reload_completes())
        .await
        .unwrap();

    // Read the output file again after reload
    let reloaded_contents = session
        .fs()
        .read(&output_path)
        .await
        .expect("output file exists after reload");

    // The file should have been updated (it should be longer or contain new content)
    assert!(
        reloaded_contents.len() >= initial_contents.len(),
        "Output file should be updated after reload"
    );

    // Should still contain the compilation output
    assert!(
        reloaded_contents.contains("modules loaded") || reloaded_contents.contains("Ok,"),
        "Output file should contain reload info"
    );
}

/// Test that output file captures compilation errors.
#[test]
async fn output_file_contains_errors() {
    let output_path = "ghci-errors-output.txt";
    let mut session = GhciWatchBuilder::new("tests/data/simple")
        .with_args(["--output", output_path])
        .start()
        .await
        .expect("ghciwatch starts");

    let output_path = session.path(output_path);

    session.wait_until_ready().await.expect("ghciwatch loads");

    // Introduce a compilation error
    let module_path = session.path("src/MyModule.hs");
    session
        .fs()
        .replace(&module_path, "example :: String", "example :: ()")
        .await
        .unwrap();

    session
        .wait_for_log(BaseMatcher::compilation_failed())
        .await
        .unwrap();

    // Read the output file
    let output_contents = session
        .fs()
        .read(&output_path)
        .await
        .expect("output file exists");

    // Verify the output file contains error information and reload status
    // The output should show that compilation Failed
    assert!(
        output_contents.contains("Failed"),
        "Output file should contain 'Failed' status"
    );
}

/// Test that output works with eval mode.
#[test]
async fn output_with_eval() {
    let output_path = "ghci-eval-output.txt";
    let mut session = GhciWatchBuilder::new("tests/data/simple")
        .with_args(["--output", output_path, "--enable-eval"])
        .start()
        .await
        .expect("ghciwatch starts");

    let output_path = session.path(output_path);

    session.wait_until_ready().await.expect("ghciwatch loads");

    // Add an eval comment to a file
    let module_path = session.path("src/MyLib.hs");
    session
        .fs()
        .append(&module_path, "\n-- $> 2 + 2\n")
        .await
        .unwrap();

    session
        .wait_for_log(BaseMatcher::reload_completes())
        .await
        .unwrap();

    // Read the output file
    let output_contents = session
        .fs()
        .read(&output_path)
        .await
        .expect("output file exists");

    // The eval result should be in the output
    assert!(
        output_contents.contains("2 + 2") || output_contents.contains("4"),
        "Output file should contain eval results"
    );
}
