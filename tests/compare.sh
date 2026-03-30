#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
INPUT_FILE="${1:-$ROOT_DIR/tests/url.txt}"
CURL_DIR="${CURL_DIR:-$ROOT_DIR/tests/curl}"
BRCURL_DIR="${BRCURL_DIR:-$ROOT_DIR/tests/bcurl}"
BRCURL_O_DIR="${BRCURL_O_DIR:-$ROOT_DIR/tests/bcurl-o}"
BRCURL_TIME="${BRCURL_TIME:-5}"
BRCURL_BIN="${BRCURL_BIN:-$ROOT_DIR/target/debug/brcurl}"

mkdir -p "$CURL_DIR" "$BRCURL_DIR" "$BRCURL_O_DIR"

(
  cd "$ROOT_DIR"
  cargo build --quiet --bin brcurl
)

slugify() {
  local input="$1"
  local slug
  slug="$(printf '%s' "$input" | tr -c '[:alnum:]' '_' | cut -c1-80)"
  slug="${slug%%_}"
  if [[ -z "$slug" ]]; then
    slug="url"
  fi
  printf '%s' "$slug"
}

write_error() {
  local path="$1"
  local message="$2"
  printf 'ERROR: %s\n' "$message" >"$path"
}

index=0
while IFS= read -r line || [[ -n "$line" ]]; do
  url="${line#"${line%%[![:space:]]*}"}"
  url="${url%"${url##*[![:space:]]}"}"

  if [[ -z "$url" || "$url" == \#* ]]; then
    continue
  fi

  index=$((index + 1))
  stem="$(printf '%03d_%s' "$index" "$(slugify "$url")")"
  curl_path="$CURL_DIR/$stem.txt"
  brcurl_path="$BRCURL_DIR/$stem.txt"
  brcurl_o_path="$BRCURL_O_DIR/$stem.png"

  if ! curl -L --silent --show-error "$url" >"$curl_path"; then
    write_error "$curl_path" "curl failed for $url"
  fi

  if ! "$BRCURL_BIN" -t "$BRCURL_TIME" "$url" >"$brcurl_path"; then
    write_error "$brcurl_path" "brcurl failed for $url"
  fi

  if ! "$BRCURL_BIN" -t "$BRCURL_TIME" -o="$brcurl_o_path" "$url" >/dev/null; then
    write_error "${brcurl_o_path%.png}.txt" "brcurl -o failed for $url"
    rm -f "$brcurl_o_path"
  fi
done <"$INPUT_FILE"
