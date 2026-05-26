CREATE INDEX idx_products_active ON products (created_at DESC) WHERE status = 'active';

-- NOTE:
-- This index was made because of 'fetch_products' function in the products repo
-- because every other function in the same repo uses 'id' in WHERE,
-- and primary keys are indexed by default, this does most of the heavy lifting, except for this one.
