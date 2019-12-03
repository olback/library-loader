# .\test.ps1 -PfxPath .\yeet.pfx -Password yeet

# Params
param (
    [Parameter(Mandatory=$true)][string]$pfxpath,
    [Parameter(Mandatory=$true)][string]$password
)

# Date
$date_str = (date).ToString("yyyy-MM-dd")

# Password
$passwd = ConvertTo-SecureString -String $password -Force -AsPlainText

# Certificate
$crt = New-SelfSignedCertificate -Type Custom -Subject "CN=Edwin Svensson, O=Edwin Svensson, C=SE" -KeyUsage DigitalSignature -FriendlyName "Library Loader $($date_str)" -CertStoreLocation "Cert:\CurrentUser\My" -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3", "2.5.29.19={text}") -KeyAlgorithm RSA -KeyLength 4096 -KeyDescription "Library Loader Signing Certificate"

# Export pfx
$crt | Export-PfxCertificate -FilePath $pfxpath -Password $passwd
