import math
import numpy as np
from numba import njit, prange
import time
import random

# ============================================
#  DEPTH CHARGE CONFIG (use in-game values)
# ============================================
MAX_TILE = 3            
BETTAH_MULT = 2.7       
BOARD_SIZE = 16          
BOMBS = 4                
LIVES = 3                
OPPONENT_HP = 1000      
DAMAGE = 25
GOLDEN_TILES = 2
BOOM_BLOCKER = 2
BIG_HIT_COMBO = 0.10
FINAL_ROUND_FURY = 2           

_QTY1 = float(MAX_TILE - 1)
_QTY3 = float(round((BETTAH_MULT - 1.0) * 100.0))
TARGET = OPPONENT_HP // DAMAGE
SAFE_TILES = BOARD_SIZE - BOMBS

def upgrade_prob(qty3):
    return 0.14 + min(0.06, qty3 / 2000.0) + min(0.5, 0.5 * qty3 / (qty3 + 1500.0))

P_UP = float(upgrade_prob(_QTY3))
MAX_TIER = float(_QTY1 + 1.0)
BASE = float(min(12.0, _QTY3 / 150.0))

# ============================================
#  PURE MATH JIT-COMPILED SIMULATION
# ============================================

@njit(parallel=True)
def run_simulations(target, num_sims):
    hits = np.zeros(SAFE_TILES, dtype=np.int32)
    
    for i in prange(SAFE_TILES): 
        flip = i + 1
        successes = 0
        
        for _ in range(num_sims):
            lives = LIVES
            score = 0
            golden_tiles = GOLDEN_TILES
            boom_blockers = BOOM_BLOCKER
            
            while lives > 0:
                lived = True
                pool = BOARD_SIZE
                bombs = BOMBS
                for _ in range(flip):
                    if golden_tiles > 0:
                        golden_tiles -= 1
                        pool -= 1
                    else:
                        if random.random() < (bombs / pool):
                            if boom_blockers > 0:
                                boom_blockers -= 1
                                bombs -= 1
                            else:
                                lived = False
                                break
                        pool -= 1

                if lived:
                    round_score = 0
                    for _f in range(flip):
                        val = math.floor(1.0 + min(BASE + np.random.random(), _QTY1))
                        while random.random() <= P_UP:
                            val += 1.0
                            if val >= MAX_TIER:
                                break
                        round_score += int(val)

                    round_score = round_score * (1.0 + (BIG_HIT_COMBO * flip))
                    if lives == 1:
                        round_score *= FINAL_ROUND_FURY
                    score += round_score
                    if score >= target:
                        successes += 1
                        break
                else:
                    lives -= 1
                            
        hits[i] = successes
        
    return hits

# ============================================
#  STANDARD WRAPPER
# ============================================
def simulate_all(target, num_sims=10000000):
    # Tiny warmup run so Numba compiles C-code before the timer starts
    _ = run_simulations(target, 10)
    
    start_time = time.time()
    hits = run_simulations(target, num_sims)
    duration = time.time() - start_time
    
    print(f"Target: {target} (Simulated {num_sims:,} times per flip-count)")
    print(f"Time Taken: {duration:.3f} seconds\n")
    print("Flips\tWin Rate")
    
    best_idx = np.argmax(hits)
    for i in range(SAFE_TILES):
        suffix = "BEST" if i == best_idx else ""
        print(f"{i+1}{suffix}\t{hits[i]/num_sims*100:.2f}%")

if __name__ == "__main__":
    simulate_all(TARGET, 1000000)