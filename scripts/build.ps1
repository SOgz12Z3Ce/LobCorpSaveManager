$ErrorActionPreference = "Stop"

$LobCorpSaveSerializerSolution = "LobCorpSaveSerializer/LobCorpSaveSerializer.sln"
$ReleaseDir = "release"
$LibDir = "$ReleaseDir/lib"
$BinDir = "$ReleaseDir/bin"

xbuild $LobCorpSaveSerializerSolution

New-Item -ItemType Directory -Force -Path $LibDir, $BinDir | Out-Null
Copy-Item -Recurse -Force `
    "LobCorpSaveSerializer/out/*" `
    $LibDir

@'
@echo off
"%~dp0..\lib\mono.exe" "%~dp0..\lib\lobcss.exe" %*
'@ | Set-Content "$BinDir/lobcss.bat" -Encoding ASCII
