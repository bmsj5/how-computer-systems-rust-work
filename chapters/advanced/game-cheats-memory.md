# Game Cheats: How They Work

## Question 3: How Do Game Cheats Work?

**Your question:** "But how about cheats for online games that somehow can allow you to see through walls (wallhack) or access to some internal information (map-hack) - are those being stolen from the RAM, or it's network-related or maybe neither in the specific way, just a little bit there, little bit here and just in general just some tweaks to catch bugs and being lucky to catch it and being unspotted?"

**Answer:** **Most game cheats read from the game process's RAM (same process, so allowed). Some use network manipulation. It's a combination of techniques.**

### How Game Cheats Actually Work

**Most game cheats read from the game's own memory (same process):**

**The game process:**
```
Game.exe (running):
  Memory contains:
    - Player positions (X, Y, Z coordinates)
    - Enemy positions
    - Map data
    - Health values
    - Weapon data
    - etc.
```

**The cheat program:**
```
Cheat.exe (separate process):
  Uses Windows API to:
    1. Attach to Game.exe process
    2. Read Game.exe's memory
    3. Extract player positions, enemy positions, etc.
    4. Display overlay (wallhack, ESP)
```

### Why This Works

**Windows allows reading another process's memory if you have permission:**

**Windows API:**
```c
// Attach to game process
HANDLE hProcess = OpenProcess(PROCESS_VM_READ, FALSE, game_pid);

// Read memory
ReadProcessMemory(hProcess, address, buffer, size, &bytes_read);

// Extract player position
struct Player {
    float x, y, z;  // Coordinates
    int health;
};

ReadProcessMemory(hProcess, player_address, &player, sizeof(Player), NULL);
```

**Why it's allowed:**
- Debugging tools need this (Visual Studio, WinDbg)
- Process monitoring tools need this
- Windows allows it for legitimate purposes
- Cheats abuse this capability

### Types of Game Cheats

**1. Memory Reading (Most Common):**
```
Cheat reads game's memory:
  - Player positions → Display on overlay (ESP)
  - Enemy positions → Show through walls (wallhack)
  - Health values → Show health bars
  - Weapon data → Show weapon info
```

**2. Memory Writing:**
```
Cheat modifies game's memory:
  - Health value → Set to 9999 (god mode)
  - Ammo count → Set to infinite
  - Speed value → Increase movement speed
```

**3. Network Manipulation:**
```
Cheat intercepts network packets:
  - Modify outgoing packets (fake position)
  - Read incoming packets (see enemy positions)
  - Delay packets (lag switch)
```

**4. DLL Injection:**
```
Cheat injects code into game process:
  - Hook game functions
  - Modify game behavior
  - Bypass anti-cheat checks
```

### How Wallhack Works

**Wallhack (see through walls):**

**Step 1: Read player positions**
```
Cheat reads from game memory:
  Player position: (100, 50, 200)
  Enemy position: (150, 60, 250)
```

**Step 2: Calculate visibility**
```
Cheat calculates:
  - Line of sight
  - Distance
  - Angle
```

**Step 3: Draw overlay**
```
Cheat draws on screen:
  - Box around enemy (even through walls)
  - Line to enemy
  - Distance indicator
```

**The game doesn't render enemies through walls, but the cheat does!**

### How Map-Hack Works

**Map-hack (see entire map):**

**Step 1: Read map data**
```
Cheat reads from game memory:
  - Map layout
  - Fog of war data
  - Hidden areas
```

**Step 2: Read player positions**
```
Cheat reads:
  - All player positions (even hidden)
  - Enemy positions
  - Resource locations
```

**Step 3: Display on overlay**
```
Cheat draws:
  - Full map (ignoring fog of war)
  - All player positions
  - Hidden information
```

### Why Anti-Cheat Struggles

**Challenges:**

1. **Legitimate tools:**
   - Debuggers need memory access
   - Can't block all memory access
   - Cheats use same APIs as legitimate tools

2. **Detection is hard:**
   - Cheats can hide (rootkit techniques)
   - Cheats can bypass detection
   - New cheats appear constantly

3. **Performance:**
   - Checking everything is slow
   - Balance between security and performance

### Modern Anti-Cheat

**Kernel-level anti-cheat (e.g., Vanguard, BattlEye):**
- Runs in kernel (ring 0)
- Can detect cheats more effectively
- More invasive (can scan entire system)

**Server-side validation:**
- Server validates player actions
- Detects impossible moves
- Bans cheaters

**Behavioral analysis:**
- Detect unusual patterns
- Machine learning
- Statistical analysis

### Summary

| Cheat Type | How It Works | Detection Difficulty |
|------------|--------------|---------------------|
| **Memory reading** | Read game's RAM (same process) | Medium |
| **Memory writing** | Modify game's RAM | Medium-Hard |
| **Network manipulation** | Intercept/modify packets | Hard |
| **DLL injection** | Inject code into game | Very Hard |

**Key insight:** Most game cheats read from the game's own memory (same process, so allowed by OS). They don't "steal" from other processes - they read from the game process itself. Anti-cheat tries to detect and prevent this, but it's an ongoing arms race.
