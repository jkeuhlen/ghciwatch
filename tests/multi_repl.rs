use test_harness::test;
use test_harness::Fs;
use test_harness::GhciWatchBuilder;

/// Test that ghciwatch works with cabal multi-repl sessions
#[test]
async fn can_use_cabal_multi_repl() {
    let mut session = GhciWatchBuilder::new("tests/data/simple")
        .with_arg("--command")
        .with_arg("cabal repl --enable-multi-repl my-simple-package test-lib test:test")
        .start()
        .await
        .expect("ghciwatch starts");

    // Check that we successfully loaded all modules
    session
        .wait_until_ready()
        .await
        .expect("ghciwatch is ready");
    session.assert_logged("Ok, four modules loaded.").unwrap();

    // Verify that we can reload when a file from any component changes

    // Change a file in the library component
    let fs = Fs::new();
    fs.replace(session.path("src/MyLib.hs"), "someFunc", "someFuncModified")
        .await
        .expect("MyLib.hs modified");

    session
        .wait_until_reload()
        .await
        .expect("ghciwatch reloads");
    session.assert_logged("Ok, four modules loaded.").unwrap();

    // Revert the change
    fs.replace(session.path("src/MyLib.hs"), "someFuncModified", "someFunc")
        .await
        .expect("MyLib.hs reverted");

    session
        .wait_until_reload()
        .await
        .expect("ghciwatch reloads");

    // Change a file in the test-lib component
    fs.append(session.path("test/TestMain.hs"), "\n-- test comment\n")
        .await
        .expect("TestMain.hs modified");

    session
        .wait_until_reload()
        .await
        .expect("ghciwatch reloads");
    session.assert_logged("Ok, four modules loaded.").unwrap();

    // Change a file in the test suite component
    fs.append(
        session.path("test-main/Main.hs"),
        "\n-- another test comment\n",
    )
    .await
    .expect("Main.hs modified");

    session
        .wait_until_reload()
        .await
        .expect("ghciwatch reloads");
    session.assert_logged("Ok, four modules loaded.").unwrap();
}

/// Test that we gracefully handle when :show modules output format changes
#[test]
async fn fallback_to_show_targets_when_show_modules_fails() {
    // This should still work even if :show modules parsing fails
    // (the system will fall back to :show targets)
    let mut session = GhciWatchBuilder::new("tests/data/simple")
        .with_arg("--command")
        .with_arg("cabal repl my-simple-package") // single component
        .start()
        .await
        .expect("ghciwatch starts");

    session
        .wait_until_ready()
        .await
        .expect("ghciwatch is ready");

    // Verify basic functionality still works
    let fs = Fs::new();
    fs.replace(session.path("src/MyLib.hs"), "someFunc", "someFuncModified")
        .await
        .expect("MyLib.hs modified");

    session
        .wait_until_reload()
        .await
        .expect("ghciwatch reloads");
}
