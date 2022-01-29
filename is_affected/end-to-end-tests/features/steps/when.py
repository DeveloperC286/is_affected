import os

from behave import when


@when('the argument --from-commit-hash is provided as "{from_commit_hash}".')
def set_from_commit_hash(context, from_commit_hash):
    context.arguments += " --from-commit-hash " + from_commit_hash + " "


@when('the argument --from-reference is provided as "{from_reference}".')
def set_from_reference(context, from_reference):
    context.arguments += " --from-reference " + from_reference + " "


@when('the argument --affects is provided as "{affects}".')
def set_affects(context, affects):
    context.arguments += " --affects " + affects + " "


@when('the --list flag is set.')
def set_affects(context):
    context.arguments += " --list "


@when('the --affects-current-directory flag is set.')
def set_affects(context):
    context.arguments += " --affects-current-directory "
