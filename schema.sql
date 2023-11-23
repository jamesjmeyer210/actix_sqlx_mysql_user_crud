DROP TABLE IF EXISTS `users_to_groups`;
DROP TABLE IF EXISTS `user_events`;
DROP TABLE IF EXISTS `user_deltas`;
DROP TABLE IF EXISTS `groups`;
DROP TABLE IF EXISTS `users`;

CREATE TABLE IF NOT EXISTS users 
(
	id TEXT PRIMARY KEY NOT NULL UNIQUE,
	name TEXT NOT NULL UNIQUE,
    password BLOB NOT NULL,
	email BLOB NOT NULL UNIQUE,
    email_verified INTEGER NOT NULL,
    phone BLOB UNIQUE,
    phone_verified INTEGER NOT NULL,
    public_key BLOB UNIQUE,
	created_on_utc TEXT NOT NULL,
	deleted_on_utc TEXT
);

/*
UserEvent
- Creation
- Deletion
- Username change
- Email change
- Email verified
- Password change
- Added to role
- Removed from role
*/
CREATE TABLE IF NOT EXISTS user_events
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id TEXT NOT NULL,
    event INTEGER NOT NULL,
    ip_address INTEGER NOT NULL,
    created_on_utc TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE IF NOT EXISTS user_deltas
(
    event_id INTEGER NOT NULL,
    name TEXT,
    email BLOB,
    FOREIGN KEY (event_id) REFERENCES user_events (id)
);

CREATE TABLE IF NOT EXISTS `realms`
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL UNIQUE,
    created_on_utc TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS `users_to_realms`
(
    user_id TEXT NOT NULL,
    realm_id TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (realm_id) REFERENCES realms(id)
);

CREATE TABLE IF NOT EXISTS `roles`
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL UNIQUE,
    max INTEGER NOT NULL NULL,
    realm_id INTEGER NOT NULL,
    created_on_utc TEXT NOT NULL,
    FOREIGN KEY(realm_id) REFERENCES realms(id)
);
            
CREATE TABLE IF NOT EXISTS `users_to_roles`
(
    `user_id` TEXT NOT NULL,
    `role_id` INTEGER NOT NULL,
    FOREIGN KEY (`user_id`) REFERENCES `users`(`id`),
    FOREIGN KEY (`role_id`) REFERENCES `roles`(`id`)
);