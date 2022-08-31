#!/bin/sh

cd /app

./action-trello-connector --version

./action-trello-connector --trello-board-id "$TRELLO_BOARD_ID" \
  --gh-event-name "$GH_EVENT_NAME" \
  --gh-repository-name "$GH_REPOSITORY_NAME" \
  --gh-pr-num "$GH_PR_NUM" \
  --gh-pr-url "$GH_PR_URL" \
  --gh-pr-title "$GH_PR_TITLE" \
  --gh-pr-body "$GH_PR_BODY" \
  --gh-pr-branch-name "$GH_PR_BRANCH_NAME" \
  --gh-push-branch-name "$GH_PUSH_BRANCH_NAME"
