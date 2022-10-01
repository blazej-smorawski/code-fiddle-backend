#Deriving the latest base image
FROM python:3.10-slim-buster

WORKDIR /

COPY "entrypoint.sh" ./

ENTRYPOINT [ "/entrypoint.sh" ]
