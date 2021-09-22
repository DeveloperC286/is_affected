Feature: Check if the current directory contains any of the effected resources within the range of the commits.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --effects is provided as "<effects>".
    Then is effected.
    Given the arguments are reset.
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the directory is changed to "<directory>".
    And the --effects-current-directory flag is set.
    Then is effected.


    Examples:
      | repository                                     | checkout_commit                          | from_commit_hash                         | effects      | directory   |
      | https://gitlab.com/gridtracker.org/gridtracker | 5c84509b26015ff47440642604dff31332ae3a8f | 5c33d4a910cc3535cd91751489beb37f8d704ca3 | ^package.nw/ | package.nw/ |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --effects is provided as "<effects>".
    Then is not effected.
    Given the arguments are reset.
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the directory is changed to "<directory>".
    And the --effects-current-directory flag is set.
    Then is not effected.


    Examples:
      | repository                                 | checkout_commit                          | from_commit_hash                         | effects | directory |
      | https://github.com/woodruffw/toml2json.git | 1365f3d0ec8617ca880131f45c14977b525167b9 | fc652de8380d89fd28c3d46a8b86ccb74443c1b0 | ^src/   | src/      |
