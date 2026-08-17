$ErrorActionPreference = 'Stop'

$repositoryRoot = $PSScriptRoot
python (Join-Path $repositoryRoot 'tools\verify.py')
if ($LASTEXITCODE -ne 0) {
    throw 'La verificación ha fallado. Consulte VERIFICATION.md.'
}
