CREATE TABLE refresh_tokens (
    token_id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID REFERENCES users (id),
    token_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL
);