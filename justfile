set shell := ["fish", "-c"]

problems := `find src/ -type f -name 'lc*.rs' | grep -Po '\d+' | sort | tr '\n' ' '`

# Choose a problem to test
@default:
    just test (gum choose {{ problems }})

# Test problem(s) without timing. Default: all
@test prob="all":
    just _test run {{ prob }}

# Test and time problem(s). Default: all
@time prob="all":
    just _test time {{ prob }}

@_test method prob:
    just _{{ method }}_test (if test (string lower {{ prob }}) = "all"; echo lc; else; echo {{ prob }}; end)

_run_test filter="lc":
    cargo test {{ filter }}

_time_test filter="lc":
    cargo +nightly test -r {{ filter }} -- -Z unstable-options --report-time

# Format Rust code and this justfile
fmt:
    cargo fmt
    just --unstable --fmt
