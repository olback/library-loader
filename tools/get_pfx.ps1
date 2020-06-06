# Params
param (
    [Parameter(Mandatory=$true)][string]$pfxpath
)

# Convert Base64 string to bytes
$pfxbytes = [Convert]::FromBase64String($env:pfxb64)

# Write file to disk
[IO.File]::WriteAllBytes("$(pwd)\$pfxpath", $pfxbytes)
