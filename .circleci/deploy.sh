#!/usr/bin/env sh
set -e

if [ "${CIRCLE_PROJECT_USERNAME}" != "jagu-sayan" ]; then
  echo "This repository owner doesn't support any deployment target..."
  exit 0
fi

# Production (v1.2.3 | v1.2.3-rc.1 | v1.2.3-alpha.1 | v1.2.3-beta.1)
if echo "$CIRCLE_TAG" | grep -Eq '^v[0-9]+(\.[0-9]+)+((-(alpha|beta|rc)\.[0-9]+)+)?$'; then
  curl -L https://github.com/tcnksm/ghr/releases/download/v0.5.4/ghr_v0.5.4_linux_amd64.zip -o ghr.zip
  unzip ghr.zip
  BINARY_NAME="docker-owl_${ARCH}"
  cp /root/.cargo/bin/docker-owl "$BINARY_NAME"
  musl-strip "$BINARY_NAME"
  ./ghr -u "$CIRCLE_PROJECT_USERNAME" -r "$CIRCLE_PROJECT_REPONAME" "$CIRCLE_TAG" "$BINARY_NAME"
  exit 0
fi

echo "No deployment target found..."
