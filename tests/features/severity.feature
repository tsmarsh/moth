Feature: Issue severity management

  Scenario: Change severity from med to high
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    Then the issue has severity "med"
    When the user changes severity of the last issue to "high"
    Then the command succeeds
    And the issue has severity "high"

  Scenario: Change severity from med to crit
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    When the user changes severity of the last issue to "crit"
    Then the command succeeds
    And the issue has severity "crit"

  Scenario: Change severity from med to low
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    When the user changes severity of the last issue to "low"
    Then the command succeeds
    And the issue has severity "low"

  Scenario: Change severity with invalid level fails
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    When the user changes severity of the last issue to "invalid"
    Then the command fails with "Invalid severity"

  Scenario: Change severity of nonexistent issue fails
    Given a moth workspace is initialized
    When the user changes severity of issue "nonexistent" to "high"
    Then the command fails with "No issue found"
