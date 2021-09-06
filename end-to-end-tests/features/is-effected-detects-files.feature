Feature: Is Effected can detect if a file has been effect or not over a range of Git commits.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --effects is provided as "<effects>".
    Then is effected.


    Examples:
      | repository                            | checkout_commit                          | from_commit_hash                         | effects    |
      | https://github.com/BurntSushi/walkdir | abf3a15887758e0af54ebca827c7b6f8b311cb45 | bab4066b218dc20a625d405e02433d882237d59c | Cargo.toml |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --effects is provided as "<effects>".
    Then is not effected.


    Examples:
      | repository                           | checkout_commit                          | from_commit_hash                         | effects      |
      | https://github.com/Lokathor/bytemuck | e01c6b3a2df2bd1636fa67b73b9392e826c5de33 | fd901461adbca0031d4b717781b0503dc838d15f | rustfmt.toml |
