## (An opinionated) dev workflow

### via ["trunk-based development"][1]

✅ Pull from `main` to update your local machine

```zsh
# from within the dotsync directory
$ git checkout main
$ git pull --rebase
```

✅ Checkout a working branch

```zsh
# from within the dotsync directory
# pull to get the latest refs - all published branches
$ git pull --rebase

# once you have the latest, you can checkout the new branch
$ git checkout some_branch

# `-b` will create a new local branch and check it out automatically.
# when you push, you will publish the branch automatically.
$ git checkout -b some_branch
```

✅ Commit some changes

```zsh
# from within the dotsync directory
$ git commit -am "Here is a new thing"

$ git push

# alternatively, use the VSCode GUI
```

✅ Open a PR

✅ Merge the PR (and delete the working branch)

✅ Checkout and pull from `main`

```zsh
# from within the dotsync directory
$ git checkout main
$ git pull --rebase
```

## Documentation and where to find stuff

✅ [https://doc.rust-lang.org/std/][3]

✅ [https://docs.rs][2]

✅ Adding dependencies

```zsh
# from within the dotsync directory
$ cargo add some_thing
```

✅ Removing dependencies

```zsh
# from within the dotsync directory
$ cargo remove some_thing
```

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
