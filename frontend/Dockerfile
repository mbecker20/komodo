FROM node:20.5-alpine

WORKDIR /app

COPY ./frontend ./frontend
COPY ./client/core/ts ./client

RUN cd client && yarn && yarn build && yarn link
RUN cd frontend && yarn link @monitor/client && yarn && yarn build

ENV PORT 4174;

CMD cd frontend && yarn preview --host --port ${PORT}