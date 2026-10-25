---
name: filesystem
triggers: [file, folder, directory, zip, unzip, copy, move, rename, delete, remove, size, find, large, tree, read, tail, head, replace, lines, count, duplicate, empty, create]
---
FILESYSTEM SKILL
- Use -Filter for one pattern, -Include for several. Always add -File or -Directory.
- Before wide deletes use the exact path the user gave. Never delete outside it.

Request: list all files recursively
PowerShell Command:
Get-ChildItem -Recurse -File

Request: find all .js files excluding node_modules
PowerShell Command:
Get-ChildItem -Recurse -File -Filter '*.js' | Where-Object { $_.FullName -notmatch 'node_modules' }

Request: find files named config
PowerShell Command:
Get-ChildItem -Recurse -File -Filter '*config*'

Request: show 10 largest files
PowerShell Command:
Get-ChildItem -Recurse -File | Sort-Object Length -Descending | Select-Object -First 10 FullName, @{n='MB';e={[math]::Round($_.Length/1MB,2)}}

Request: files larger than 100 MB
PowerShell Command:
Get-ChildItem -Recurse -File | Where-Object { $_.Length -gt 100MB } | Select-Object FullName, @{n='MB';e={[math]::Round($_.Length/1MB)}}

Request: files modified in last 24 hours
PowerShell Command:
Get-ChildItem -Recurse -File | Where-Object { $_.LastWriteTime -gt (Get-Date).AddDays(-1) }

Request: size of each folder in current directory
PowerShell Command:
Get-ChildItem -Directory | ForEach-Object { [pscustomobject]@{ Folder = $_.Name; MB = [math]::Round((Get-ChildItem $_.FullName -Recurse -File -ErrorAction SilentlyContinue | Measure-Object Length -Sum).Sum / 1MB, 2) } } | Sort-Object MB -Descending

Request: delete all node_modules folders
PowerShell Command:
Get-ChildItem -Recurse -Directory -Filter 'node_modules' | Remove-Item -Recurse -Force

Request: delete bin and obj folders
PowerShell Command:
Get-ChildItem -Recurse -Directory | Where-Object { $_.Name -in 'bin','obj' } | Remove-Item -Recurse -Force

Request: delete all .log files older than 7 days
PowerShell Command:
Get-ChildItem -Recurse -File -Filter '*.log' | Where-Object { $_.LastWriteTime -lt (Get-Date).AddDays(-7) } | Remove-Item -Force

Request: delete empty folders
PowerShell Command:
Get-ChildItem -Recurse -Directory | Where-Object { -not (Get-ChildItem $_.FullName -Force) } | Remove-Item

Request: zip folder dist
PowerShell Command:
Compress-Archive -Path '.\dist' -DestinationPath '.\dist.zip' -Force

Request: unzip file app.zip into folder app
PowerShell Command:
Expand-Archive -Path '.\app.zip' -DestinationPath '.\app' -Force

Request: copy folder src to backup
PowerShell Command:
Copy-Item -Path '.\src' -Destination '.\backup' -Recurse

Request: move all .png files to images folder
PowerShell Command:
New-Item -ItemType Directory -Path '.\images' -Force | Out-Null; Move-Item -Path '.\*.png' -Destination '.\images'

Request: rename file old.txt to new.txt
PowerShell Command:
Rename-Item -Path '.\old.txt' -NewName 'new.txt'

Request: rename all .jpeg files to .jpg
PowerShell Command:
Get-ChildItem -File -Filter '*.jpeg' | Rename-Item -NewName { $_.Name -replace '\.jpeg$', '.jpg' }

Request: create folder structure src/components
PowerShell Command:
New-Item -ItemType Directory -Path '.\src\components' -Force

Request: create empty file index.js
PowerShell Command:
New-Item -ItemType File -Path '.\index.js' -Force

Request: show first 20 lines of file app.log
PowerShell Command:
Get-Content .\app.log -TotalCount 20

Request: show last 50 lines of file app.log
PowerShell Command:
Get-Content .\app.log -Tail 50

Request: follow log file live
PowerShell Command:
Get-Content .\app.log -Tail 50 -Wait

Request: search text TODO in all source files
PowerShell Command:
Get-ChildItem -Recurse -File -Include '*.js','*.ts','*.py','*.java' | Select-String -Pattern 'TODO'

Request: search ERROR in log files and show only file names
PowerShell Command:
Get-ChildItem -Recurse -File -Filter '*.log' | Select-String 'ERROR' | Select-Object -ExpandProperty Path -Unique

Request: replace oldtext with newtext in all .txt files
PowerShell Command:
Get-ChildItem -Recurse -File -Filter '*.txt' | ForEach-Object { (Get-Content $_.FullName -Raw) -replace 'oldtext', 'newtext' | Set-Content $_.FullName }

Request: count lines of code in all .js files
PowerShell Command:
(Get-ChildItem -Recurse -File -Filter '*.js' | Where-Object { $_.FullName -notmatch 'node_modules' } | Get-Content | Measure-Object -Line).Lines

Request: count files by extension
PowerShell Command:
Get-ChildItem -Recurse -File | Group-Object Extension | Sort-Object Count -Descending | Select-Object Count, Name

Request: find duplicate files by hash
PowerShell Command:
Get-ChildItem -Recurse -File | Get-FileHash | Group-Object Hash | Where-Object { $_.Count -gt 1 } | ForEach-Object { $_.Group.Path }

Request: check if file exists package.json
PowerShell Command:
Test-Path '.\package.json'

Request: show folder tree
PowerShell Command:
tree /F

Request: open current folder in explorer
PowerShell Command:
Invoke-Item .

Request: open current folder in vs code
PowerShell Command:
code .