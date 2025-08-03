Add-Type -AssemblyName System.Drawing

# Create a bitmap
$bitmap = New-Object System.Drawing.Bitmap(32, 32)
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)

# Fill with blue background
$blueBrush = [System.Drawing.Brushes]::DodgerBlue
$graphics.FillRectangle($blueBrush, 0, 0, 32, 32)

# Add white text "SC"
$font = New-Object System.Drawing.Font("Arial", 12, [System.Drawing.FontStyle]::Bold)
$whiteBrush = [System.Drawing.Brushes]::White
$graphics.DrawString("SC", $font, $whiteBrush, 4, 8)

# Save as PNG first
$bitmap.Save("icon.png", [System.Drawing.Imaging.ImageFormat]::Png)

# Convert PNG to ICO using magick if available, otherwise use online converter instruction
try {
    & magick icon.png -resize 32x32 icon.ico
    Write-Host "Icon created successfully at: $(Get-Location)\icon.ico"
} catch {
    Write-Host "ImageMagick not found. Saving PNG instead. Please convert to ICO format manually."
    Write-Host "PNG saved at: $(Get-Location)\icon.png"
    
    # Try to create a basic ICO manually
    $icoBytes = @()
    
    # ICO header (6 bytes)
    $icoBytes += 0x00, 0x00  # Reserved, must be 0
    $icoBytes += 0x01, 0x00  # Type: 1 for ICO
    $icoBytes += 0x01, 0x00  # Number of images: 1
    
    # Image directory entry (16 bytes)
    $icoBytes += 0x20        # Width: 32 pixels
    $icoBytes += 0x20        # Height: 32 pixels
    $icoBytes += 0x00        # Color palette: 0 for no palette
    $icoBytes += 0x00        # Reserved: 0
    $icoBytes += 0x01, 0x00  # Color planes: 1
    $icoBytes += 0x20, 0x00  # Bits per pixel: 32
    
    # For simplicity, we'll create a minimal ICO
    # This is complex, so let's use a different approach
    Write-Host "Creating minimal ICO file..."
    
    # Simple approach: convert bitmap to byte array and wrap in ICO format
    $pngData = [System.IO.File]::ReadAllBytes("icon.png")
    $iconData = @(0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x20, 0x20, 0x00, 0x00, 0x01, 0x00, 0x20, 0x00) + 
                [BitConverter]::GetBytes($pngData.Length) + 
                [BitConverter]::GetBytes(22) + 
                $pngData
    
    [System.IO.File]::WriteAllBytes("icon.ico", $iconData)
    Write-Host "Basic ICO created at: $(Get-Location)\icon.ico"
}

# Cleanup
$graphics.Dispose()
$bitmap.Dispose()
