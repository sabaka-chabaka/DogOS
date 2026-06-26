$Source = "D:\DogOS"
$Dest = "\\wsl.localhost\Ubuntu\home\$env:USERNAME\projects\DogOS"

robocopy $Source $Dest /MIR `
    /XD target build .git .idea .vscode `
    /XF *.pdb

if ($LASTEXITCODE -gt 7) {
    throw "Robocopy failed with exit code $LASTEXITCODE"
}

wsl -d Ubuntu bash -lc "cd ~/projects/DogOS && make run"