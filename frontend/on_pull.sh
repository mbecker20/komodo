#!/bin/sh

## Install, build, and setup yarn link on client.
cd ../client/core/ts && yarn && yarn build && yarn link

## Link, install, build frontend
yarn link @monitor/client && yarn && yarn build