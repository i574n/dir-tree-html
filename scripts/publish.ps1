param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../deps/spiral/scripts/core.ps1

{ pwsh ../deps/spiral/scripts/publish-tree.ps1 -Root .. -Tool "../dist/dir-tree-html$(_exe)" } | Invoke-Block
