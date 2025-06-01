
# Make sure this runs in the <repo-root>/parser/parser directory.

# Parser combinators can generate long Rust type names.
# This is an ad-hoc script to check them.
# Copy the functions into PowerShell, and then run one of the commands at the bottom of this script.

function Indent {

    param(
        [Parameter(ValueFromPipeline, Position = 0)]
        [string] $Value
    )

    $text = $Value -split '(\},?|\{|,)' `
        | ForEach-Object {$indent = 0} {

            switch -exact ($_) {
                '' { break }
                '{' {
                    $indent += 1
                    " $_`n"
                    break
                }
                '},' {
                    $indent -= 1
                    "`n$('  ' * $indent)$_`n"
                    break
                }
                '}' {
                    $indent -= 1
                    "`n$('  ' * $indent)$_`n"
                    break
                }
                ',' {
                    "$_`n"
                    break
                }
                Default {
                    "$('  ' * $indent)$_"
                    break
                }
            }
        }

    $text -join '' -split "`n" `
        | Where-Object {$_} # Remove empty lines
}

function ParseTypes {

    $ErrorActionPreference = 'Stop'

    cargo rustc -- --emit=llvm-ir -Awarnings

    if (!$Global:target_directory) {
        $j = cargo metadata --format-version=1 --no-deps | ConvertFrom-Json
        $Global:target_directory = $j.target_directory
    }

    Get-ChildItem "$($Global:target_directory)/debug/deps/*.ll" `
        | Sort-Object LastWriteTime -Descending `
        | Select-Object -First 1 `
        | Get-Content `
        | Select-String 'DISubprogram' -Raw `
        | Select-String 'name: "([^"]+)"' `
        | ForEach-Object { $_.Matches.Groups[1].Value } `
        | Group-Object {$_} -NoElement `
        | ForEach-Object {&{
            $simplified = $_.Name `
                -replace '(^|[<[(, ])(\w+?::)+','$1' `
                -replace ',Global|Combi\b' `
                -replace 'enum2\$<\s*(\w+)\s*>','$1' `
                -replace '\s*\$?<','{' `
                -replace '\s*\$?>','}'

            [PSCustomObject]@{
                Count = $_.Count
                Length = $_.Name.Length
                Name = $simplified
            }
        }} `
        | Sort-Object Length -Descending
}

($x = ParseTypes)[0..10] | Format-Table ; $x[0].Name | Indent | Set-Clipboard

($x = ParseTypes)[0..10] | Format-Table ; $x[0].Name | Indent | code -
