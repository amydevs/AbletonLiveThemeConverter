#!/usr/bin/env bash
jq ".files += [\"$1\"]" pkg/package.json > tmp.json && mv tmp.json pkg/package.json