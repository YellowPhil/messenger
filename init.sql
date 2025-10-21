CREATE KEYSPACE IF NOT EXISTS messenger WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 };


--- ROOMS ---
CREATE TABLE IF NOT EXISTS messenger.rooms (
    room_id UUID PRIMARY KEY,
    name TEXT,
    description TEXT,
    admin_id UUID,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS rooms_by_admin ON messenger.rooms (admin_id);

CREATE TYPE messenger.user_info (
    avatar_url TEXT,
    username TEXT,
);

CREATE TYPE messenger.room_info (
    name TEXT,
    description TEXT,
    avatar_url TEXT,
    role: TEXT
);

CREATE TABLE IF NOT EXISTS messenger.rooms (
    room_id UUID PRIMARY KEY,
    room_info frozen<room_info>,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS messenger.room_members(
    room_id UUID,
    user_id UUID,
    joined_at TIMESTAMP,
    role TEXT,
    PRIMARY KEY (room_id, joined_at)
);

CREATE TABLE IF NOT EXISTS messenger.rooms_by_user (
    user_id UUID,
    room_info frozen<room_info>,
    joined_at TIMESTAMP,
    PRIMARY KEY (user_id, joined_at)
);


--- INVITES ---
CREATE TABLE IF NOT EXISTS messenger.invites_by_user (
    user_id UUID,
    room_id UUID,
    invited_by UUID,
    invited_at TIMESTAMP,
    PRIMARY KEY (user_id, room_id)
);

CREATE TABLE IF NOT EXISTS messenger.invites_by_room (
    room_id UUID,
    invite_id UUID,
    user_id UUID,
    invited_at TIMESTAMP,
    PRIMARY KEY (room_id, invite_id)
);

--- MESSAGES ---
-- TODO: add bucketing by day
CREATE TABLE IF NOT EXISTS messenger.messages_by_room (
    room_id UUID,
    timestamp TIMESTAMP,
    message_id UUID,
    sender_info frozen<user_info>,
    sender_id UUID,
    content TEXT,
    PRIMARY KEY (room_id, timestamp)
);