Feature: A Git commit hash can be provided as an argument to indicate where to start taking the range of commits from till HEAD, to check if the resources are effected.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --effects is provided as "<effects>".
    Then is effected.


    Examples:
      | repository                           | checkout_commit                          | from_commit_hash                         | effects    |
      | https://github.com/Lokathor/bytemuck | e01c6b3a2df2bd1636fa67b73b9392e826c5de33 | fd901461adbca0031d4b717781b0503dc838d15f | Cargo.toml |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --effects is provided as "<effects>".
    Then is not effected.


    Examples:
      | repository                        | checkout_commit                          | from_commit_hash                         | effects |
      | https://github.com/RAUI-labs/raui | 4d923829bd5c2f5c73b50c64378302fc3a2afcc2 | 12b7aedceb3b477bc54bdc694dec8dc239212df5 | LICENSE |


  Scenario Outline: When you provide a commit hash which does not exist a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then their is a could not find commit hash "<from_commit_hash>" error.


    Examples:
      | repository                                  | checkout_commit                          | from_commit_hash                         |
      | https://github.com/SergioBenitez/Rocket.git | 549c9241c41320fc5af76b53c2ffc3bd8db88f8c | ecfc2c474575c6cdbc6d273c94c13181bd1dbaa6 |
