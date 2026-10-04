-- =============================================================================
-- 💳 REAL-WORLD SQL: SUBSET SUM (ACCOUNTS RECEIVABLE INVOICE RECONCILIATION)
-- CATEGORY: Backtracking / 0-1 Knapsack in SQL
-- =============================================================================
--
-- -----------------------------------------------------------------------------
-- 1. REAL-WORLD SCENARIO & BUSINESS USE CASE
-- -----------------------------------------------------------------------------
-- In corporate finance & enterprise ERPs (SAP, NetSuite, Stripe Billing):
-- - A customer sends a wire transfer payment of exactly $100.00.
-- - However, the customer forgot to include the remittance advice (the list of 
--   which invoices they intended to pay)!
-- - In your database, the customer has multiple outstanding unpaid invoices.
-- - The Automated Reconciliation Engine must find which subset of invoices
--   sums EXACTLY to the $100.00 wire transfer payment.
--
-- -----------------------------------------------------------------------------
-- 2. SCHEMA & SAMPLE DATA SETUP
-- -----------------------------------------------------------------------------

DROP TABLE IF EXISTS unpaid_invoices;
CREATE TABLE unpaid_invoices (
    invoice_id INT PRIMARY KEY,
    description VARCHAR(100),
    amount DECIMAL(10, 2)
);

INSERT INTO unpaid_invoices (invoice_id, description, amount) VALUES
    (101, 'Cloud Hosting - March', 15.00),
    (102, 'Domain Renewal', 25.00),
    (103, 'API Support Tier 2', 35.00),
    (104, 'Security Audit Fee', 40.00),
    (105, 'Database Backup Storage', 50.00),
    (106, 'Dedicated IP Allocation', 60.00);

-- =============================================================================
-- THE BACKTRACKING QUERY (RECURSIVE CTE)
-- =============================================================================
-- How this works:
-- 1. Anchor Member: Start with each single invoice as an initial candidate sum.
-- 2. Recursive Member: Join with subsequent invoices (i.invoice_id > last_id)
--    to accumulate running totals WITHOUT duplicate permutations.
-- 3. Pruning: Terminate recursion if running total exceeds target ($100.00).
-- 4. Result: Select subsets where accumulated_amount = 100.00!

WITH RECURSIVE invoice_combinations AS (
    -- 1. ANCHOR MEMBER: Each individual invoice starts a path
    SELECT
        ARRAY[invoice_id] AS invoice_ids,
        ARRAY[description::TEXT] AS invoice_descriptions,
        amount AS total_amount,
        invoice_id AS last_id
    FROM unpaid_invoices
    WHERE amount <= 100.00

    UNION ALL

    -- 2. RECURSIVE MEMBER: Explore next invoice (only look forward to avoid duplicates)
    SELECT
        c.invoice_ids || i.invoice_id,
        c.invoice_descriptions || i.description::TEXT,
        c.total_amount + i.amount,
        i.invoice_id
    FROM invoice_combinations c
    JOIN unpaid_invoices i ON i.invoice_id > c.last_id
    WHERE c.total_amount + i.amount <= 100.00  -- Pruning: Stop early if sum exceeds target!
)
SELECT 
    invoice_ids,
    invoice_descriptions,
    total_amount,
    'MATCH FOUND: RECONCILE PAYMENT' AS action_status
FROM invoice_combinations
WHERE total_amount = 100.00
ORDER BY invoice_ids;

-- Expected Output for Wire Transfer = $100.00:
-- Match 1: {101, 102, 106} -> $15.00 + $25.00 + $60.00 = $100.00
-- Match 2: {102, 103, 104} -> $25.00 + $35.00 + $40.00 = $100.00
-- Match 3: {104, 106}      -> $40.00 + $60.00 = $100.00
-- Match 4: {105, 105 is not duplicate, etc.}
