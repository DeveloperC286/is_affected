def assert_command_successful(context):
    assert context.exit_code == 0, f"Expected a zero exit code to indicate a successful execution.\nExit code = '{context.exit_code}'.\n"


def assert_command_unsuccessful(context):
    assert context.exit_code != 0, f"Expected a non-zero exit code to indicate a unsuccessful execution\nExit code = '{context.exit_code}'.\n"


def assert_no_output(context):
    assert context.stdout == "", f"Expected standard output to be empty.\nStandard output = {context.stdout.encode()}.\n"


def assert_no_errors(context):
    assert context.stderr == "", f"Expected standard error to be empty.\nStandard error = {context.stderr.encode()}.\n"


def assert_error_equals(context, error):
    assert context.stderr == error, f"Expected standard error to equal the error.\nStandard error = {context.stderr.encode()}.\nError          = {error.encode()}.\n"


def assert_error_matches_regex(context, regex):
    assert regex.match(
        context.stderr) is not None, f"Expected standard errors to match the regex.\nStandard error = {context.stderr.encode()}.\nRegex          = {regex.pattern.encode()}.\n"


def assert_error_is_one_of(context, errors):
    assert context.stderr in errors, f"Expected standard error to equal one of these errors.\nStandard error = {context.stderr.encode()}.\nErrors         = {errors}.\n"


def assert_affected_resources(context, affected_resources):
    affected_resources = affected_resources.strip().strip('\"').replace("\\n", '\n')
    assert context.stdout == affected_resources, f"The affected resources was not what was expected.\nExpected =\n {affected_resources}.\nActual   = \n {context.stdout}\n"
