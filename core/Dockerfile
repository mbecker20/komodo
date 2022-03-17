FROM node:alpine

RUN apk update && apk add docker git openrc --no-cache
RUN addgroup root docker && rc-update add docker boot

WORKDIR /monitor

COPY ./build/package.json ./

RUN npm i

COPY ./build/main.js ./

CMD node main.js