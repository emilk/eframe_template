$crate = Read-Host "To fill the template, tell me your egui project crate name: "
$name = Read-Host "To fill the template, tell me your name (for author in Cargo.toml): "
$email = Read-Host "To fill the template, tell me your e-mail address (also for Cargo.toml): "
$package = Read-Host "To fill the template tell me your android package name: "


Write-Host "Patching files..."

(Get-Content "Cargo.toml") -replace "eframe_template", $crate | Set-Content "Cargo.toml"
(Get-Content "src\main.rs") -replace "eframe_template", $crate | Set-Content "src\main.rs"
(Get-Content "index.html") -replace "eframe template", $crate -replace "eframe_template", $crate | Set-Content "index.html"
(Get-Content "assets\sw.js") -replace "eframe_template", $crate | Set-Content "assets\sw.js"
(Get-Content "Cargo.toml") -replace "Emil Ernerfeldt", $name -replace "emil.ernerfeldt@gmail.com", $email | Set-Content "Cargo.toml"

(Get-Content "kotlin\MainActivity.kt") -replace "eframe_template", $crate | Set-Content  "kotlin\MainActivity.kt"
(Get-Content "kotlin\MainActivity.kt") -replace "rs.egui.$crate", $package | Set-Content  "kotlin\MainActivity.kt"
(Get-Content "manifest.yaml") -replace "rs.egui.eframe_template", $package | Set-Content  "manifest.yaml"

Write-Host "Done."
