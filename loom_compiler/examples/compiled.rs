use core::mem;
use loom_compiler::jit;

fn main() -> Result<(), String> {
    // Create the JIT instance, which manages all generated functions and data.
    let mut jit = jit::JIT::default();
    println!("the answer is: {}", run_foo(&mut jit)?);
    println!(
        "recursive_fib(10) = {}",
        run_recursive_fib_code(&mut jit, 10)?
    );
    println!(
        "iterative_fib(10) = {}",
        run_iterative_fib_code(&mut jit, 10)?
    );
    run_hello(&mut jit)?;
    println!(
        "array test = {}",
        run_array(&mut jit)?
    );
    /*println!(
        "print int test = {}",
        run_print_int(&mut jit)?
    );*/
    Ok(())
}

fn run_foo(jit: &mut jit::JIT) -> Result<isize, String> {
    unsafe { run_code(jit, FOO_CODE, (42, 63)) }
}

fn run_recursive_fib_code(jit: &mut jit::JIT, input: isize) -> Result<isize, String> {
    unsafe { run_code(jit, RECURSIVE_FIB_CODE, input) }
}

fn run_iterative_fib_code(jit: &mut jit::JIT, input: isize) -> Result<isize, String> {
    unsafe { run_code(jit, ITERATIVE_FIB_CODE, input) }
}

fn run_hello(jit: &mut jit::JIT) -> Result<isize, String> {
    jit.create_data("hello_string", "hello world!\0".as_bytes().to_vec())?;
    unsafe { run_code(jit, HELLO_CODE, ()) }
}

fn run_array(jit: &mut jit::JIT) -> Result<isize, String> {
    unsafe { run_code(jit, ARRAY_CODE, ()) }
}

fn run_print_int(jit: &mut jit::JIT) -> Result<isize, String> {
    unsafe { run_code(jit, PRINT_INT_CODE, ()) }
}

/// Executes the given code using the cranelift JIT compiler.
///
/// Feeds the given input into the JIT compiled function and returns the resulting output.
///
/// # Safety
///
/// This function is unsafe since it relies on the caller to provide it with the correct
/// input and output types. Using incorrect types at this point may corrupt the program's state.
unsafe fn run_code<I, O>(jit: &mut jit::JIT, code: &str, input: I) -> Result<O, String> {
    // Pass the string to the JIT, and it returns a raw pointer to machine code.
    let code_ptr = jit.compile(code)?;
    // Cast the raw pointer to a typed function pointer. This is unsafe, because
    // this is the critical point where you have to trust that the generated code
    // is safe to be called.
    let code_fn = mem::transmute::<_, fn(I) -> O>(code_ptr);
    // And now we can call it!
    Ok(code_fn(input))
}

// A small test function.
//
// The `(c)` declares a return variable; the function returns whatever value
// it was assigned when the function exits. Note that there are multiple
// assignments, so the input is not in SSA form, but that's ok because
// Cranelift handles all the details of translating into SSA form itself.
const FOO_CODE: &str = r#"
    (fn foo [a b] []
        (set sum (+ a b))
        sum
    )
"#;

/// Another example: Recursive fibonacci.
const RECURSIVE_FIB_CODE: &str = r#"
    (fn recursive_fib [n] []
        (if (= n 0)
            0
            (if (= n 1)
                1
                (+
                    (recursive_fib (- n 1))
                    (recursive_fib (- n 2))
                )
            )
        )
    )
"#;

/// Another example: Iterative fibonacci.
const ITERATIVE_FIB_CODE: &str = r#"
    (fn iterative_fib [n] []
        (if (= n 0)
            0
            (do
                (set n (- n 1))
                (set a 0)
                (set result 1)
                (while (!= n 0)
                    (set t result)
                    (set result (+ result a))
                    (set a t)
                    (set n (- n 1))
                )
                result
            )
        )
    )
"#;

const ARRAY_CODE: &str = r#"
    (fn array_test [] []
        (set a (array 8))
        (array_set a 7 420)
        (array_set a 3 69)
        (+
            (array_get a 7)
            (array_get a 3)
        )
    )
"#;

const PRINT_INT_CODE: &str = r#"
    (fn print_int [] []
        (set n 9997)

        ; Find out how many digits n is
        (set log 0)
        (while (>= n 10)
            (set n (/ n 10))
            (set log (+ log 1))
        )

        ; Printing integers
        ; These need to be put into an array and printed in reverse
        (set string (array 4))
        (set i log)
        (while (> n 0)
            (array_set string i (+ 48 (% n 10)))
            (set n (/ n 10))
            (set i (- i 1))
        )

        (puts string)
    )
"#;

/// Let's say hello, by calling into libc. The puts function is resolved by
/// dlsym to the libc function, and the string &hello_string is defined below.
const HELLO_CODE: &str = r#"
    (fn hello [] []
        (puts &hello_string)
    )
"#;
