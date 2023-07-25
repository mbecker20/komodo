FROM node:alpine

WORKDIR /app

COPY ./frontend ./frontend
COPY ./client/ts ./client

RUN cd client && yarn && yarn build && yarn link
RUN cd frontend && yarn link @monitor/client && yarn && yarn build

CMD cd frontend && yarn preview --host