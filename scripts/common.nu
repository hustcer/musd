#!/usr/bin/env nu
# Author: hustcer
# Created: 2021/10/10 07:36:56
# Usage:
#   use source command to load it

# Global date format
# let _DATE_FMT = '%Y.%m.%d'
# let _TIME_FMT = '%Y-%m-%d %H:%M:%S'
# let _UPGRADE_TAG = '$-FORCE-UPGRADE-$'

# All available exit codes:
#   0: Success
#   1: Outdated
#   2: Missing binary
#   3: Missing dependency
#   5: Condition not satisfied
#   6: Server error
#   7: Invalid parameter
#   8: Auth failed

export const _DATE_FMT  = '%Y.%m.%d'
export const _TIME_FMT =  '%Y/%m/%d %H:%M:%S'
export const _UPGRADE_TAG = '$-FORCE-UPGRADE-$'

# Commonly used exit codes
export const ECODE = {
  SUCCESS: 0,
  OUTDATED: 1,
  MISSING_BINARY: 2,
  MISSING_DEPENDENCY: 3,
  CONDITION_NOT_SATISFIED: 5,
  SERVER_ERROR: 6,
  INVALID_PARAMETER: 7,
  AUTH_FAILED: 8,
}

export-env {
  # FIXME: 去除前导空格背景色
  $env.config.color_config.leading_trailing_space_bg = { attr: n }
}

# If current host is Windows
export def windows? [] {
  # Windows / Darwin
  (sys host | get name) == 'Windows'
}

# Check if some command available in current shell
export def is-installed [ app: string ] {
  (which $app | length) > 0
}

# Get the specified env key's value or ''
export def get-env [
  key: string,       # The key to get it's env value
  default?: string,  # The default value for an empty env
] {
  $env | get -i $key | default $default
  # let hasEnv = (env | any { |it| $it.name == $key })
  # if $hasEnv { $env | get $key } else { $default }
}

# Check if a CLI App was installed, if true get the installed version, otherwise return 'N/A'
export def get-ver [
  app: string,     # The CLI App to check
  verCmd: string,  # The Nushell command to get it's version number
] {
  let installed = (which $app | length) > 0
  (if $installed { (nu -c $verCmd | str trim) } else { 'N/A' })
}

# Check if a git repo has the specified ref: could be a branch or tag, etc.
export def has-ref [
  ref: string   # The git ref to check
] {
  let checkRepo = (do -i { git rev-parse --is-inside-work-tree } | complete)
  if not ($checkRepo.stdout =~ 'true') { return false }
  # Brackets were required here, or error will occur
  let parse = (do -i { (git rev-parse --verify -q $ref) })
  if ($parse | is-empty) { false } else { true }
}

# Compare two version number, return `1` if first one is higher than second one,
# Return `0` if they are equal, otherwise return `-1`
export def compare-ver [
  from: string,
  to: string,
] {
  let dest = ($to | str downcase | str trim -c 'v' | str trim)
  let source = ($from | str downcase | str trim -c 'v' | str trim)
  # Ignore '-beta' or '-rc' suffix
  let v1 = ($source | split row '.' | each {|it| ($it | parse -r '(?P<v>\d+)' | get v | get 0 )})
  let v2 = ($dest | split row '.' | each {|it| ($it | parse -r '(?P<v>\d+)' | get v | get 0 )})
  for $v in ($v1 | enumerate) {
    let c1 = ($v1 | get -i $v.index | default 0 | into int)
    let c2 = ($v2 | get -i $v.index | default 0 | into int)
    if $c1 > $c2 {
      return 1
    } else if ($c1 < $c2) {
      return (-1)
    }
  }
  return 0
}

# Compare two version number, return true if first one is lower then second one
export def is-lower-ver [
  from: string,
  to: string,
] {
  (compare-ver $from $to) < 0
}

# Check if git was installed and if current directory is a git repo
export def git-check [
  dest: string,        # The dest dir to check
  --check-repo: int,   # Check if current directory is a git repo
] {
  cd $dest
  let isGitInstalled = (which git | length) > 0
  if (not $isGitInstalled) {
    print $'You should (ansi r)INSTALL git(ansi reset) first to run this command, bye...'
    exit $ECODE.MISSING_BINARY
  }
  # If we don't need repo check just quit now
  if ($check_repo != 0) {
    let checkRepo = (do -i { git rev-parse --is-inside-work-tree } | complete)
    if not ($checkRepo.stdout =~ 'true') {
      print $'Current directory is (ansi r)NOT(ansi reset) a git repo, bye...(char nl)'
      exit $ECODE.CONDITION_NOT_SATISFIED
    }
  }
}

# Create a line by repeating the unit with specified times
def build-line [
  times: int,
  unit: string = '-',
] {
  0..<$times | reduce -f '' { |i, acc| $unit + $acc }
}

# Log some variables
export def log [
  name: string,
  var: any,
] {
  print $'(ansi g)(build-line 18)> Debug Begin: ($name) <(build-line 18)(ansi reset)'
  print $var
  print $'(ansi g)(build-line 20)>  Debug End <(build-line 20)(char nl)(ansi reset)'
}

export def hr-line [
  width?: int = 90,
  --blank-line(-b),
  --with-arrow(-a),
  --color(-c): string = 'g',
] {
  print $'(ansi $color)(build-line $width)(if $with_arrow {'>'})(ansi reset)'
  if $blank_line { print -n (char nl) }
}

# parallel { print "Oh" } { print "Ah" } { print "Eeh" }
export def parallel [...closures] {
  $closures | par-each {
    |c| do $c
  }
}

# Display a progress bar with specified length
export def progress [
  count: int,               # Total tick count of the progress bar
  interval: float = 1.0,    # The interval between each tick
  --char(-c): string = '█', # The char to display for each tick
] {
  mut x = 0
  let duration = $'($interval)sec' | into duration
  # Available chars: █ ▓ ▒ ░ = - ~ *
  while $x < $count { print -n $char; $x = $x + 1; sleep $duration }
}
