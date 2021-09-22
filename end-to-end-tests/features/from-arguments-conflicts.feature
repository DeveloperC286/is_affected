Feature: The from arguments conflict with one another and can not be provided at the same time.


  Scenario Outline: You can not provide both a reference and a commit hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --from-reference is provided as "<from_reference>".
    And the --list flag is set.
    Then their is a conflicting from arguments error.


    Examples:
      | repository                                                    | checkout_commit                          | from_commit_hash                         | from_reference |
      | https://github.com/ErickWendel/semana-javascript-expert05.git | 0fc4fe64ac53219ba202a6cc80e8bbcaa68bbd1f | c26d693cb53d3f82f52b462e1ca5f4a36510d094 | V1.2.0         |


  Scenario Outline: You must provide one of the from arguments.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the --list flag is set.
    Then their is a missing from argument error.


    Examples:
      | repository                                                    | checkout_commit                          |
      | https://github.com/ErickWendel/semana-javascript-expert05.git | 0fc4fe64ac53219ba202a6cc80e8bbcaa68bbd1f |
