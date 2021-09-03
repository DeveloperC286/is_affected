from behave import when


@when('the argument --from-commit-hash is provided as "{from_commit_hash}".')
def set_from_commit_hash(context, from_commit_hash):
    context.arguments += " --from-commit-hash " + from_commit_hash + " "


@when('the argument --effects is provided as "{effects}".')
def set_effects(context, effects):
    context.arguments += " --effects " + effects + " "
