@ECHO OFF
cargo watch -c -d 0 -x "test --no-fail-fast -q --message-format short --test tests -- tests --nocapture" -x "run --quiet" -q --use-shell cmd
