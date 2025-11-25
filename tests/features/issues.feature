Feature: Issue management

  Scenario: Create issue with default severity
    Given a moth workspace is initialized
    When the user creates issue "Fix login bug"
    Then the command succeeds
    And 1 issue exists in "ready" status
    And the issue has severity "med"

  Scenario: Create issue with severity
    Given a moth workspace is initialized
    When the user creates issue "Fix login bug" with severity "high"
    Then the command succeeds
    And 1 issue exists in "ready" status
    And the issue has severity "high"

  Scenario: Create issue with invalid severity fails
    Given a moth workspace is initialized
    When the user creates issue "Test" with severity "invalid"
    Then the command fails with "Invalid severity"

  Scenario: Create issue with --start flag moves to doing
    Given a moth workspace is initialized
    When the user creates issue "Test issue with start" with --start flag
    Then the command succeeds
    And 1 issue exists in "doing" status
    And 0 issues exist in "ready" status

  Scenario: Create issue respects no_edit config
    Given a moth workspace is initialized
    And the config has no_edit set to true
    When the user creates issue "Test issue with no_edit" without --no-edit
    Then the command succeeds
    And 1 issue exists in "ready" status

  Scenario: Show issue displays content
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    When the user shows the current issue
    Then the command fails with "No current issue"

  Scenario: Show with nonexistent id fails
    Given a moth workspace is initialized
    When the user shows issue "nonexistent"
    Then the command fails with "No issue found"

  Scenario: Delete issue removes it
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    When the user deletes the last created issue
    Then the command succeeds
    And no issues exist

  Scenario: Delete nonexistent issue fails
    Given a moth workspace is initialized
    When the user deletes issue "nonexistent"
    Then the command fails

  Scenario: Partial ID matching works
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    Then partial ID matching works for the issue
