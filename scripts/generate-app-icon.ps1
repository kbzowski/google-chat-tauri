# Generates a 1024x1024 source PNG used as input for `cargo tauri icon`.
# Output: ../app-icon-source.png (gitignored).
# After running this script, expand the icon set with:
#     cargo tauri icon app-icon-source.png
Add-Type -AssemblyName System.Drawing
$src = Join-Path $PSScriptRoot "..\app-icon-source.png"

$size = 1024
$bmp = New-Object System.Drawing.Bitmap $size, $size
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
$g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAlias
$g.Clear([System.Drawing.Color]::Transparent)

$brush = New-Object System.Drawing.SolidBrush ([System.Drawing.ColorTranslator]::FromHtml("#1A73E8"))
$g.FillEllipse($brush, 32, 32, $size - 64, $size - 64)

$font = New-Object System.Drawing.Font ("Segoe UI", 580, [System.Drawing.FontStyle]::Bold)
$rect = New-Object System.Drawing.RectangleF 0, 0, $size, $size
$sf = New-Object System.Drawing.StringFormat
$sf.Alignment = [System.Drawing.StringAlignment]::Center
$sf.LineAlignment = [System.Drawing.StringAlignment]::Center
$textBrush = New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::White)
$g.DrawString("G", $font, $textBrush, $rect, $sf)

$bmp.Save($src, [System.Drawing.Imaging.ImageFormat]::Png)
$g.Dispose(); $bmp.Dispose(); $brush.Dispose(); $textBrush.Dispose(); $font.Dispose()
Write-Host "Generated source icon at $src"
Write-Host "Run: cargo tauri icon $src"
