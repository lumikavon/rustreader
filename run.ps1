$ErrorActionPreference = 'Stop'

Set-Location -LiteralPath $PSScriptRoot

$exePath = Join-Path (Join-Path (Join-Path $PSScriptRoot 'target') 'release') 'rustweb.exe'
if (-not (Test-Path -LiteralPath $exePath)) {
  throw "Binary not found: $exePath`nRun build.cmd/build.ps1 first."
}

& $exePath @Args
exit $LASTEXITCODE
