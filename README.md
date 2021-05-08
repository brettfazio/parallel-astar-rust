# Parallel A* Rust

The team's code has been modularized and lives in 1 cargo project to make testing and running the application for many different cases very simple.

## Code

The bulk of our interesting code is in the src/a_star folder where you can view files such as `hda.rs` `kpbfs.rs` and `dpa.rs`.

## Running

Below is a template for how to run our project inside the "src" folder.  Anything in "<>" is used as a flag and is variable input that the user can decide.

cargo run -- --graph <input_file_name> --num_threads <#of_threads> --algo <algorithmn_tested> --heur <hueristic_tested>

### <input_file_name>

This flag decides which graph of file input you would like to test.  We have sample data in "data" folder that can be used.  You would just simply specify the name of the file in the "data" folder that you want to test on when running.

### <#of_threads>

This specifys how many threads you want to test on the project.  2,4,8,16 are some common options but any will suffice.

### <algorithmn_tested>

This is where the user can specify which algorithmn they want to test.  Our project has 3 main algorithmns that can be found by moving through the "src" folder then the "a_star" folder.  There you will find "kpbfs", "dpa, and "hda".  You can specify any one of these 3 algorithmns to run the program on.

### <heuristic_tested>

This is where the user can specify which hueristic they want our algorithmn to use.  A hueristic is a crucial part of the A*star algorithmn
and the type you use can impact results more than one would think.  The heuristics one can choose are "euclidian", "manhattan", "expensive", "nonadmissiable", 
and "expnon"

### Sample Run

So for example let's say I wanted to run a medium level graph on 4 threads of the dpa algorithmn with the euclidean heurstic.  I would run this command below inside the "src" folder.

`cargo run -- --graph medium1.in --num_threads 4 --algo dpa --heur euclidean`
  
## Running via Bench

An alternative way to run is via the `cargo bench` command.

Below is the high level way of running.

```
cargo bench -- <algo>_<number>t_<heur>
```

So an example run would be `cargo bench -- dpa_2t_manhattan`

<algo> can be any of `kpbfs` `dpa` or `hda`
<number> can be 1, 2, 4, 8, or 16
<heur> can be `expensive` (warning this is probably too slow to run unless you change the graph size from medium1.in to small1.in in benchmark.rs so probably just avoid), `euclidean`, or `manhattan`.
