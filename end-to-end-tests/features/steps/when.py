import os

from behave import when


@when('the argument --from-commit-hash is provided as "{from_commit_hash}".')
def set_from_commit_hash(context, from_commit_hash):
    context.arguments += " --from-commit-hash " + from_commit_hash + " "


@when('the argument --from-reference is provided as "{from_reference}".')
def set_from_reference(context, from_reference):
    context.arguments += " --from-reference " + from_reference + " "


@when('the argument --effects is provided as "{effects}".')
def set_effects(context, effects):
    context.arguments += " --effects " + effects + " "


@when('the --list flag is set.')
def set_effects(context):
    context.arguments += " --list "


@when('the --effects-current-directory flag is set.')
def set_effects(context):
    context.arguments += " --effects-current-directory "


@when('the directory is changed to "{directory}".')
def change_directory(_, directory):
    os.chdir(directory)
