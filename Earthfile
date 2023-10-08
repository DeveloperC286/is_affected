VERSION 0.7


e2e-formatting-base:
		FROM python:3.12.0-slim
		COPY "./ci" "./ci"
		COPY "./is_affected/end-to-end-tests" "./is_affected/end-to-end-tests"
		RUN pip3 install -r "./is_affected/end-to-end-tests/autopep8.requirements.txt"


check-e2e-formatting:
		FROM +e2e-formatting-base
		RUN ./ci/check-e2e-formatting.sh


fix-e2e-formatting:
		FROM +e2e-formatting-base
		RUN ./ci/fix-e2e-formatting.sh
		SAVE ARTIFACT "./is_affected/end-to-end-tests" AS LOCAL "./is_affected/end-to-end-tests"
