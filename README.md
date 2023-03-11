# 🧶 Loom
Loom weaves itself! I will write a more helpful page later!

## How 2 run
Get Rust. Run `cargo run --example repl` from its directory. Have fun!

## How 2 write
### let
`let` defines a global variable.
```
(let message "Hello, world!")
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
    (let sweating true)
    (let sweating false))
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

### object
`object` will create combine data into a group of key-value pairs.
```
(let kitty (object
    (species #cat)
    (name "Caesar")
    (stats (object
        (str 17)
        (dex 19)
        (int 4)))))
```

### get
`get` can retrieve values from deeply nested objects. Dot-separated keys are valid shorthand. In this case, we can get the cat's `int` value out of two nested objects:
```
(get kitty stats int)
kitty.stats.int
```

### write
`write` will export a value to Loom code in a file.
```
(write "fruits.loom" [#apple #orange #banana #melon])
```

### load
`load` will load a value from Loom code in a file.
```
(load "fruits.loom")
```

### run
`run` will run a file as a script.
```
(run "script.loom")
```
