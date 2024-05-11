#!/bin/sh

## Unlink any previous yarn links
yarn unlink --all

## Install, build, and setup yarn link on client.
yarn build-client

## Link, install, build frontend
yarn link @monitor/client && yarn && yarn build