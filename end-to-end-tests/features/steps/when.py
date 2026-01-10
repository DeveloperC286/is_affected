from behave import when


@when('linting from the "{git}".')
def set_linting_from_the(context, git):
    context.from_ref = f" \"{git}\""


@when('the argument --affects is provided as "{affects}".')
def set_affects(context, affects):
    context.arguments += f" --affects {affects} "


@when('the --list flag is set.')
def set_list_flag(context):
    context.arguments += " --list "


@when('the --affects-current-directory flag is set.')
def set_affects_current_directory_flag(context):
    context.arguments += " --affects-current-directory "
