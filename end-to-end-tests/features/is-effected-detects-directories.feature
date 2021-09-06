Feature: Is Effected can detect if a directory has been effect or not over a range of Git commits.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --effects is provided as "<effects>".
    Then is effected.


    Examples:
      | repository                        | checkout_commit                          | from_commit_hash                         | effects |
      | https://github.com/rust-lang/libm | 89eaba13ebe7044f6e2034dcd90c5c945b7c3a01 | b0f666e1a218740db09d7d6efb6a610be470225c | src/    |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --effects is provided as "<effects>".
    Then is not effected.


    Examples:
      | repository                                | checkout_commit                          | from_commit_hash                         | effects    |
      | https://gitlab.com/susurrus/serialport-rs | cdc0dd2ea60e0d273a7524117922da4bc7a8868b | 957a83744ad3d410608b687a42fa6990be9e9f61 | src/posix/ |