# 🚀 BLAZINGLY FAST LINE COUNTER 🏎️💨

Welcome to the **BLAZINGLY FAST LINE COUNTER**! This is no ordinary line counter—this bad boy is *supercharged* with Rust power and will count your lines faster than a caffeinated squirrel on roller skates! 🐿️🛼

## Features 🎉

- **Recursive Directory Safari**: 🏞️ Explore every nook and cranny of your specified directory and its subdirectories like an adventurous explorer on a quest for… lines!
- **Parallel Processing Power-Up**: 💪 With the **Rayon** library, we harness the power of parallel processing, making sure this line counter zooms through your files faster than a racecar on turbo boost.
- **Fancy-Pants Formatted Output**: 💅 Get your results served on a silver platter—neatly formatted in a table so stylish, it could walk the runway!

## Requirements 🛠️

- **Rust** (because we like things fast)
- **Cargo** (to build this speed demon)
- **A Terminal That Gets Our ANSI Jokes** (we're serious about those escape codes!)

## Installation 🔧

1. **Clone the Repo and Enter the Speed Zone**:

    ```bash
    git clone https://github.com/mvstermind/li-nes.git
    cd li-nes
    ```

2. **Build It Like You Mean It**:

    ```bash
    cargo build --release
    ```

    Now you've got yourself a supercharged line counter ready to roll! 🏁

## Usage 🕹️

Ready to race? Just pass the directory path as an argument and watch this tool blaze through your files:

```bash
./target/release/li-nes <directory_path>
```

### Example - Because We Love Showing Off 😏

```bash
./target/release/li-nes /path/to/your/directory
```

Sit back, relax, and let the speedster do its thing. 🏎️💨

## Output 🎯

When it's done, **BLAZINGLY FAST LINE COUNTER** will proudly display your line counts in a sleek, stylish table, right in the middle of your terminal (assuming your terminal can handle the swag).

### Sample Output 📊

```
                       BLAZINGLY FAST LINE COUNTER
                       ╔═══════════════════════════╗
                       ║ rs            ║      1200 ║
                       ║ txt           ║       800 ║
                       ║ md            ║       450 ║
                       ║ TOTAL         ║      2450 ║
                       ╚═══════════════════════════╝
This took: 0.0023s
```

**Bam!** There you have it—line counts faster than a lightning bolt! ⚡

## License 📜

This project is licensed under the **MIT License**, so you can use it freely while still feeling like the speedster you are. 🏁

## Contributing - Join the Speed Team! 🏎️

Got an idea to make this line counter even faster? Or maybe you found a bug that needs squashing? We’d love to have you on board! Fork the repo, put on your racing helmet, and submit a pull request. Together, we’ll make the world a faster place! 🏆
