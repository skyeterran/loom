# Loom: A modern lisp 🧶
Loom is a purely functional, compiled Lisp based on Scheme.

## Features
- Multiple return values

## Example
```
(def (run logic)
  (def (tick last-tick logic)
    (def now (time.now))
    (def delta (- now last-tick))
    (if (>= delta 40)
      (do (logic) (tick now logic))
      (tick last-tick logic)
    )
  )
  (tick (time.now) logic)
)
```
