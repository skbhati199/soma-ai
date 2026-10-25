---
name: powershell
triggers: [port, process, kill, service, env, environment, ip, network, disk, hash, json, path, install, version, uptime, memory, cpu]
---
POWERSHELL SYSTEM SKILL
- Use native cmdlets. Variables like $_ must stay exactly as written.

Request: kill whatever is using port 3000
PowerShell Command:
Get-NetTCPConnection -LocalPort 3000 -ErrorAction SilentlyContinue | ForEach-Object { Stop-Process -Id $_.OwningProcess -Force }

Request: which process is using port 8080
PowerShell Command:
Get-NetTCPConnection -LocalPort 8080 | ForEach-Object { Get-Process -Id $_.OwningProcess } | Select-Object Id, ProcessName, Path

Request: list all listening ports
PowerShell Command:
Get-NetTCPConnection -State Listen | Sort-Object LocalPort | Select-Object LocalAddress, LocalPort, OwningProcess

Request: kill all node processes
PowerShell Command:
Get-Process node -ErrorAction SilentlyContinue | Stop-Process -Force

Request: top 10 processes by memory
PowerShell Command:
Get-Process | Sort-Object WorkingSet64 -Descending | Select-Object -First 10 Name, Id, @{n='MB';e={[math]::Round($_.WorkingSet64/1MB)}}

Request: top 10 processes by cpu
PowerShell Command:
Get-Process | Sort-Object CPU -Descending | Select-Object -First 10 Name, Id, CPU

Request: list running services
PowerShell Command:
Get-Service | Where-Object { $_.Status -eq 'Running' }

Request: restart service postgresql
PowerShell Command:
Restart-Service -Name 'postgresql*' -Force

Request: show environment variable PATH one per line
PowerShell Command:
$env:PATH -split ';'

Request: set environment variable API_KEY for current session
PowerShell Command:
$env:API_KEY = 'value'

Request: set environment variable API_KEY permanently
PowerShell Command:
[Environment]::SetEnvironmentVariable('API_KEY', 'value', 'User')

Request: add C:\tools to PATH permanently
PowerShell Command:
[Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path','User') + ';C:\tools', 'User')

Request: find where a command is installed like node
PowerShell Command:
Get-Command node | Select-Object -ExpandProperty Source

Request: show my public ip
PowerShell Command:
Invoke-RestMethod -Uri 'https://api.ipify.org'

Request: show my local ip addresses
PowerShell Command:
Get-NetIPAddress -AddressFamily IPv4 | Where-Object { $_.IPAddress -ne '127.0.0.1' } | Select-Object InterfaceAlias, IPAddress

Request: test if a host port is open example.com 443
PowerShell Command:
Test-NetConnection -ComputerName example.com -Port 443

Request: ping google 4 times
PowerShell Command:
Test-Connection -TargetName google.com -Count 4

Request: flush dns cache
PowerShell Command:
Clear-DnsClientCache

Request: download a file from url to current folder
PowerShell Command:
Invoke-WebRequest -Uri 'https://example.com/file.zip' -OutFile '.\file.zip'

Request: call api and show json response
PowerShell Command:
Invoke-RestMethod -Uri 'https://api.example.com/data' | ConvertTo-Json -Depth 10

Request: post json to api
PowerShell Command:
Invoke-RestMethod -Method Post -Uri 'https://api.example.com/items' -ContentType 'application/json' -Body '{"name":"test"}'

Request: pretty print json file data.json
PowerShell Command:
Get-Content .\data.json -Raw | ConvertFrom-Json | ConvertTo-Json -Depth 20

Request: post json to api
PowerShell Command:
Invoke-RestMethod -Method Post -Uri 'https://api.example.com/items' -ContentType 'application/json' -Body '{"name":"test"}'

Request: pretty print json file data.json
PowerShell Command:
Get-Content .\data.json -Raw | ConvertFrom-Json | ConvertTo-Json -Depth 20

Request: sha256 hash of file app.zip
PowerShell Command:
Get-FileHash .\app.zip -Algorithm SHA256

Request: show free disk space of all drives
PowerShell Command:
Get-PSDrive -PSProvider FileSystem | Select-Object Name, @{n='FreeGB';e={[math]::Round($_.Free/1GB,1)}}, @{n='UsedGB';e={[math]::Round($_.Used/1GB,1)}}

Request: show windows uptime
PowerShell Command:
(Get-Date) - (Get-CimInstance Win32_OperatingSystem).LastBootUpTime

Request: show node npm git and docker versions
PowerShell Command:
node -v; npm -v; git --version; docker --version

Request: base64 encode text hello
PowerShell Command:
[Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes('hello'))

Request: generate a random guid
PowerShell Command:
[guid]::NewGuid().ToString()

Request: generate random 32 character password
PowerShell Command:
-join ((48..57)+(65..90)+(97..122) | Get-Random -Count 32 | ForEach-Object { [char]$_ })

Request: copy last command output to clipboard
PowerShell Command:
Get-Process | Set-Clipboard

Request: run a command every 5 seconds
PowerShell Command:
while ($true) { docker ps; Start-Sleep -Seconds 5 }