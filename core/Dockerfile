FROM node:alpine

RUN apk update && apk add docker git openrc --no-cache
RUN addgroup root docker && rc-update add docker boot

COPY ./frontend /frontend

WORKDIR /monitor

COPY ./build/package.json ./

RUN apk add --no-cache --virtual .gyp python3 make g++ \
    && npm i \
    && apk del .gyp

COPY ./build/main.js ./

CMD node main.js