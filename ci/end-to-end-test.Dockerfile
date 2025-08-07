FROM python:3.13.6-alpine3.22@sha256:7f9d4ab05a18a368db663461919926cd869343de781199a6748b61a60f28dd0a
RUN apk add --no-cache \
	git=2.49.1-r0

COPY end-to-end-tests/requirements.txt ./
RUN pip3 install -r requirements.txt

ENTRYPOINT ["behave"]
