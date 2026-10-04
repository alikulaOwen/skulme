-- =============================================================================
-- ✈️ REAL-WORLD SQL: HAMILTONIAN CYCLE (LOGISTICS CARGO FLIGHT TOUR)
-- CATEGORY: Graph Backtracking in SQL (Recursive Graph Traversals)
-- =============================================================================
--
-- -----------------------------------------------------------------------------
-- 1. REAL-WORLD SCENARIO & BUSINESS USE CASE
-- -----------------------------------------------------------------------------
-- An air-cargo carrier (like FedEx or DHL) operates flights between cargo hubs:
-- - The distribution network is modeled in a `flight_network` table.
-- - A cargo plane starts at primary maintenance base Hub 0.
-- - The logistics director needs to plan a closed round-trip tour:
--   "Visit EVERY cargo hub in the regional cluster exactly once, and
--    return to Base Hub 0 at the end of the night."
-- - In Graph Theory, this is the HAMILTONIAN CYCLE problem solved in SQL!
--
-- -----------------------------------------------------------------------------
-- 2. SCHEMA & SAMPLE DATA SETUP
-- -----------------------------------------------------------------------------

DROP TABLE IF EXISTS flight_network;
CREATE TABLE flight_network (
    origin_hub INT,
    dest_hub INT,
    distance_miles INT,
    PRIMARY KEY (origin_hub, dest_hub)
);

-- Hubs: 0 (Chicago), 1 (Dallas), 2 (Atlanta), 3 (New York)
-- 4-hub network with bidirectional connections
INSERT INTO flight_network (origin_hub, dest_hub, distance_miles) VALUES
    -- Chicago (0) <-> Dallas (1)
    (0, 1, 920), (1, 0, 920),
    -- Dallas (1) <-> Atlanta (2)
    (1, 2, 780), (2, 1, 780),
    -- Atlanta (2) <-> New York (3)
    (2, 3, 750), (3, 2, 750),
    -- New York (3) <-> Chicago (0)
    (3, 0, 790), (0, 3, 790),
    -- Diagonal route: Chicago (0) <-> Atlanta (2)
    (0, 2, 710), (2, 0, 710);

-- =============================================================================
-- THE BACKTRACKING GRAPH TRAVERSAL (RECURSIVE CTE)
-- =============================================================================
-- How this works:
-- 1. Anchor Member: Start the flight tour at Hub 0 (ARRAY[0]).
-- 2. Recursive Member: Explore outbound flights to dest_hub ONLY if dest_hub
--    has NOT yet been visited (WHERE NOT (f.dest_hub = ANY(t.visited_hubs))).
-- 3. Base Case / Cycle Closure: When length of visited_hubs = 4 (all hubs visited),
--    check if a return flight exists from last_hub back to Hub 0!

WITH RECURSIVE cargo_tour AS (
    -- 1. ANCHOR MEMBER: Start at Chicago (Hub 0)
    SELECT
        0 AS start_hub,
        origin_hub AS current_hub,
        ARRAY[origin_hub] AS visited_hubs,
        0 AS total_distance,
        1 AS hubs_visited_count
    FROM flight_network
    WHERE origin_hub = 0
    GROUP BY origin_hub

    UNION ALL

    -- 2. RECURSIVE MEMBER: Fly to next unvisited hub
    SELECT
        t.start_hub,
        f.dest_hub,
        t.visited_hubs || f.dest_hub,
        t.total_distance + f.distance_miles,
        t.hubs_visited_count + 1
    FROM cargo_tour t
    JOIN flight_network f ON f.origin_hub = t.current_hub
    WHERE t.hubs_visited_count < 4  -- Total hubs = 4
      AND NOT (f.dest_hub = ANY(t.visited_hubs)) -- Prevent revisiting any hub!
)
-- 3. CYCLE COMPLETION: Check return leg to start_hub (Hub 0)
SELECT 
    t.visited_hubs || t.start_hub AS complete_hamiltonian_cycle,
    t.total_distance + return_leg.distance_miles AS total_tour_distance_miles,
    'VALID CLOSED TOUR' AS flight_status
FROM cargo_tour t
JOIN flight_network return_leg 
  ON return_leg.origin_hub = t.current_hub 
 AND return_leg.dest_hub = t.start_hub
WHERE t.hubs_visited_count = 4
ORDER BY total_tour_distance_miles;

-- Expected Output for N = 4 hubs starting at 0:
-- Route 1: {0, 1, 2, 3, 0} -> 920 + 780 + 750 + 790 = 3,240 miles
-- Route 2: {0, 3, 2, 1, 0} -> 790 + 750 + 780 + 920 = 3,240 miles (reversed cycle)
