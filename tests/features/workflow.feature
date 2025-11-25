Feature: Issue workflow management

  Scenario: Start moves issue to doing
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    When the user starts the last created issue
    Then the command succeeds
    And 1 issue exists in "doing" status
    And 0 issues exist in "ready" status

  Scenario: Done moves issue to done
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    When the user marks the last created issue as done
    Then the command succeeds
    And 1 issue exists in "done" status
    And 0 issues exist in "ready" status

  Scenario: Done with no args finishes current
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    And the issue is started
    When the user marks the current issue as done
    Then the command succeeds
    And 1 issue exists in "done" status

  Scenario: Done with no args and no current fails
    Given a moth workspace is initialized
    When the user marks the current issue as done
    Then the command fails with "No current issue"

  Scenario: Move changes status
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    When the user moves the last created issue to "doing"
    Then the command succeeds
    And 1 issue exists in "doing" status
    And 0 issues exist in "ready" status

  Scenario: Move with invalid status fails
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    When the user moves the last created issue to "invalid"
    Then the command fails with "Unknown status"

  Scenario: Full workflow from ready to done
    Given a moth workspace is initialized
    And an issue "Fix bug" exists with severity "high"
    And an issue "Add feature" exists
    When the user starts the last created issue
    Then the command succeeds
    And 1 issue exists in "doing" status
    And 1 issue exists in "ready" status
    When the user marks the current issue as done
    Then the command succeeds
    And 1 issue exists in "done" status
    And 1 issue exists in "ready" status
    And 0 issues exist in "doing" status

  Scenario: Show no args shows current issue
    Given a moth workspace is initialized
    And an issue "Test issue" exists
    And the issue is started
    When the user shows the current issue
    Then the command succeeds

  Scenario: Show no args with no current fails
    Given a moth workspace is initialized
    When the user shows the current issue
    Then the command fails with "No current issue"
