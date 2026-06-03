$WshShell = New-Object -comObject WScript.Shell
$DesktopPath = [Environment]::GetFolderPath("Desktop")
$Shortcut = $WshShell.CreateShortcut("$DesktopPath\AlgoMentor.lnk")
$Shortcut.TargetPath = "$PWD\target\release\algomentor-gui.exe"
$Shortcut.WorkingDirectory = "$PWD\target\release"
$Shortcut.Description = "AlgoMentor IDE"
$Shortcut.Save()

Write-Host "Desktop shortcut created successfully!"
