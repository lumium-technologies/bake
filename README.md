# bake
a simple monorepo build system - as simple as make, as scalable as bazel

## how it works
the project is based on the concept of the `Cakefile` - a file not unlike the traditional `Makefile`, yet different in various ways. yes `Cakefile`, because the expression is "to bake a cake" - sorry, i'll show myself out.

syntax-wise the `Cakefile` feels pretty similar to the `Makefile`. it has the same syntax constructs to declare targets and define dependencies between targets. a target in a `Cakefile` is nothing but a name to a command/sequence of commands, like this:

```
build:
    cargo build --release
```

unlike with `Makefile`s, a target in a `Cakefile` does not represent a file on disk. that way, we can also remove the need for `.PHONY` targets in `Makefile`s. targets in `Cakefile`s really just represent a sequence of commands to run - that's it, that's the whole concept. by removing the notion of a target representing a file on disk, we can have our build system that is based on `make` still look like `make` and retain the intuition of `make`, but simplify the whole system dramatically.

of course, targets can have other targets as dependencies, and even have nested targets, individual files or even globs as dependencies and outputs. output files are specified after a minus after the dependency list:

```
build: module1::build module2::build "object-file1.o" "object-file2.o" - "object"
    # link objects produced by module1::build and module2::build
    # with "object-file1.o" and "object-file2.o" to create one single relocatable object file
```

where the project structure would look like this:

```
.
|-- Cakefile
|-- module1
|   `-- Cakefile # defines an own build target
|-- module2
|   `-- Cakefile # defines an own build target
|-- object-file1.o
`-- object-file2.o
```

to avoid rebuilding everything all the time, targets can specify output files. dependent targets then only run when a declared output file has changed, similar to how `make` does it - just more explicit. that way targets are self-contained, reusable units. please note that `Cakefile`s follow a top-down approach - that is, a `Cakefile` can only reference targets from other `Cakefile`s if those other `Cakefile`s are located in the directory tree under the current `Cakefile` - you cannot reference targets higher up in the directory tree. this serves to enforce encapsulation of individual monorepo components - note that it is an antipattern to conjure together some magic to nonetheless somehow execute a target higher up in the directory tree - it breaks the monorepo contract. it is also an antipattern to use the `bake` command in a `Cakefile`.

unlike with `Makefile`s, nesting of `Cakefile`s is not only allowed, but actively encouraged.
