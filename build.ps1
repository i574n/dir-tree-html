param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. deps/spiral/scripts/core.ps1
. deps/spiral/lib/spiral/lib.ps1

if (!(BuildSpiral dir_tree_html.spi dir_tree_html.rs "dir-tree-html")) {
    throw "RUST-FAILED dir-tree-html / compile"
}
{ cargo +nightly-2026-10-18 build --release } | Invoke-Block
$cargoTarget = (cargo +nightly-2026-10-18 metadata --format-version 1 --no-deps | ConvertFrom-Json).target_directory
if (!$cargoTarget) { throw "RUST-FAILED dir-tree-html / cargo metadata found no target directory" }

Remove-Item dist -Recurse -Force -ErrorAction Ignore
New-Item -ItemType Directory -Force -Path dist | Out-Null
Copy-Item -Force "$cargoTarget/release/dir-tree-html$(_exe)" "dist/dir-tree-html$(_exe)"
{ & "dist/dir-tree-html$(_exe)" --self-test } | Invoke-Block
Write-Output "RUST-OK dir-tree-html"
