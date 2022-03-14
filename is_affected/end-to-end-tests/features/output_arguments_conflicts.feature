Feature: The output arguments conflict with one another and can not be provided at the same time.


  Scenario Outline: You can not provide both affects and the list arguments.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --affects is provided as "<affects>".
    And the --list flag is set.
    Then their is a conflicting output arguments error.


    Examples:
      | repository                                                                         | checkout_commit                          | from_commit_hash                         | affects |
      | https://gitlab.com/NamingThingsIsHard/crypto/freqtrade/clients/freqtrade-client-py | cb1539e4c09cc67f75cfab89af04904d65ecf224 | bf5f8c7e716e8d8522be0e4f53078ff7275fda3e | LICENSE |


  Scenario Outline: You can not provide both affects and the affects current directory arguments.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --affects is provided as "<affects>".
    And the --affects-current-directory flag is set.
    Then their is a conflicting output arguments error.


    Examples:
      | repository                                                                         | checkout_commit                          | from_commit_hash                         | affects |
      | https://gitlab.com/NamingThingsIsHard/crypto/freqtrade/clients/freqtrade-client-py | cb1539e4c09cc67f75cfab89af04904d65ecf224 | bf5f8c7e716e8d8522be0e4f53078ff7275fda3e | LICENSE |


  Scenario Outline: You can not provide both the affects current directory and the list arguments.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the --affects-current-directory flag is set.
    And the --list flag is set.
    Then their is a conflicting output arguments error.


    Examples:
      | repository                                                                         | checkout_commit                          | from_commit_hash                         |
      | https://gitlab.com/NamingThingsIsHard/crypto/freqtrade/clients/freqtrade-client-py | cb1539e4c09cc67f75cfab89af04904d65ecf224 | bf5f8c7e716e8d8522be0e4f53078ff7275fda3e |


  Scenario Outline: You must provide one of the output arguments.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    Then their is a missing output argument error.


    Examples:
      | repository                                       | checkout_commit                          | from_commit_hash                         |
      | https://gitlab.com/gitmate/open-source/gitmate-2 | 2a1c955357d5ae61133ae2b47b6d686f257d98e0 | c02f0eadb908dd2880484cc6063ad6d0027d30ef |
