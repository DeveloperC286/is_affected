Feature: The effected resources are correctly parsed from merge commit's.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the --list flag is set.
    Then the effected resources listed are "<effected_resources>".


    Examples:
      | repository                               | checkout_commit                          | from_commit_hash                         | effected_resources                                                                                                                                                                                                        |
	  | https://gitlab.com/ccsl-usp/culturaeduca | 972be0a97b119ff1db8c47396e088fcc59e96017 | b2e2e7ac7be11cb906bbead12847d31410f60c0d | "busca/forms.py\nbusca/views.py\ntemplates/base.html\ntemplates/busca/acao_listagem.html\ntemplates/busca/agente_listagem.html\ntemplates/busca/divisaourbana_listagem.html\ntemplates/busca/equipamento_listagem.html\n" |
