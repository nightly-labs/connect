#!/bin/bash

read_env() {
  local filePath="${1:-.env}"

  if [ ! -f "$filePath" ]; then
    echo "Missing ${filePath}"
    exit 1
  fi

  echo "Reading $filePath"
  while IFS='=' read -r key value; do
    key=$(echo "$key" | awk '{$1=$1};1' | tr -d '\r')
    value=$(echo "$value" | sed 's/#.*//' | awk '{$1=$1};1' | tr -d '\r')

    if [[ -n $key ]] && [[ -n $value ]]; then
      if [[ "$value" =~ ^\".*\"$ || "$value" =~ ^\'.*\'$ ]]; then
        value="${value:1:-1}"  # Strip the quotes
      fi
      export "$key"="$value"
    fi
  done < "$filePath"
}
