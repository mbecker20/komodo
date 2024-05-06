#!/bin/sh

## Install, build, and setup yarn link on client.
yarn build-client

## Link, install, build frontend
yarn link @monitor/client && yarn && yarn build