$ErrorActionPreference = 'Stop'

$packageName = 'soma-ai'
$url = 'https://github.com/skbhati199/soma-ai/releases/download/v0.1.0/soma-x86_64-pc-windows-msvc.zip'
$checksum = 'REPLACE_WITH_SHA256'
$checksumType = 'sha256'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

Install-ChocolateyZipPackage -PackageName $packageName `
  -Url $url `
  -UnzipLocation $toolsDir `
  -Checksum $checksum `
  -ChecksumType $checksumType

# Create shim for soma.exe
Install-BinFile -Name 'soma' -Path "$toolsDir\soma.exe"
