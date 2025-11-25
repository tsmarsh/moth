Feature: Issue listing and filtering

  Scenario: List shows issues
    Given a moth workspace is initialized
    And an issue "Issue 1" exists
    And an issue "Issue 2" exists with severity "high"
    When the user lists issues
    Then the command succeeds
    And 2 issues exist in "ready" status

  Scenario: List filters by status - ready
    Given a moth workspace is initialized
    And an issue "Ready issue" exists
    When the user lists issues with status "ready"
    Then the command succeeds
    And 1 issue exists in "ready" status

  Scenario: List filters by status - doing
    Given a moth workspace is initialized
    And an issue "Ready issue" exists
    When the user lists issues with status "doing"
    Then the command succeeds
    And 0 issues exist in "doing" status
