-- =============================================================================
-- 🛒 REAL-WORLD SQL: ALL COMBINATIONS OF SIZE K (PRODUCT BUNDLE GENERATOR)
-- CATEGORY: Backtracking / Combinatorics in SQL
-- =============================================================================
--
-- -----------------------------------------------------------------------------
-- 1. REAL-WORLD SCENARIO & BUSINESS USE CASE
-- -----------------------------------------------------------------------------
-- An e-commerce platform (like Amazon or Shopify) wants to create promotional
-- product bundles:
-- - You have a catalog of N products in a category (e.g., Gaming Accessories).
-- - Marketing wants to generate all possible unique K-item promotional bundles
--   (e.g., K = 2 or K = 3) to test bundle discount conversion rates.
-- - Order does NOT matter: Bundle (Keyboard, Mouse) is identical to (Mouse, Keyboard).
-- - No item can be repeated within the same bundle.
--
-- -----------------------------------------------------------------------------
-- 2. SCHEMA & SAMPLE DATA SETUP
-- -----------------------------------------------------------------------------

DROP TABLE IF EXISTS products;
CREATE TABLE products (
    product_id INT PRIMARY KEY,
    product_name VARCHAR(100),
    price DECIMAL(10, 2)
);

-- Insert 4 products (N = 4, indexed 0 to 3)
INSERT INTO products (product_id, product_name, price) VALUES
    (0, 'Gaming Mouse', 49.99),
    (1, 'Mechanical Keyboard', 89.99),
    (2, 'RGB Mousepad', 19.99),
    (3, 'Surround Headset', 79.99);

-- =============================================================================
-- PATTERN A: SELF-JOIN (Best when K is small and known, e.g. K = 2)
-- =============================================================================
-- Notice the JOIN condition: a.product_id < b.product_id
-- This single '<' inequality strictly enforces:
--   1. No self-pairing (a.product_id != b.product_id)
--   2. No reversed duplicates (never pairs (1, 0) if (0, 1) was already generated)!

SELECT 
    a.product_id AS item_1_id,
    a.product_name AS item_1_name,
    b.product_id AS item_2_id,
    b.product_name AS item_2_name,
    (a.price + b.price) * 0.85 AS bundle_price_15_pct_off
FROM products a
JOIN products b ON a.product_id < b.product_id
ORDER BY a.product_id, b.product_id;

-- Expected Output for N = 4, K = 2 (6 unique bundles):
-- (0, 1) -> Mouse + Keyboard
-- (0, 2) -> Mouse + Mousepad
-- (0, 3) -> Mouse + Headset
-- (1, 2) -> Keyboard + Mousepad
-- (1, 3) -> Keyboard + Headset
-- (2, 3) -> Mousepad + Headset


-- =============================================================================
-- PATTERN B: RECURSIVE CTE (Universal: Works for ANY K dynamically!)
-- =============================================================================
-- This is the exact SQL equivalent of Backtracking / DFS:
-- - The Anchor Member starts combinations of size 1.
-- - The Recursive Member adds candidate items where item_id > last_item_id.
-- - The Termination Condition stops recursion when combo_size reaches K.

WITH RECURSIVE combo_generator AS (
    -- 1. ANCHOR MEMBER (Seed with combinations of size 1)
    SELECT
        ARRAY[product_id] AS bundle_ids,
        ARRAY[product_name::TEXT] AS bundle_names,
        product_id AS last_id,
        price AS total_price,
        1 AS current_size
    FROM products

    UNION ALL

    -- 2. RECURSIVE MEMBER (CHOOSE & EXPLORE next item where id > last_id)
    SELECT
        c.bundle_ids || p.product_id,
        c.bundle_names || p.product_name::TEXT,
        p.product_id,
        c.total_price + p.price,
        c.current_size + 1
    FROM combo_generator c
    JOIN products p ON p.product_id > c.last_id
    WHERE c.current_size < 3  -- Target K = 3
)
SELECT 
    bundle_ids,
    bundle_names,
    total_price,
    ROUND(total_price * 0.80, 2) AS bundle_discount_20_pct
FROM combo_generator
WHERE current_size = 3
ORDER BY bundle_ids;

-- Expected Output for N = 4, K = 3 (4 unique bundles):
-- {0, 1, 2} -> Mouse, Keyboard, Mousepad
-- {0, 1, 3} -> Mouse, Keyboard, Headset
-- {0, 2, 3} -> Mouse, Mousepad, Headset
-- {1, 2, 3} -> Keyboard, Mousepad, Headset
