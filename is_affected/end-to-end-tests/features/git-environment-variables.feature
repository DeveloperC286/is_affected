Feature: Git environment variables are respected and used instead of using the current working directory.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --affects is provided as "<affects>".
    Then is affected.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then is affected.


    Examples:
      | repository                               | checkout_commit                          | from_commit_hash                         | affects               |
      | https://github.com/aristocratos/btop.git | 026a9311e9999f979e5a01e415e963bcee01ea36 | b552e75dc3870e465e1bac009fe3fef5a8745cf7 | "^src/btop_draw.cpp$" |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --affects is provided as "<affects>".
    Then is not affected.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then is not affected.


    Examples:
      | repository | checkout_commit | from_commit_hash | affects |
