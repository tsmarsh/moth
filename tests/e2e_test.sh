#!/bin/bash

set -e

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

MOTH_BIN="$1"

if [ -z "$MOTH_BIN" ]; then
    echo "Usage: $0 <path-to-moth-binary>"
    exit 1
fi

if [ ! -x "$MOTH_BIN" ]; then
    echo "Error: $MOTH_BIN is not executable"
    exit 1
fi

TEST_DIR=$(mktemp -d)
cleanup() {
    rm -rf "$TEST_DIR"
}
trap cleanup EXIT

cd "$TEST_DIR"

echo "Running e2e tests in $TEST_DIR"
echo

test_count=0
pass_count=0

run_test() {
    test_count=$((test_count + 1))
    local test_name="$1"
    shift
    echo -n "Test $test_count: $test_name ... "

    if "$@" > /dev/null 2>&1; then
        echo -e "${GREEN}PASS${NC}"
        pass_count=$((pass_count + 1))
        return 0
    else
        echo -e "${RED}FAIL${NC}"
        return 1
    fi
}

assert_exists() {
    test -e "$1"
}

assert_not_exists() {
    ! test -e "$1"
}

assert_contains() {
    local file="$1"
    local pattern="$2"
    grep -q "$pattern" "$file"
}

echo "=== Basic Initialization ==="
run_test "moth init creates .moth directory" assert_exists .moth
run_test "config.yml exists" assert_exists .moth/config.yml
run_test "ready directory exists" assert_exists .moth/ready
run_test "doing directory exists" assert_exists .moth/doing
run_test "done directory exists" assert_exists .moth/done

echo

echo "=== Creating Issues ==="
"$MOTH_BIN" new "Fix login bug" -p high --no-edit > /tmp/moth_output 2>&1
ISSUE1_ID=$(grep "Created" /tmp/moth_output | awk '{print $2}' | cut -d: -f1)
run_test "created issue with high priority" test -n "$ISSUE1_ID"

"$MOTH_BIN" new "Add dark mode" --no-edit > /tmp/moth_output 2>&1
ISSUE2_ID=$(grep "Created" /tmp/moth_output | awk '{print $2}' | cut -d: -f1)
run_test "created issue with default priority" test -n "$ISSUE2_ID"

run_test "issue file exists in ready" assert_exists ".moth/ready/${ISSUE1_ID}-high-fix-login-bug.md"
run_test "issue has correct priority in filename" assert_exists ".moth/ready/${ISSUE1_ID}-high-fix-login-bug.md"

echo

echo "=== Listing Issues ==="
"$MOTH_BIN" ls > /tmp/moth_list 2>&1
run_test "list shows ready status" assert_contains /tmp/moth_list "ready"
run_test "list shows first issue" assert_contains /tmp/moth_list "$ISSUE1_ID"
run_test "list shows second issue" assert_contains /tmp/moth_list "$ISSUE2_ID"

echo

echo "=== Moving Issues ==="
"$MOTH_BIN" start "$ISSUE1_ID"
run_test "issue moved out of ready" assert_not_exists ".moth/ready/${ISSUE1_ID}-high-fix-login-bug.md"
run_test "issue moved to doing" assert_exists ".moth/doing/${ISSUE1_ID}-high-fix-login-bug.md"

"$MOTH_BIN" done "$ISSUE1_ID"
run_test "issue moved out of doing" assert_not_exists ".moth/doing/${ISSUE1_ID}-high-fix-login-bug.md"
run_test "issue moved to done" assert_exists ".moth/done/${ISSUE1_ID}-high-fix-login-bug.md"

"$MOTH_BIN" mv "$ISSUE2_ID" doing
run_test "mv command works" assert_exists ".moth/doing/${ISSUE2_ID}-med-add-dark-mode.md"

echo

echo "=== Showing Issues ==="
"$MOTH_BIN" show "$ISSUE1_ID" > /tmp/moth_show 2>&1
run_test "show displays issue" assert_contains /tmp/moth_show "ID: $ISSUE1_ID"
run_test "show displays priority" assert_contains /tmp/moth_show "Priority: high"
run_test "show displays status" assert_contains /tmp/moth_show "Status: done"

echo

echo "=== Partial ID Matching ==="
PARTIAL_ID="${ISSUE2_ID:0:3}"
"$MOTH_BIN" show "$PARTIAL_ID" > /tmp/moth_show_partial 2>&1
run_test "partial ID works" assert_contains /tmp/moth_show_partial "ID: $ISSUE2_ID"

echo

echo "=== Deleting Issues ==="
"$MOTH_BIN" rm "$ISSUE1_ID"
run_test "rm deletes issue file" assert_not_exists ".moth/done/${ISSUE1_ID}-high-fix-login-bug.md"

echo

echo "=== List Filtering ==="
"$MOTH_BIN" ls > /tmp/moth_list_default 2>&1
run_test "default list excludes done" bash -c "! grep -q 'done' /tmp/moth_list_default"

"$MOTH_BIN" ls -a > /tmp/moth_list_all 2>&1
run_test "list -a shows all statuses" assert_contains /tmp/moth_list_all "done"

"$MOTH_BIN" ls -s doing > /tmp/moth_list_doing 2>&1
run_test "list -s filters by status" assert_contains /tmp/moth_list_doing "doing"

echo

echo "=== Priority Sorting ==="
cd "$TEST_DIR"
rm -rf .moth
"$MOTH_BIN" init > /dev/null 2>&1
"$MOTH_BIN" new "Low priority" -p low --no-edit > /dev/null 2>&1
"$MOTH_BIN" new "Critical issue" -p crit --no-edit > /dev/null 2>&1
"$MOTH_BIN" new "Medium priority" -p med --no-edit > /dev/null 2>&1
"$MOTH_BIN" new "High priority" -p high --no-edit > /dev/null 2>&1

"$MOTH_BIN" ls > /tmp/moth_list_sorted 2>&1
FIRST_PRIORITY=$(grep "\[" /tmp/moth_list_sorted | head -1 | sed 's/.*\[\(.*\)\].*/\1/')
run_test "issues sorted by priority (crit first)" test "$FIRST_PRIORITY" = "crit"

echo

echo "=== Error Handling ==="
run_test "init fails when already initialized" bash -c "! '$MOTH_BIN' init > /dev/null 2>&1"
run_test "show fails with nonexistent ID" bash -c "! '$MOTH_BIN' show nonexistent > /dev/null 2>&1"
run_test "mv fails with invalid status" bash -c "! '$MOTH_BIN' mv \$ISSUE2_ID invalid > /dev/null 2>&1"
run_test "new fails with empty title" bash -c "! '$MOTH_BIN' new '' --no-edit > /dev/null 2>&1"
run_test "new fails with invalid priority" bash -c "! '$MOTH_BIN' new 'Test' -p invalid --no-edit > /dev/null 2>&1"

echo

echo "========================================"
echo "E2E Test Summary: $pass_count/$test_count tests passed"
echo "========================================"

if [ "$pass_count" -eq "$test_count" ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed${NC}"
    exit 1
fi
