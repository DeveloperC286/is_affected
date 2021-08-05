import os

from subprocess import Popen, PIPE


def execute_command(command):
    process = Popen(
        command,
        shell=True,
        stdin=PIPE,
        stdout=PIPE,
        stderr=PIPE)
    process.wait()

    stdout, stderr = process.communicate()
    return process.returncode, stdout.decode("utf-8"), stderr.decode("utf-8")


def execute_is_effected(context):
    os.chdir(context.temporary_directory.name)
    (context.exit_code, context.stdout, context.stderr) = execute_command(
        context.is_effected_path + context.arguments)
    os.chdir(context.behave_directory)
