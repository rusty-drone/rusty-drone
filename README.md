# rusty-drone

Fully custom designed and programmed drone, that uses Rust and the Rasberry Pi 4.

Rusty_Drone Architecture Plan

```
|--- controllers
    |--- mod.rs
|--- events
    |--- continuous_event
    |--- impulse_event
    |--- event_handler
|--- streams
    |--- constant_stream
    |--- map_stream
    |--- zip_stream
    |--- custom_stream
|--- tasks
    |--- finite_task
    |--- infinite_task
    |--- parallel_task
    |--- sequential_task
|--- time
    |--- mod.rs
```

All _subsystems_ are `Components` which contain `Controllers` to reach desired states. The `Event Handler` schedules and handles `Events`. `Events` are triggered and contain a `Task` which they execute when they are fired. When fired, they manage the task by initializing, executing, and ending it. Essentially, an event is triggered which instantiates a task. The task then uses controllers to achieve a desired state for a subsystem.
