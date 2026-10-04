-- =============================================================================
-- 🤖 REAL-WORLD SQL: RAT IN A MAZE (WAREHOUSE AGV ROBOT PATHFINDING)
-- CATEGORY: Backtracking / Grid Navigation in SQL
-- =============================================================================
--
-- -----------------------------------------------------------------------------
-- 1. REAL-WORLD SCENARIO & BUSINESS USE CASE
-- -----------------------------------------------------------------------------
-- In smart fulfillment centers (Amazon Robotics, Ocado, Alibaba Cainiao):
-- - Autonomous Mobile Robots (AMRs / AGVs) transport shelving pods across a grid floor.
-- - The floor layout is stored in a database table of grid cells.
-- - Some grid cells are blocked by fixed pillars, charging stations, or fallen items.
-- - A robot at Loading Bay (0, 0) needs to find valid, collision-free paths
--   to the Assembly Packing Station at (3, 3).
--
-- -----------------------------------------------------------------------------
-- 2. SCHEMA & SAMPLE DATA SETUP
-- -----------------------------------------------------------------------------

DROP TABLE IF EXISTS warehouse_grid;
CREATE TABLE warehouse_grid (
    x INT,
    y INT,
    is_blocked BOOLEAN,
    PRIMARY KEY (x, y)
);

-- 4x4 Grid layout:
-- [0] = Open, [X] = Blocked
-- (0,0)[0]  (0,1)[0]  (0,2)[X]  (0,3)[0]
-- (1,0)[0]  (1,1)[0]  (1,2)[0]  (1,3)[0]
-- (2,0)[X]  (2,1)[X]  (2,2)[0]  (2,3)[X]
-- (3,0)[0]  (3,1)[0]  (3,2)[0]  (3,3)[0]  <-- Destination!

INSERT INTO warehouse_grid (x, y, is_blocked) VALUES
    (0, 0, FALSE), (0, 1, FALSE), (0, 2, TRUE),  (0, 3, FALSE),
    (1, 0, FALSE), (1, 1, FALSE), (1, 2, FALSE), (1, 3, FALSE),
    (2, 0, TRUE),  (2, 1, TRUE),  (2, 2, FALSE), (2, 3, TRUE),
    (3, 0, FALSE), (3, 1, FALSE), (3, 2, FALSE), (3, 3, FALSE);

-- =============================================================================
-- THE BACKTRACKING QUERY (RECURSIVE CTE)
-- =============================================================================
-- How this works:
-- 1. Anchor Member: Place the AGV robot at start coordinate (0, 0).
-- 2. Recursive Member: Explore 4 cardinal movements:
--      - Down  (x + 1, y)
--      - Right (x, y + 1)
--      - Up    (x - 1, y)
--      - Left  (x, y - 1)
--    Backtracking Constraints:
--      - Target cell exists and is NOT blocked (is_blocked = FALSE)
--      - Target coordinate has NOT been visited yet on this path (cycle prevention)
-- 3. Base Case:
--      Filter results where current position = Destination (3, 3).

WITH RECURSIVE directions(dx, dy, dir_name) AS (
    VALUES 
        (1, 0, 'D'),   -- Down
        (0, 1, 'R'),   -- Right
        (-1, 0, 'U'),  -- Up
        (0, -1, 'L')   -- Left
),
agv_paths AS (
    -- 1. ANCHOR MEMBER: Robot starts at (0, 0)
    SELECT
        0 AS curr_x,
        0 AS curr_y,
        ARRAY['(0,0)'] AS path_coords,
        '' AS directions_taken,
        1 AS step_count
    FROM warehouse_grid
    WHERE x = 0 AND y = 0 AND is_blocked = FALSE

    UNION ALL

    -- 2. RECURSIVE MEMBER: Explore next adjacent open cell
    SELECT
        g.x,
        g.y,
        p.path_coords || format('(%s,%s)', g.x, g.y),
        p.directions_taken || d.dir_name,
        p.step_count + 1
    FROM agv_paths p
    CROSS JOIN directions d
    JOIN warehouse_grid g 
      ON g.x = p.curr_x + d.dx 
     AND g.y = p.curr_y + d.dy
    WHERE g.is_blocked = FALSE
      AND NOT (format('(%s,%s)', g.x, g.y) = ANY(p.path_coords)) -- Backtracking safety: no looping!
      AND p.step_count < 16 -- Max boundary safety limit
)
SELECT 
    directions_taken,
    step_count,
    path_coords,
    'VALID AGV DISPATCH ROUTE' AS status
FROM agv_paths
WHERE curr_x = 3 AND curr_y = 3
ORDER BY step_count ASC, directions_taken;

-- Sample Output:
-- directions_taken | step_count | path_coords
-- DDRRDR           | 7          | {(0,0),(1,0),(1,1),(1,2),(2,2),(3,2),(3,3)}
