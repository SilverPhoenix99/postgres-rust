
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

function Get-LlvmTypes {

    $ErrorActionPreference = 'Stop'

    cargo +beta rustc -- --emit=llvm-ir -Awarnings

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

function Parse-LlvmType([string] $LlvmType) {

    $ErrorActionPreference = 'Stop'

    $stream = $LlvmType -split '(\(\*\)|[}),])|[({]' | ? { -not [string]::IsNullOrWhiteSpace($_) }

    $nodes = [Collections.Stack]::new()

    foreach ($element in $stream) {

        switch -Exact ($element) {

            {$_ -in @('}', '},', ')', '),', ',')} {

                $node = $nodes.Pop()
                $node = [Collections.DictionaryEntry]::new(
                    $node.Name,
                    $node.Elements.ToArray()
                )

                if ($nodes.Count -gt 0) {
                    $nodes.Peek().Elements.Add($node) > $null
                }

                break
            }

            '(*)' {

                # C-style function pointer: "ret (*)(arg, ...)"
                # Output format: "(*)(ret, arg, ...)"

                # Return type
                $return = $nodes.Pop()
                $return = [Collections.DictionaryEntry]::new(
                    $return.Name,
                    $return.Elements.ToArray()
                )

                $nodes.Push([PSCustomObject] @{
                    Name = $_
                    Elements = [Collections.ArrayList] @($return)
                })

                break
            }

            Default {
                $nodes.Push([PSCustomObject] @{
                    Name = $_
                    Elements = [Collections.ArrayList] @()
                })

                break
            }
        }
    }

    $nodes.ToArray() | % {
        [Collections.DictionaryEntry]::new(
            $_.Name,
            $_.Elements.ToArray()
        )
    }

    if ($nodes.Count -ne 1) {
        throw 'Parsing error: Stack should contain exactly one element at the end.'
    }
}

function Format-LlvmTree([Collections.DictionaryEntry] $Node) {

    $ErrorActionPreference = 'Stop'

    switch ($Node.Name) {

        'enum2' {

            $children = $Node.Value | % { Format-LlvmTree $_ ; ',' }
            $children = $children[0..($children.Length - 2)]

            return $children
        }

        'tuple' {

            $children = $Node.Value | % { Format-LlvmTree $_ ; ',' }
            $children = $children[0..($children.Length - 2)]
            $children = $children | % { "  $_" }

            return @(
                '('
                $children
                ')'
            )
        }

        '(*)' {

            $ret = $Node.Value[0] | % { Format-LlvmTree $_ } | % { "  $_" }

            $arguments = $Node.Value[1..($Node.Value.Length)] | % { Format-LlvmTree $_; ',' }
            $arguments = $arguments[0..($arguments.Length - 2)]
            $arguments = $arguments | % { "  $_" }

            return @(
                'impl Fn('
                $arguments
                ')'
                '->'
                $ret
            )
        }

        'ref_mut' {

            $Node = $Node.Value[0]

            $typeName = $Node.Name
            $typeName = "&mut $typeName"

            if ($child.Value.Length -eq 0) {
                return $typeName
            }

            $children = $Node.Value | % { Format-LlvmTree $_ ; ',' }
            $children = $children[0..($children.Length - 2)]
            $children = $children | % { "  $_" }

            return @(
                "$typeName<"
                $children
                '>'
            )
        }

        Default {

            if ($Node.Value.Length -eq 0) {
                return $_
            }

            $children = $Node.Value | % { Format-LlvmTree $_ ; ',' }
            $children = $children[0..($children.Length - 2)]
            $children = $children | % { "  $_" }

            return @(
                "${_}<"
                $children
                '>'
            )
        }
    }
}

<#

($x = ParseTypes)[0..10] | Format-Table ; $x[0].Name | Indent | Set-Clipboard
($x = ParseTypes)[0..10] | Format-Table ; $x[0].Name | Indent | code -


. ..\..\scripts\check-Rust-type-lengths.ps1

$llvmTypes = Get-LlvmTypes
$tree = Parse-LlvmType $llvmTypes[0].Name


Format-LlvmTree $tree | code -

# or

Format-LlvmTree $tree | Set-Clipboard

#>

