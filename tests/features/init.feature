Feature: Initialize moth workspace

  Scenario: Initialize creates directory structure
    Given an empty directory
    When the user runs init
    Then the command succeeds
    And a .moth directory exists
    And a config.yml file exists
    And ready, doing, done directories exist

  Scenario: Double initialization fails
    Given a moth workspace is initialized
    When the user runs init
    Then the command fails with "already initialized"

  Scenario: Commands without init fail - new
    Given an empty directory
    When the user creates issue "Test"
    Then the command fails

  Scenario: Commands without init fail - list
    Given an empty directory
    When the user lists issues
    Then the command fails
