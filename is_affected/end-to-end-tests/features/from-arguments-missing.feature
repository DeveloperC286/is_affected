Feature: A from argument is required and one must be provided.


  Scenario Outline: You must provide either a reference or commit hash.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the --list flag is set.
    Then their is a missing from argument error.


    Examples:
      | repository                                                    | checkout_commit                          |
      | https://github.com/ErickWendel/semana-javascript-expert05.git | 0fc4fe64ac53219ba202a6cc80e8bbcaa68bbd1f |
