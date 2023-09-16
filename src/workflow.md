---
marp: true
class:
    - invert
---

# (An opinionated) dev workflow

## via ["trunk-based development"][1]

---

✅ Pull from `main`

```zsh
# from within the dotsync directory
$ git checkout main
$ git pull
```

---

✅ Checkout working branch

```zsh
# from within the dotsync directory
$ git checkout some_branch

# `-b` will create a new local branch (to be published)
$ git checkout -b some_branch
```

---

✅ Commit some changes

```zsh
# from within the dotsync directory
$ git commit -am "Here is a new thing"

# alternatively, use the VSCode GUI
```

---

✅ Open a PR

✅ Merge the PR (and delete the working branch)

✅ Checkout and pull from `main`

```zsh
# from within the dotsync directory
$ git checkout main
$ git pull
```

---

## Documentation and where to find stuff

✅ [https://doc.rust-lang.org/std/][3]

✅ [https://docs.rs][2]

---

✅ Adding dependencies

```zsh
# from within the dotsync directory
$ cargo add some_thing
```

---

✅ Removing dependencies

```zsh
# from within the dotsync directory
$ cargo remove some_thing
```

---

## Rust APIs / modules, and documentation

✅ Split functionality into crates (libraries and programs)

✅ Documenting code

```zsh
# from within the dotsync directory
$ cargo doc --no-deps --open
```

[1]: https://cloud.google.com/architecture/devops/devops-tech-trunk-based-development
[2]: https://docs.rs
[3]: https://doc.rust-lang.org/std/