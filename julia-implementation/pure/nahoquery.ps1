# method 1
# julia --project="@." `
#     --startup-file=no `
#     --color=yes `
#     --compile=yes `
#     --optimize=2 `
#     -- src/main.jl $ARGS

# method 2
# https://docs.microsoft.com/zh-tw/powershell/scripting/learn/deep-dives/everything-about-arrays?view=powershell-7.2#null-or-empty
if ( $null -ne $ARGS -and @($ARGS).count -gt 0 )
{
    # Enclose each element with """", e.g. args1 => """"args1""""
    $ARGS = $ARGS | % {-join("`"`"`"`"", $_ , "`"`"`"`"")}

    # Enclose each element with """", e.g. args1 => """"args1""""
    $ArgsList =  "`[$($ARGS -join ',')`]"
    julia --project=""@."" `
    --startup-file=no `
    --color=yes `
    --compile=yes `
    --optimize=2 `
    -e "using NAHOQuery; exit(NAHOQuery.command_main(`"$(`"$ArgsList`")`"))"
}
else
{
    julia --project=""@."" `
    --startup-file=no `
    --color=yes `
    --compile=yes `
    --optimize=2 `
    -e "using NAHOQuery; exit(NAHOQuery.command_main())"
}
