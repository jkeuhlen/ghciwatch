use indoc::indoc;
use nix::sys::signal;
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use test_harness::test;
use test_harness::BaseMatcher;
use test_harness::GhciWatch;
use test_harness::JsonValue;

/// Test that `ghciwatch` handles GHCi crashes gracefully.
/// When GHCi is killed during a reload, ghciwatch should detect the crash
/// and shut down gracefully rather than panicking with a broken pipe error.
#[test]
async fn can_handle_ghci_crash_during_reload() {
    let mut session = GhciWatch::new("tests/data/simple")
        .await
        .expect("ghciwatch starts");

    let event = session
        .wait_for_log(BaseMatcher::message("^Started ghci$"))
        .await
        .expect("ghciwatch starts ghci");

    let ghci_pid: i32 = match event.fields.get("pid").unwrap() {
        JsonValue::Number(pid) => pid,
        value => {
            panic!("pid field has wrong type: {value:?}");
        }
    }
    .as_i64()
    .expect("pid is i64")
    .try_into()
    .expect("pid is i32");

    session
        .wait_until_ready()
        .await
        .expect("ghciwatch loads ghci");

    // Trigger a reload by modifying a file
    session
        .fs()
        .append(
            session.path("src/MyLib.hs"),
            indoc!(
                "

            hello = 1 :: Integer

            "
            ),
        )
        .await
        .unwrap();

    // Wait a moment for the reload to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Kill GHCi during the reload to simulate a crash
    signal::kill(Pid::from_raw(ghci_pid), Signal::SIGKILL).expect("Failed to kill ghci");

    // ghciwatch should detect that GHCi exited and shut down gracefully
    // (not panic with a broken pipe error)
    session
        .wait_for_log("^ghci exited:")
        .await
        .expect("ghci exits");

    // Wait for ghciwatch to shut down gracefully
    session.wait_for_log("^Shutdown requested$").await.unwrap();
    session
        .wait_for_log("^All tasks completed successfully$")
        .await
        .unwrap();

    let status = session.wait_until_exit().await.unwrap();
    assert!(status.success(), "ghciwatch exits successfully without panicking");
}

/// Test that `ghciwatch` handles file deletion gracefully without panicking.
#[test]
async fn can_handle_file_deletion_without_panic() {
    let mut session = GhciWatch::new("tests/data/simple")
        .await
        .expect("ghciwatch starts");

    session
        .wait_until_ready()
        .await
        .expect("ghciwatch loads ghci");

    // Create a temporary module
    let temp_module = session.path("src/TempModule.hs");
    session
        .fs()
        .write(
            &temp_module,
            indoc!(
                "module TempModule where

                tempValue :: Int
                tempValue = 42
                "
            ),
        )
        .await
        .unwrap();

    session
        .wait_until_add()
        .await
        .expect("ghciwatch adds new module");

    session
        .wait_for_log(BaseMatcher::compilation_succeeded())
        .await
        .expect("module compiles successfully");

    // Delete the file - this should trigger a removal, not a crash
    session.fs().remove(&temp_module).await.unwrap();

    // ghciwatch should handle the deletion gracefully
    session
        .wait_for_log(BaseMatcher::ghci_remove())
        .await
        .expect("ghciwatch removes deleted module");

    session
        .wait_for_log(BaseMatcher::compilation_succeeded())
        .await
        .expect("reload succeeds after deletion");

    // Verify ghciwatch is still functional by doing another reload
    session
        .fs()
        .append(
            session.path("src/MyLib.hs"),
            indoc!(
                "

            afterDeletion = 99 :: Integer

            "
            ),
        )
        .await
        .unwrap();

    session
        .wait_until_reload()
        .await
        .expect("ghciwatch continues to work after file deletion");

    session
        .wait_for_log(BaseMatcher::compilation_succeeded())
        .await
        .expect("compilation succeeds");
}
