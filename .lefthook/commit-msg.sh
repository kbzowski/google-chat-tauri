#!/usr/bin/env sh
# Conventional Commits validator. Usage: commit-msg.sh <path-to-commit-msg>
set -e
PATTERN='^(feat|fix|docs|style|refactor|test|chore|build|ci|perf|revert)(\(.+\))?!?: .+'
if head -n1 "$1" | grep -qE "$PATTERN"; then
  exit 0
fi
echo "Conventional Commits required: type(scope): message"
echo "Examples: feat: add tray icon | fix(window): resolve close-to-tray | chore: bump deps"
exit 1
