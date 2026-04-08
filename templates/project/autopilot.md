# Autopilot Configuration

## Metric
- **name**: val_loss
- **direction**: lower_is_better
- **threshold**: 0.01
- **grep_pattern**: ^val_loss:

## Run
- **command**: python train.py > run.log 2>&1
- **timeout**: 600
- **log_file**: run.log

## Loop
- **max_rounds**: 100
- **max_consecutive_failures**: 5

## Git
- **experiment_repo**: .
- **branch_prefix**: autopilot
