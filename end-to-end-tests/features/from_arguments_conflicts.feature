Feature: The from arguments conflict with one another and can not be provided at the same time.


  Scenario Outline: You can not provide both a reference and a commit hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the --list flag is set.
    And the argument --from-reference is provided as "<from_reference>".
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then their is a conflicting from arguments error.


    Examples:
      | repository                                                    | checkout_commit                          | from_commit_hash                         | from_reference |
	  | https://github.com/ErickWendel/semana-javascript-expert05.git | 0fc4fe64ac53219ba202a6cc80e8bbcaa68bbd1f | c26d693cb53d3f82f52b462e1ca5f4a36510d094 | V1.2.0         |


  Scenario Outline: You can not provide both a reference and a commit hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the --list flag is set.
    And the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --from-reference is provided as "<from_reference>".
    Then their is a conflicting from arguments error.


    Examples:
      | repository                                                    | checkout_commit                          | from_commit_hash                         | from_reference |
	  | https://github.com/ErickWendel/semana-javascript-expert05.git | 0fc4fe64ac53219ba202a6cc80e8bbcaa68bbd1f | c26d693cb53d3f82f52b462e1ca5f4a36510d094 | V1.2.0         |
