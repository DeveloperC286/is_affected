Feature: Git environment variables are respected and used instead of using the current working directory.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --affects is provided as "<affects>".
    Then is affected.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then is affected.


    Examples:
      | repository                           | checkout_commit                          | from_commit_hash                         | affects    |
      | https://github.com/Lokathor/bytemuck | e01c6b3a2df2bd1636fa67b73b9392e826c5de33 | fd901461adbca0031d4b717781b0503dc838d15f | Cargo.toml |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --affects is provided as "<affects>".
    Then is not affected.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then is not affected.


    Examples:
      | repository                        | checkout_commit                          | from_commit_hash                         | affects |
      | https://github.com/RAUI-labs/raui | 4d923829bd5c2f5c73b50c64378302fc3a2afcc2 | 12b7aedceb3b477bc54bdc694dec8dc239212df5 | LICENSE |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-reference is provided as "<from_reference>".
    And the argument --affects is provided as "<affects>".
    Then is affected.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then is affected.


    Examples:
      | repository                              | checkout_commit                          | from_reference | affects                             |
      | https://github.com/xfangfang/Macast.git | f105522c9290dd3202249ff8c8778bebcd3f361b | v0.5           | .github/workflows/build-macast.yaml |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-reference is provided as "<from_reference>".
    And the argument --affects is provided as "<affects>".
    Then is not affected.
    Given the GIT_DIR environment variable is set to the cloned repository.
    Then is not affected.


    Examples:
      | repository                               | checkout_commit                          | from_reference | affects        |
      | https://gitlab.com/wwn-mode/WWN-Mode.git | 3839e0b2a02687a23aae4fa49d5ec0eea1762f17 | 0.0.2Update    | Units/BUM0.ini |
