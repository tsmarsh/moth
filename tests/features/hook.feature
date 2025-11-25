Feature: Git hook management

  Scenario: Install hook in git repository
    Given a moth workspace is initialized
    And a git repository is initialized
    When the user installs the hook
    Then the command succeeds
    And the prepare-commit-msg hook exists

  Scenario: Install hook when already installed
    Given a moth workspace is initialized
    And a git repository is initialized
    And the hook is installed
    When the user installs the hook
    Then the command succeeds

  Scenario: Install hook with force overwrites existing
    Given a moth workspace is initialized
    And a git repository is initialized
    And a custom prepare-commit-msg hook exists
    When the user installs the hook with force
    Then the command succeeds
    And the prepare-commit-msg hook exists

  Scenario: Uninstall hook
    Given a moth workspace is initialized
    And a git repository is initialized
    And the hook is installed
    When the user uninstalls the hook
    Then the command succeeds
    And the prepare-commit-msg hook does not exist

  Scenario: Uninstall when no hook exists
    Given a moth workspace is initialized
    And a git repository is initialized
    When the user uninstalls the hook
    Then the command succeeds

  Scenario: Install hook without git repository fails
    Given a moth workspace is initialized
    When the user installs the hook
    Then the command fails with "No .git directory"
