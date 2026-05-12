# One-off generator for placeholder tray icons (32x32 PNG).
# Run when icons need regenerating. Output is committed under src-tauri/icons/tray/.
Add-Type -AssemblyName System.Drawing
$dir = Join-Path $PSScriptRoot "..\src-tauri\icons\tray"
New-Item -ItemType Directory -Force $dir | Out-Null

function Save-Circle($path, $colorHex) {
    $bmp = New-Object System.Drawing.Bitmap 32, 32
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.Clear([System.Drawing.Color]::Transparent)
    $brush = New-Object System.Drawing.SolidBrush ([System.Drawing.ColorTranslator]::FromHtml($colorHex))
    $g.FillEllipse($brush, 2, 2, 28, 28)
    $bmp.Save($path, [System.Drawing.Imaging.ImageFormat]::Png)
    $g.Dispose(); $bmp.Dispose(); $brush.Dispose()
}

Save-Circle (Join-Path $dir "normal.png")  "#1A73E8"  # Google blue
Save-Circle (Join-Path $dir "badge.png")   "#EA4335"  # Google red
Save-Circle (Join-Path $dir "offline.png") "#9AA0A6"  # Google grey
Write-Host "Generated tray icons in $dir"
