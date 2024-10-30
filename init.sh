#!/bin/sh

set +e
export GREP_OPTIONS=''

CONTAINER_NAME='getting_rusty'
CONTAINER_IMAGE='debian:bookworm'

container_exists() {
  (
    name="$1"
    docker container ls -a --format '{{.Names}}' | (
      while read -r container; do
        if [ "/${name}" = "/${container}" ]; then
          printf ' > Container already exists: %s\n' "${name}" >&2
          exit 0
        fi
      done
      exit 1
    )
  )
}

container_create() {
  (
    fresh='no'
    name="$1"
    if [ "/${name}" = '/-f' ]; then
      shift 1
      name="$1"
      fresh='yes'
    fi

    if container_exists "${name}"; then
      if [ "/${fresh}" != '/yes' ]; then
        printf '%s\n' "${name}"
        exit 0
      fi
      printf ' > Removing container: %s\n' "${name}" >&2
      if ! docker container remove -f "${name}" >&2; then
        printf ' > Error removing container: %s\n' "${name}" >&2
        exit 1
      fi
    fi

    printf ' > Creating container: %s\n' "${name}" >&2
    docker container create -it --name "${name}" --hostname "${name}" \
      -v "$(pwd):/app" \
      -p '8282:8282/tcp' -p '8282:8282/udp' \
      -p '8283:8283/tcp' -p '8283:8283/udp' \
      "${CONTAINER_IMAGE}" /usr/bin/env bash >&2 || (
      printf ' > Error creating container: %s\n' "${name}" >&2
      exit 1
    ) && (
      printf ' > Container successfully created: %s\n' "${name}" >&2
      printf '%s\n' "${name}"
      exit 0
    )
  )
}

(
  if [ $# -lt 2 ]; then
    set -- "$@" "${CONTAINER_NAME}"
  fi
  container_create "$@"
) | (
  if read -r name; then
    [ -n "${name}" ] && (
      exec </dev/tty >/dev/tty 2>&1
      docker container start -ia "${name}"
    )
  fi
)
