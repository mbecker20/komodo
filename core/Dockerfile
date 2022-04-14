FROM node:alpine

RUN apk update && apk add docker git openrc yarn python3 --no-cache
RUN addgroup root docker && rc-update add docker boot

WORKDIR /monitor

COPY ./build/package.json ./

RUN apk add --no-cache --virtual .gyp python3 make g++ \
    && npm i \
    && apk del .gyp

COPY ./build/main.js ./

COPY ./frontend /frontend

CMD node main.js