#!/usr/bin/env nu
# Author: hustcer
# Created: 2022/04/29 10:06:56
# Description: Script to release setup-nu
#
# TODO:
#   [√] Make sure the release tag does not exist;
#   [√] Make sure there are no uncommit changes;
#   [√] Update change log if required;
#   [√] Create a release tag and push it to the remote repo;
# Usage:
#   Change `actionVer` in package.json and then run: `just release` OR `just release true`

def 'release' [
  --update-log: any  # Set to `true` do enable updating CHANGELOG.md, defined as `any` acutually `bool`
] {

  cd $env.SETUP_MUSD_PATH
  let version = (open cargo.toml | get package.version)
  let releaseVer = $'v($version)'

  if (has-ref $releaseVer) {
  	$'The version ($releaseVer) already exists, Please choose another version.(char nl)'
  	exit --now
  }
  let statusCheck = (git status --porcelain)
  if not ($statusCheck | empty?) {
  	$'You have uncommit changes, please commit them and try `release` again!(char nl)'
  	exit --now
  }
  if ($update-log) {
    git cliff --unreleased --tag $releaseVer --prepend CHANGELOG.md;
    git commit CHANGELOG.md -m $'update CHANGELOG.md for ($releaseVer)'
  }
  # Delete tags that not exist in remote repo
  git fetch origin --prune '+refs/tags/*:refs/tags/*'
  let commitMsg = $'A new release for version: ($releaseVer) created by Release command of musd'
  git tag $releaseVer -am $commitMsg; git push origin --tags
}
