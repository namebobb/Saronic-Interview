# PDF command verification

**Result: 56/56 command responses passed; 0 failures.**

Actual executable stdin/stdout, compared as parsed JSON (object key order ignored).
Full commands recovered from embedded PDF text despite visual clipping. Omitted ticks are expanded.
Base example adapted only by adding dock_throughput=[2,4,1] and empty cargo fields required by Follow-on 1.

## 01-base-example

### 1. init: PASS

PDF example

Input:
```json
{"command":"init","parameters":{"travel_times":[[0,4,7],[4,0,3],[7,3,0]],"vessels":[{"id":"v1","location":0},{"id":"v2","location":2}],"dock_throughput":[2,4,1]}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":0,"vessels":{"v1":{"cargo":[],"location":0,"state":"idle"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":0,"vessels":{"v1":{"state":"idle","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 2. set_destination: PASS

PDF example

Input:
```json
{"command":"set_destination","parameters":{"vessel_id":"v1","destination":2}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":0,"vessels":{"v1":{"arrives_at_tick":7,"cargo":[],"from":0,"state":"transit","to":2},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":0,"vessels":{"v1":{"state":"transit","from":0,"to":2,"arrives_at_tick":7,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 3. set_destination: PASS

PDF example

Input:
```json
{"command":"set_destination","parameters":{"vessel_id":"v1","destination":1}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":0,"vessels":{"v1":{"arrives_at_tick":4,"cargo":[],"from":0,"state":"transit","to":1},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":0,"vessels":{"v1":{"state":"transit","from":0,"to":1,"arrives_at_tick":4,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 4. tick: PASS

PDF example

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":1,"vessels":{"v1":{"arrives_at_tick":4,"cargo":[],"from":0,"state":"transit","to":1},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":1,"vessels":{"v1":{"state":"transit","from":0,"to":1,"arrives_at_tick":4,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 5. set_destination: PASS

PDF example

Input:
```json
{"command":"set_destination","parameters":{"vessel_id":"v1","destination":2}}
```
Actual output:
```json
{"status":"error","payload":{"message":"vessel v1 is not idle"}}
```
Expected:
```json
{"status":"error","payload":{"message":"vessel v1 is not idle"}}
```

### 6. tick: PASS

Inserted omitted tick

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":2,"vessels":{"v1":{"arrives_at_tick":4,"cargo":[],"from":0,"state":"transit","to":1},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":2,"vessels":{"v1":{"state":"transit","from":0,"to":1,"arrives_at_tick":4,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 7. tick: PASS

Inserted omitted tick

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":3,"vessels":{"v1":{"arrives_at_tick":4,"cargo":[],"from":0,"state":"transit","to":1},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":3,"vessels":{"v1":{"state":"transit","from":0,"to":1,"arrives_at_tick":4,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 8. tick: PASS

PDF example

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":4,"vessels":{"v1":{"cargo":[],"location":1,"state":"idle"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":4,"vessels":{"v1":{"state":"idle","location":1,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 9. set_destination: PASS

PDF example

Input:
```json
{"command":"set_destination","parameters":{"vessel_id":"v1","destination":2}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":4,"vessels":{"v1":{"arrives_at_tick":7,"cargo":[],"from":1,"state":"transit","to":2},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":4,"vessels":{"v1":{"state":"transit","from":1,"to":2,"arrives_at_tick":7,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

## 02-cargo-and-history

### 1. init: PASS

PDF example

Input:
```json
{"command":"init","parameters":{"travel_times":[[0,4,7],[4,0,3],[7,3,0]],"dock_throughput":[2,4,1],"vessels":[{"id":"v1","location":0},{"id":"v2","location":2}]}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":0,"vessels":{"v1":{"cargo":[],"location":0,"state":"idle"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":0,"vessels":{"v1":{"state":"idle","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 2. tick: PASS

PDF example

Input:
```json
{"command":"tick","parameters":{"cargo_manifest":[{"location":0,"destination":2,"quantity":1},{"location":0,"destination":1,"quantity":1}]}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[{"destination":2,"location":0,"quantity":1},{"destination":1,"location":0,"quantity":1}],"tick":1,"vessels":{"v1":{"cargo":[],"location":0,"state":"idle"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":1,"vessels":{"v1":{"state":"idle","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[{"location":0,"destination":2,"quantity":1},{"location":0,"destination":1,"quantity":1}]}}
```

### 3. dock: PASS

PDF example

Input:
```json
{"command":"dock","parameters":{"vessel_id":"v1"}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[{"destination":2,"location":0,"quantity":1},{"destination":1,"location":0,"quantity":1}],"tick":1,"vessels":{"v1":{"cargo":[],"location":0,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":1,"vessels":{"v1":{"state":"docked","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[{"location":0,"destination":2,"quantity":1},{"location":0,"destination":1,"quantity":1}]}}
```

### 4. tick: PASS

PDF example

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[{"destination":2,"location":0,"quantity":1},{"destination":1,"location":0,"quantity":1}],"tick":2,"vessels":{"v1":{"cargo":[],"location":0,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":2,"vessels":{"v1":{"state":"docked","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[{"location":0,"destination":2,"quantity":1},{"location":0,"destination":1,"quantity":1}]}}
```

### 5. load: PASS

PDF example

Input:
```json
{"command":"load","parameters":{"vessel_id":"v1","destination":2}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[{"destination":1,"location":0,"quantity":1}],"tick":2,"vessels":{"v1":{"cargo":[{"destination":2,"quantity":1}],"location":0,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":2,"vessels":{"v1":{"state":"docked","location":0,"cargo":[{"destination":2,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[{"location":0,"destination":1,"quantity":1}]}}
```

### 6. load: PASS

PDF example

Input:
```json
{"command":"load","parameters":{"vessel_id":"v1","destination":1}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":2,"vessels":{"v1":{"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}],"location":0,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":2,"vessels":{"v1":{"state":"docked","location":0,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 7. tick: PASS

PDF example

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":3,"vessels":{"v1":{"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}],"location":0,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":3,"vessels":{"v1":{"state":"docked","location":0,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 8. undock: PASS

PDF example

Input:
```json
{"command":"undock","parameters":{"vessel_id":"v1"}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":3,"vessels":{"v1":{"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}],"location":0,"state":"idle"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":3,"vessels":{"v1":{"state":"idle","location":0,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 9. tick: PASS

PDF example

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":4,"vessels":{"v1":{"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}],"location":0,"state":"idle"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":4,"vessels":{"v1":{"state":"idle","location":0,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 10. set_destination: PASS

PDF example

Input:
```json
{"command":"set_destination","parameters":{"vessel_id":"v1","destination":2}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":4,"vessels":{"v1":{"arrives_at_tick":11,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}],"from":0,"state":"transit","to":2},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":4,"vessels":{"v1":{"state":"transit","from":0,"to":2,"arrives_at_tick":11,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 11. tick: PASS

Inserted omitted tick

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":5,"vessels":{"v1":{"arrives_at_tick":11,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}],"from":0,"state":"transit","to":2},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":5,"vessels":{"v1":{"state":"transit","from":0,"to":2,"arrives_at_tick":11,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 12. tick: PASS

Inserted omitted tick

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":6,"vessels":{"v1":{"arrives_at_tick":11,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}],"from":0,"state":"transit","to":2},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":6,"vessels":{"v1":{"state":"transit","from":0,"to":2,"arrives_at_tick":11,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 13. tick: PASS

Inserted omitted tick

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":7,"vessels":{"v1":{"arrives_at_tick":11,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}],"from":0,"state":"transit","to":2},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":7,"vessels":{"v1":{"state":"transit","from":0,"to":2,"arrives_at_tick":11,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 14. tick: PASS

Inserted omitted tick

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":8,"vessels":{"v1":{"arrives_at_tick":11,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}],"from":0,"state":"transit","to":2},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":8,"vessels":{"v1":{"state":"transit","from":0,"to":2,"arrives_at_tick":11,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 15. tick: PASS

Inserted omitted tick

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":9,"vessels":{"v1":{"arrives_at_tick":11,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}],"from":0,"state":"transit","to":2},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":9,"vessels":{"v1":{"state":"transit","from":0,"to":2,"arrives_at_tick":11,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 16. tick: PASS

Inserted omitted tick

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":10,"vessels":{"v1":{"arrives_at_tick":11,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}],"from":0,"state":"transit","to":2},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":10,"vessels":{"v1":{"state":"transit","from":0,"to":2,"arrives_at_tick":11,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 17. tick: PASS

PDF example

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":11,"vessels":{"v1":{"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}],"location":2,"state":"idle"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":11,"vessels":{"v1":{"state":"idle","location":2,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 18. dock: PASS

PDF example

Input:
```json
{"command":"dock","parameters":{"vessel_id":"v1"}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":11,"vessels":{"v1":{"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}],"location":2,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":11,"vessels":{"v1":{"state":"docked","location":2,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 19. tick: PASS

PDF example

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":12,"vessels":{"v1":{"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}],"location":2,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":12,"vessels":{"v1":{"state":"docked","location":2,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 20. unload: PASS

PDF example

Input:
```json
{"command":"unload","parameters":{"vessel_id":"v1","destination":2}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":12,"vessels":{"v1":{"cargo":[{"destination":1,"quantity":1}],"location":2,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":12,"vessels":{"v1":{"state":"docked","location":2,"cargo":[{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 21. tick: PASS

PDF example

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":13,"vessels":{"v1":{"cargo":[{"destination":1,"quantity":1}],"location":2,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":13,"vessels":{"v1":{"state":"docked","location":2,"cargo":[{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 22. get_state_at: PASS

PDF example

Input:
```json
{"command":"get_state_at","parameters":{"tick":1}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[{"destination":2,"location":0,"quantity":1},{"destination":1,"location":0,"quantity":1}],"tick":1,"vessels":{"v1":{"cargo":[],"location":0,"state":"idle"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":1,"vessels":{"v1":{"state":"idle","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[{"location":0,"destination":2,"quantity":1},{"location":0,"destination":1,"quantity":1}]}}
```

### 23. get_state_at: PASS

PDF example

Input:
```json
{"command":"get_state_at","parameters":{"tick":13}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":13,"vessels":{"v1":{"cargo":[{"destination":1,"quantity":1}],"location":2,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":13,"vessels":{"v1":{"state":"docked","location":2,"cargo":[{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 24. get_state_at: PASS

PDF example

Input:
```json
{"command":"get_state_at","parameters":{"tick":99}}
```
Actual output:
```json
{"status":"error","payload":{"message":"tick 99 is in the future (current tick: 13)"}}
```
Expected:
```json
{"status":"error","payload":{"message":"tick 99 is in the future (current tick: 13)"}}
```

### 25. get_state: PASS

PDF example

Input:
```json
{"command":"get_state","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":13,"vessels":{"v1":{"cargo":[{"destination":1,"quantity":1}],"location":2,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":13,"vessels":{"v1":{"state":"docked","location":2,"cargo":[{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

## 03-swap-example

### 1. init: PASS

PDF example

Input:
```json
{"command":"init","parameters":{"travel_times":[[0,4,7],[4,0,3],[7,3,0]],"dock_throughput":[2,4,1],"vessels":[{"id":"v1","location":0},{"id":"v2","location":2}]}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":0,"vessels":{"v1":{"cargo":[],"location":0,"state":"idle"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":0,"vessels":{"v1":{"state":"idle","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 2. tick: PASS

PDF example

Input:
```json
{"command":"tick","parameters":{"cargo_manifest":[{"location":0,"destination":2,"quantity":1},{"location":0,"destination":1,"quantity":1}]}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[{"destination":2,"location":0,"quantity":1},{"destination":1,"location":0,"quantity":1}],"tick":1,"vessels":{"v1":{"cargo":[],"location":0,"state":"idle"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":1,"vessels":{"v1":{"state":"idle","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[{"location":0,"destination":2,"quantity":1},{"location":0,"destination":1,"quantity":1}]}}
```

### 3. dock: PASS

PDF example

Input:
```json
{"command":"dock","parameters":{"vessel_id":"v1"}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[{"destination":2,"location":0,"quantity":1},{"destination":1,"location":0,"quantity":1}],"tick":1,"vessels":{"v1":{"cargo":[],"location":0,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":1,"vessels":{"v1":{"state":"docked","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[{"location":0,"destination":2,"quantity":1},{"location":0,"destination":1,"quantity":1}]}}
```

### 4. tick: PASS

PDF example

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[{"destination":2,"location":0,"quantity":1},{"destination":1,"location":0,"quantity":1}],"tick":2,"vessels":{"v1":{"cargo":[],"location":0,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":2,"vessels":{"v1":{"state":"docked","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[{"location":0,"destination":2,"quantity":1},{"location":0,"destination":1,"quantity":1}]}}
```

### 5. load: PASS

PDF example

Input:
```json
{"command":"load","parameters":{"vessel_id":"v1","destination":2}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[{"destination":1,"location":0,"quantity":1}],"tick":2,"vessels":{"v1":{"cargo":[{"destination":2,"quantity":1}],"location":0,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":2,"vessels":{"v1":{"state":"docked","location":0,"cargo":[{"destination":2,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[{"location":0,"destination":1,"quantity":1}]}}
```

### 6. swap: PASS

Standalone swap example, PDF physical page 9; setup loads one unit at tick 2.

Input:
```json
{"command":"swap","parameters":{"vessel_id":"v1","load_destination":1,"unload_destination":2}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[{"destination":2,"location":0,"quantity":1}],"tick":2,"vessels":{"v1":{"cargo":[{"destination":1,"quantity":1}],"location":0,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":2,"vessels":{"v1":{"state":"docked","location":0,"cargo":[{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[{"location":0,"destination":2,"quantity":1}]}}
```

## 04-manifest-init-and-addition

### 1. init: PASS

Standalone init, PDF physical page 8.

Input:
```json
{"command":"init","parameters":{"travel_times":[[0,4,7],[4,0,3],[7,3,0]],"dock_throughput":[2,4,1],"vessels":[{"id":"v1","location":0},{"id":"v2","location":2}],"cargo_manifest":[{"location":0,"destination":2,"quantity":1},{"location":0,"destination":1,"quantity":1},{"location":2,"destination":0,"quantity":1}]}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[{"destination":2,"location":0,"quantity":1},{"destination":1,"location":0,"quantity":1},{"destination":0,"location":2,"quantity":1}],"tick":0,"vessels":{"v1":{"cargo":[],"location":0,"state":"idle"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":0,"vessels":{"v1":{"state":"idle","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[{"location":0,"destination":2,"quantity":1},{"location":0,"destination":1,"quantity":1},{"location":2,"destination":0,"quantity":1}]}}
```

### 2. tick: PASS

Applying the standalone tick after this init must ADD cargo: quantity 2 each. The printed tick response shows quantity 1 and is not a sequential continuation of this init.

Input:
```json
{"command":"tick","parameters":{"cargo_manifest":[{"location":0,"destination":2,"quantity":1},{"location":0,"destination":1,"quantity":1},{"location":2,"destination":0,"quantity":1}]}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[{"destination":2,"location":0,"quantity":2},{"destination":1,"location":0,"quantity":2},{"destination":0,"location":2,"quantity":2}],"tick":1,"vessels":{"v1":{"cargo":[],"location":0,"state":"idle"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":1,"vessels":{"v1":{"state":"idle","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[{"location":0,"destination":2,"quantity":2},{"location":0,"destination":1,"quantity":2},{"location":2,"destination":0,"quantity":2}]}}
```

## 05-rejection-cases

### 1. get_state: PASS

Before init

Input:
```json
{"command":"get_state","parameters":{}}
```
Actual output:
```json
{"status":"error","payload":{"message":"Not initialized"}}
```
Expected:
```json
{"status":"error","payload":{"message":"Not initialized"}}
```

### 2. init: PASS

PDF example

Input:
```json
{"command":"init","parameters":{"travel_times":[[0,4,7],[4,0,3],[7,3,0]],"dock_throughput":[2,4,1],"vessels":[{"id":"v1","location":0},{"id":"v2","location":2}]}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":0,"vessels":{"v1":{"cargo":[],"location":0,"state":"idle"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":0,"vessels":{"v1":{"state":"idle","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 3. init: PASS

Second init

Input:
```json
{"command":"init","parameters":{"travel_times":[[0,4,7],[4,0,3],[7,3,0]],"dock_throughput":[2,4,1],"vessels":[{"id":"v1","location":0},{"id":"v2","location":2}]}}
```
Actual output:
```json
{"status":"error","payload":{"message":"Already initialized"}}
```
Expected:
```json
{"status":"error","payload":{"message":"Already initialized"}}
```

### 4. get_state_at: PASS

Negative historical tick

Input:
```json
{"command":"get_state_at","parameters":{"tick":-1}}
```
Actual output:
```json
{"status":"error","payload":{"message":"invalid value: integer `-1`, expected u64"}}
```
Expected:
```json
Any structured error response
```

### 5. set_destination: PASS

Unknown vessel

Input:
```json
{"command":"set_destination","parameters":{"vessel_id":"missing","destination":1}}
```
Actual output:
```json
{"status":"error","payload":{"message":"Cannot find vessel missing"}}
```
Expected:
```json
{"status":"error","payload":{"message":"Cannot find vessel missing"}}
```

### 6. set_destination: PASS

Out-of-range destination

Input:
```json
{"command":"set_destination","parameters":{"vessel_id":"v1","destination":3}}
```
Actual output:
```json
{"status":"error","payload":{"message":"invalid destination"}}
```
Expected:
```json
{"status":"error","payload":{"message":"invalid destination"}}
```

## 06-throughput-limit

### 1. init: PASS

PDF example

Input:
```json
{"command":"init","parameters":{"travel_times":[[0,4,7],[4,0,3],[7,3,0]],"dock_throughput":[2,4,1],"vessels":[{"id":"v1","location":0},{"id":"v2","location":2}]}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":0,"vessels":{"v1":{"cargo":[],"location":0,"state":"idle"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":0,"vessels":{"v1":{"state":"idle","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 2. tick: PASS

PDF example

Input:
```json
{"command":"tick","parameters":{"cargo_manifest":[{"location":0,"destination":2,"quantity":1},{"location":0,"destination":1,"quantity":1}]}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[{"destination":2,"location":0,"quantity":1},{"destination":1,"location":0,"quantity":1}],"tick":1,"vessels":{"v1":{"cargo":[],"location":0,"state":"idle"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":1,"vessels":{"v1":{"state":"idle","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[{"location":0,"destination":2,"quantity":1},{"location":0,"destination":1,"quantity":1}]}}
```

### 3. dock: PASS

PDF example

Input:
```json
{"command":"dock","parameters":{"vessel_id":"v1"}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[{"destination":2,"location":0,"quantity":1},{"destination":1,"location":0,"quantity":1}],"tick":1,"vessels":{"v1":{"cargo":[],"location":0,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":1,"vessels":{"v1":{"state":"docked","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[{"location":0,"destination":2,"quantity":1},{"location":0,"destination":1,"quantity":1}]}}
```

### 4. tick: PASS

PDF example

Input:
```json
{"command":"tick","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[{"destination":2,"location":0,"quantity":1},{"destination":1,"location":0,"quantity":1}],"tick":2,"vessels":{"v1":{"cargo":[],"location":0,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":2,"vessels":{"v1":{"state":"docked","location":0,"cargo":[]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[{"location":0,"destination":2,"quantity":1},{"location":0,"destination":1,"quantity":1}]}}
```

### 5. load: PASS

PDF example

Input:
```json
{"command":"load","parameters":{"vessel_id":"v1","destination":2}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[{"destination":1,"location":0,"quantity":1}],"tick":2,"vessels":{"v1":{"cargo":[{"destination":2,"quantity":1}],"location":0,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":2,"vessels":{"v1":{"state":"docked","location":0,"cargo":[{"destination":2,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[{"location":0,"destination":1,"quantity":1}]}}
```

### 6. load: PASS

PDF example

Input:
```json
{"command":"load","parameters":{"vessel_id":"v1","destination":1}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":2,"vessels":{"v1":{"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}],"location":0,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":2,"vessels":{"v1":{"state":"docked","location":0,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

### 7. unload: PASS

Third cargo operation exceeds throughput; PDF physical page 9.

Input:
```json
{"command":"unload","parameters":{"vessel_id":"v1","destination":2}}
```
Actual output:
```json
{"status":"error","payload":{"message":"transfer of 3 operations exceeds dock throughput of 2 at location 0"}}
```
Expected:
```json
{"status":"error","payload":{"message":"transfer of 3 operations exceeds dock throughput of 2 at location 0"}}
```

### 8. get_state: PASS

Rejected operation leaves planned state unchanged.

Input:
```json
{"command":"get_state","parameters":{}}
```
Actual output:
```json
{"status":"ok","payload":{"cargo":[],"tick":2,"vessels":{"v1":{"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}],"location":0,"state":"docked"},"v2":{"cargo":[],"location":2,"state":"idle"}}}}
```
Expected:
```json
{"status":"ok","payload":{"tick":2,"vessels":{"v1":{"state":"docked","location":0,"cargo":[{"destination":2,"quantity":1},{"destination":1,"quantity":1}]},"v2":{"state":"idle","location":2,"cargo":[]}},"cargo":[]}}
```

## Existing test suite

cargo test --locked --target-dir .\verification\target

Result: 14 protocol tests and 2 CLI tests passed; 0 failures.

## Reproduce

From src, build with: cargo build --locked --target-dir .\verification\target
Run verify_pdf.py with Python and pypdf installed. Each scenario also has a complete .input.jsonl and captured .output.jsonl beside this report. Pipe an input file into verification\target\debug\Saronic-Interview.exe to replay it.

