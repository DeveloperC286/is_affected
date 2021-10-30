Feature: Is affected can detect if a directory has been affect or not over a range of Git commits.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --affects is provided as "<affects>".
    Then is affected.


    Examples:
      | repository                        | checkout_commit                          | from_commit_hash                         | affects |
      | https://github.com/rust-lang/libm | 89eaba13ebe7044f6e2034dcd90c5c945b7c3a01 | b0f666e1a218740db09d7d6efb6a610be470225c | src/    |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --affects is provided as "<affects>".
    Then is not affected.


    Examples:
      | repository                                | checkout_commit                          | from_commit_hash                         | affects    |
      | https://gitlab.com/susurrus/serialport-rs | cdc0dd2ea60e0d273a7524117922da4bc7a8868b | 957a83744ad3d410608b687a42fa6990be9e9f61 | src/posix/ |
