# Welcome to TypeBuddy!
![TypeBuddy](./assets/typeBuddy.png)

This CLI will help you improve your typing accuracy and speed! 
Improve your personal bests and look back on your previous records in a graph. 
All in the convenience of your own terminal!

## Quickstart
1. Run the following command to install with cargo:
- `cargo install type_buddy`

2. Open your terminal and run:
- `type_buddy start`

3. Have fun improving your typing skills :)

## Download Binary
if you do not want to compile the binary with cargo you can also download the binary itself for
Intel chip MacOS and x86_64 Linux systems

Head over to the latest GitHub release and download the artifact.


## Additional settings
If you want to start saving your stats and being able to plot them in a graph, your will need to
set up an ENV variable in your machine: 

```
TB_STATS_DIR=<PATH TO A DIRECTORY>
```

This directory will be used to save a JSON file called `type_buddy_stats.json`. 
TypeBuddy will use this to read from and write to in order to save your stats.

## Commands
- `type_buddy start`
- `type_buddy start --max-minutes <minutes>` typing with a deadline :)
- `type_buddy plot --wpm` to plot your WPM
- `type_buddy plot --accuracy` to plot your accuracy

# Demo's
### Training
![Plotter](./assets/example.gif)
### Plotter
![Plotter](./assets/plotter.png)



