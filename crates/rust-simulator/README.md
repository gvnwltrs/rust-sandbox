# Elevator Simulator

A real-time elevator physics simulation with terminal visualization.

## Running the Simulation

### Method 1: Using cargo (Recommended)
```bash
cargo run
```

### Method 2: Using the shell script
```bash
bash test_sim.sh
```

### Method 3: With a custom input file
```bash
cargo run path/to/input.txt
```

## Controls

- **Ctrl+C**: Exit the simulation early and display summary statistics
- The simulation will automatically exit when all floor requests are completed

## Input File Format

The `test1.txt` file (or your custom input file) should have:
1. Line 1: Number of floors (integer)
2. Line 2: Height per floor in meters (float)
3. Lines 3+: Floor request numbers (one per line)

Example (`test1.txt`):
```
5
3.0
2
5
1
```

## Troubleshooting

### Issue: Screen scrolls instead of updating in-place

**Cause**: The terminal isn't properly in raw mode.

**Solutions**:
1. Make sure you're running directly in a terminal (not through an IDE's output panel)
2. Try running with: `cargo run` (not `cargo run --lib`)
3. Ensure your terminal emulator supports ANSI escape codes

### Issue: Ctrl+C doesn't exit

**Cause**: The signal handler may not be working in your environment.

**Solutions**:
1. Try pressing Ctrl+C multiple times
2. Use Ctrl+\ (SIGQUIT) as an alternative
3. Check that the `ctrlc` crate is properly installed: `cargo build`

### Issue: "Not running in a terminal" warning

**Cause**: stdout is not connected to a TTY.

**Solution**: Run the program directly in a terminal, not through a pipe or file redirection.

## Technical Details

- Uses `termion` for raw terminal mode and ANSI escape sequences
- Uses `ctrlc` crate for signal handling
- Physics simulation runs at ~100Hz (10ms sleep per iteration)
- Updates terminal display in-place using `clear::All` and `cursor::Goto(1,1)`