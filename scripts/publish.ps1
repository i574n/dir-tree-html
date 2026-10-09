param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../deps/spiral/scripts/core.ps1

{ & "../dist/dir-tree-html$(_exe)" --dir ../dist --html ../dist/index.html } | Invoke-Block
