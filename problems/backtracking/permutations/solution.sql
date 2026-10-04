-- =============================================================================
-- 📦 REAL-WORLD SQL: PERMUTATIONS (LOGISTICS MULTI-STOP ROUTE SEQUENCER)
-- CATEGORY: Backtracking / Permutation Sequences in SQL
-- =============================================================================
--
-- -----------------------------------------------------------------------------
-- 1. REAL-WORLD SCENARIO & BUSINESS USE CASE
-- -----------------------------------------------------------------------------
-- In supply chain dispatching & route optimization (FedEx, Amazon, DoorDash):
-- - A delivery van has N priority packages to deliver across distinct locations.
-- - Before running expensive road-network routing engines, dispatchers need
--   to generate all possible stop sequences (permutations of stops) to evaluate
--   different customer priority rankings and delivery schedules.
-- - Order matters! [Depot -> Stop A -> Stop B] != [Depot -> Stop B -> Stop A].
-- - Each stop must be visited EXACTLY ONCE.
--
-- -----------------------------------------------------------------------------
-- 2. SCHEMA & SAMPLE DATA SETUP
-- -----------------------------------------------------------------------------

DROP TABLE IF EXISTS delivery_stops;
CREATE TABLE delivery_stops (
    stop_id INT PRIMARY KEY,
    customer_name VARCHAR(100),
    neighborhood VARCHAR(100),
    urgency_tier INT
);

INSERT INTO delivery_stops (stop_id, customer_name, neighborhood, urgency_tier) VALUES
    (1, 'TechCorp HQ', 'Downtown', 1),
    (2, 'Metro Hospital', 'Northside', 1),
    (3, 'Retail Hub', 'West End', 2),
    (4, 'Airport Freight', 'East Bay', 2);

-- =============================================================================
-- THE BACKTRACKING QUERY (RECURSIVE CTE)
-- =============================================================================
-- How this works:
-- 1. Anchor Member: Pick any stop as the 1st delivery stop (step_count = 1).
-- 2. Recursive Member: Explore the next stop from all available stops.
--    Constraint (The Backtracking Check):
--      WHERE NOT (next_stop.stop_id = ANY(visited_stops))
-- 3. Base Case / Termination:
--      Recursion halts when array_length(visited_stops, 1) reaches total stops (4).
-- 4. Result: All 4! = 24 unique dispatch itineraries!

WITH RECURSIVE total_count AS (
    SELECT COUNT(*) AS total_stops FROM delivery_stops
),
route_permutations AS (
    -- 1. ANCHOR MEMBER: Start a sequence with any delivery stop
    SELECT
        ARRAY[stop_id] AS route_order,
        ARRAY[customer_name::TEXT] AS stop_names,
        1 AS step_count
    FROM delivery_stops

    UNION ALL

    -- 2. RECURSIVE MEMBER: Pick the next unvisited stop (Backtracking check!)
    SELECT
        r.route_order || s.stop_id,
        r.stop_names || s.customer_name::TEXT,
        r.step_count + 1
    FROM route_permutations r
    JOIN delivery_stops s ON NOT (s.stop_id = ANY(r.route_order)) -- Pruning: Must not be already visited!
    CROSS JOIN total_count tc
    WHERE r.step_count < tc.total_stops
)
SELECT 
    route_order,
    stop_names,
    array_to_string(stop_names, ' -> ') AS full_itinerary
FROM route_permutations, total_count
WHERE step_count = total_count.total_stops
ORDER BY route_order;

-- Expected Output:
-- Exactly 4! = 24 rows showing every complete itinerary:
-- {1,2,3,4} -> 'TechCorp HQ -> Metro Hospital -> Retail Hub -> Airport Freight'
-- {1,2,4,3} -> 'TechCorp HQ -> Metro Hospital -> Airport Freight -> Retail Hub'
-- ...
-- {4,3,2,1} -> 'Airport Freight -> Retail Hub -> Metro Hospital -> TechCorp HQ'
