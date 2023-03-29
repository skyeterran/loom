# 🧶 Loom
Loom weaves itself! I will write a more helpful page later!

## How 2 run
Get Rust. Run `cargo run --example repl` from its directory. Have fun!

## How 2 write
### set
`set` defines a global variable.
```
(set message "Hello, world!")
```
### print
`print` will show the value (in Loom code) on-screen.
```
(print message)
```
### if
`if` will return one thing if the first argument isn't nil, and optionally return another thing if it is nil.
```
(if (= temperature #hot)
    (set sweating true)
    (set sweating nil))
```

### match
`match` will compare the first argument with the heads of the following pairs, and return the tail of the first matching one.
```
(match temperature
    (#hot "It's boiling in here!")
    (#mild "It's room-temperature, I guess?")
    (#cold "It's freezing in here!")
```
### env
`env` will debug the entire running Loom environment.
### list
`list` will create a list of values from its arguments. Square brackets are valid shorthand:
```
(list 1 2 3 4)
[1 2 3 4]
```

### table
`table` will combine data into a group of key-value pairs. Curly braces are valid shorthand:
```
(set kitty (table
    #species #cat
    #name "Caesar"
    #stats (table
        #str 17
        #dex 19
        #int 4
    ))
))
(set kitty {
    #species #cat
    #name "Caesar"
    #stats {
        #str 17
        #dex 19
        #int 4
    })
})
```

### get
`get` can retrieve values from deeply nested tables. Dot-separated keys are valid shorthand. In this case, we can get the cat's `int` value out of two nested tables:
```
(get kitty stats int)
kitty.stats.int
```

### save
`save` will export a value to Loom code in a file.
```
(set fruits [#apple #orange #banana #melon])
(save fruits "fruits.loom")
```

### load
`load` will load a value from Loom code in a file. If you give it a symbol as the second argument, it will set the symbol to the loaded value:
```
(load "fruits.loom")
(load "fruits.loom" fruits)
```

### run
`run` will run a file as a script.
```
(run "script.loom")
```
