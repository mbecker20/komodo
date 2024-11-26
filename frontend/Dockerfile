FROM node:20.12-alpine as builder

WORKDIR /builder

COPY ./frontend ./frontend
COPY ./client/core/ts ./client

# Optionally specify a specific host.
ARG VITE_KOMODO_HOST
ENV VITE_KOMODO_HOST=${VITE_KOMODO_HOST}

RUN cd client && yarn && yarn build && yarn link
RUN cd frontend && yarn link komodo_client && yarn && yarn build

ENV PORT=4174

CMD cd frontend && yarn preview --host --port ${PORT}