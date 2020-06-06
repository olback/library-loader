# Params
param (
    [Parameter(Mandatory=$true)][string]$binpath,
    [Parameter(Mandatory=$true)][string]$pfxpath,
    [Parameter(Mandatory=$true)][string]$password
)

# Path to signtool.exe
$signtool = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.18362.0\x64\signtool.exe"

# Run
& $($signtool) sign /t http://timestamp.digicert.com /f $($pfxpath) /p $($password) $($binpath)
