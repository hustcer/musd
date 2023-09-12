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

export const _DATE_FMT  = '%Y.%m.%d'
export const _TIME_FMT =  '%Y-%m-%d %H:%M:%S'
export const _UPGRADE_TAG = '$-FORCE-UPGRADE-$'

# Termix.toml config file path
export def get-termix-conf [] { ([$env.TERMIX_DIR 'termix.toml'] | path join) }

# If current host is Windows
export def windows? [] {
  # Windows / Darwin
  (sys).host.name == 'Windows'
}

# Get the specified env key's value or ''
export def 'get-env' [
  key: string       # The key to get it's env value
  default?: string  # The default value for an empty env
] {
  $env | get -i $key | default $default
  # let hasEnv = (env | any { |it| $it.name == $key })
  # if $hasEnv { $env | get $key } else { $default }
}

# Get the specified config from `termix.toml` by key
export def 'get-conf' [
  key: string       # The key to get it's value from termix.toml
  default?: any     # The default value for an empty conf
] {
  let _TERMIX_CONF = get-termix-conf
  let result = (open $_TERMIX_CONF | get $key)
  if ($result | is-empty) { $default } else { $result }
}

# Get TERMIX_TMP_PATH
export def 'get-tmp-path' [] {
  let _TERMIX_CONF = get-termix-conf
  let actionConf = (open $_TERMIX_CONF)
  # 先从环境变量里面查找临时文件路径
  let tmpDir = (get-env TERMIX_TMP_PATH '')
  let tmpPath = if ($tmpDir | is-empty) { ($actionConf | get termixTmpPath) } else { $tmpDir }
  if not ($tmpPath | path exists) {
    print $'(ansi r)Path ($tmpPath) does not exist, please create it and try again...(ansi reset)(char nl)(char nl)'
    exit 3
  }
  echo $tmpPath
}

# Check if a CLI App was installed, if true get the installed version, otherwise return 'N/A'
export def 'get-ver' [
  app: string     # The CLI App to check
  verCmd: string  # The Nushell command to get it's version number
] {
  let installed = (which $app | length) > 0
  echo (if $installed { (nu -c $verCmd | str trim) } else { 'N/A' })
}

# Check if a git repo has the specified ref: could be a branch or tag, etc.
export def 'has-ref' [
  ref: string   # The git ref to check
] {
  let checkRepo = (do -i { git rev-parse --is-inside-work-tree } | complete)
  if not ($checkRepo.stdout =~ 'true') { return false }
  # Brackets were required here, or error will occur
  let parse = (do -i { (git rev-parse --verify -q $ref) })
  if ($parse | is-empty) { false } else { true }
}

# Compare two version number, return `true` if first one is higher than second one,
# Return `null` if they are equal, otherwise return `false`
export def 'compare-ver' [
  from: string,
  to: string,
] {
  let dest = ($to | str downcase | str trim -c 'v' | str trim)
  let source = ($from | str downcase | str trim -c 'v' | str trim)
  # Ignore '-beta' or '-rc' suffix
  let v1 = ($source | split row '.' | each {|it| ($it | parse -r '(?P<v>\d+)' | get v | get 0 )})
  let v2 = ($dest | split row '.' | each {|it| ($it | parse -r '(?P<v>\d+)' | get v | get 0 )})
  for $v in $v1 -n {
    let c1 = ($v1 | get -i $v.index | default 0 | into int)
    let c2 = ($v2 | get -i $v.index | default 0 | into int)
    if $c1 > $c2 {
      return true
    } else if ($c1 < $c2) {
      return false
    }
  }
  return null
}

# Compare two version number, return true if first one is lower then second one
export def 'is-lower-ver' [
  from: string,
  to: string,
] {
  (compare-ver $from $to) == false
}

# Check if git was installed and if current directory is a git repo
export def 'git-check' [
  dest: string        # The dest dir to check
  --check-repo: int   # Check if current directory is a git repo
] {
  cd $dest
  let isGitInstalled = (which git | length) > 0
  if (not $isGitInstalled) {
    print $'You should (ansi r)INSTALL git(ansi reset) first to run this command, bye...'
    exit 2
  }
  # If we don't need repo check just quit now
  if ($check_repo != 0) {
    let checkRepo = (do -i { git rev-parse --is-inside-work-tree } | complete)
    if not ($checkRepo.stdout =~ 'true') {
      print $'Current directory is (ansi r)NOT(ansi reset) a git repo, bye...(char nl)'
      exit 5
    }
  }
}

# Log some variables
export def 'log' [
  name: string
  var: any
] {
  print $'(ansi g)('─' * 18)> Debug Begin: ($name) <('─' * 18)(ansi reset)'
  print $var
  print $'(ansi g)('─' * 20)>  Debug End <('─' * 20)(char nl)(ansi reset)'
}

export def 'hr-line' [
  width?: int = 90,
  --color(-c): string = 'g',
  --blank-line(-b): bool,
  --with-arrow(-a): bool,
] {
  print $'(ansi $color)('─' * $width)(if $with_arrow {'>'})(ansi reset)'
  if $blank_line { char nl }
}

# parallel { print "Oh" } { print "Ah" } { print "Eeh" }
export def parallel [...closures] {
  $closures | par-each {
    |c| do $c
  }
}
