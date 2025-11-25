Feature: Issue priority management

  Scenario: Set priority to a specific number
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    When the user sets priority of the last issue to "1"
    Then the command succeeds
    And the issue has priority 1

  Scenario: Set priority to top
    Given a moth workspace is initialized
    And an issue "First issue" exists
    And an issue "Second issue" exists
    When the user sets priority of the last issue to "top"
    Then the command succeeds

  Scenario: Set priority to bottom removes priority
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    When the user sets priority of the last issue to "1"
    Then the command succeeds
    When the user sets priority of the last issue to "bottom"
    Then the command succeeds
    And the issue has no priority

  Scenario: Compact renumbers priorities
    Given a moth workspace is initialized
    And an issue "First issue" exists
    And an issue "Second issue" exists
    When the user sets priority of the last issue to "5"
    Then the command succeeds
    When the user compacts priorities
    Then the command succeeds

  Scenario: Priority on non-prioritized status fails
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    And the issue is started
    When the user sets priority of the last issue to "1"
    Then the command fails with "not configured for prioritization"

  Scenario: Invalid priority position fails
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    When the user sets priority of the last issue to "invalid"
    Then the command fails with "Invalid position"
