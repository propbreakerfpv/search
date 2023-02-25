# search
a simpl cli tool to search through fils

build the project 
`cargo build`

to realy use this you need to run the executable file directly because cargo thinks you passing it paramaters so to run it do.
`./target/debug/search [your paramaters]`

index a directory with
`./target/debug/search --index [path to dir]`

search a indexed dir
`./target/debug/search [dir] [search]`

list all indexed dirs 
`./target/debug/search --list`

all those commands should work but theres also a large posibility that it just doesnt work.
