-- Add migration script here
CREATE TABLE products (
    -- This is NOT full table implementation, more columns will be added
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR NOT NULL,
    description TEXT NOT NULL,
    price INT NOT NULL CHECK ( price >= 0 ), -- INT is used by stripe is what I found
    stock_quantity INT NOT NULL CHECK ( stock_quantity >= 0 ),
    status VARCHAR(20) DEFAULT 'draft' CHECK ( status in ('draft', 'active', 'archived')), -- product visibility
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);