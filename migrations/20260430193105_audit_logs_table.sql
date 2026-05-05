-- this is still an experimental table, not implemented in code.

CREATE TABLE audit.audit_log (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    actor_id UUID, -- NULL if system
    actor_type TEXT NOT NULL, -- 'user', 'admin', 'system'
    actor_ip INET, -- NULL if system
    actor_user_agent TEXT, -- browser
    action TEXT NOT NULL,
    entity_type TEXT, -- 'user' / 'product'
    entity_id UUID, -- record id
    old_value JSONB, -- state before action
    new_value JSONB -- state after action
);

