param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"


$url = git ls-remote --get-url
$owner = ($url -split '/' | Select-Object -Last 2 | Select-Object -First 1) -replace '\.git$', '' ?? $env:GITHUB_REPOSITORY_OWNER
$domain = ($url -split '/' | Select-Object -Last 3 | Select-Object -First 1) ?? $env:GITHUB_SERVER_URL -replace 'https?://', ''
Write-Output "init.ps1 / url: $url / owner: $owner / domain: $domain"

if (!(Test-Path "../../spiral/scripts/core.ps1")) {
    git clone https://$domain/$owner/spiral.git ../../spiral
    if ($LASTEXITCODE -ne 0) { throw "init.ps1 / git clone spiral failed (exit code $LASTEXITCODE)" }
}

. ../../spiral/scripts/core.ps1

EnsureSymbolicLink -Path "../deps/spiral" -Target "../../spiral"

{ pwsh ../deps/spiral/scripts/init-app-build.ps1 -fast $($fast ?? '') } | Invoke-Block
