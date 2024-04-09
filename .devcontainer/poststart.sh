#!/bin/bash
git config --global user.email "${GIT_USER_NAME}@illusioncolors.com" 
git config --global user.name "${GIT_USER_NAME}"
git config --global --add safe.directory /ws/asset_server
echo "$GOOGLE_APPLICATION_CREDENTIALS_BASE64" | base64 -d > /google-secret.json
gcloud auth activate-service-account --key-file=/google-secret.json
gcloud config set project matr1x-fire-development