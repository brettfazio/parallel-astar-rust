# parallel-project-dump

The team's code has been modularized and lives in 1 cargo project to make testing and running the application for many different cases very simple.

## Running

Below is a template for how to run our project inside the "src" folder.  Anything in "<>" is used as a flag and is variable input that the user can decide.

cargo run -- --graph <input_file_name> --num_threads <#of_threads> --algo <algorithmn_tested> --heur <hueristic_tested>

## <input_file_name>

This flag decides which graph of file input you would like to test.  We have sample data in "data" folder that can be used.  You would just simply specify the name of the file in the "data" folder that you want to test on when running.

## <#of_threads>

This specifys how many threads you want to test on the project.  2,4,8,16 are some common options but any will suffice.

## <algorithmn_tested>

This is where the user can specify which algorithmn they want to test.  Our project has 3 main algorithmns that can be found by moving through the "src" folder then the "a_star" folder.  There you will find "kpbfs", "dpa, and "hda".  You can specify any one of these 3 algorithmns to run the program on.

## <heuristic_tested>

This is where the user can specify which hueristic they want our algorithmn to use.  A hueristic is a crucial part of the A*star algorithmn
and the type you use can impact results more than one would think.  The heuristics one can choose are "euclidian", "manhattan", "expensive", "nonadmissiable", 
and "expnon"

So for example let's say I wanted to run a medium level graph on 4 threads of the dpa algorithmn with the euclidean heurstic.  I would run this command below inside the "src" folder.

cargo run -- --graph medium1.in --num_threads 4 --algo dpa --heur euclidean
  
