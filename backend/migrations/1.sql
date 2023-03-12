CREATE TABLE monitoring.servers (
	id INTEGER NOT NULL,
	name text NOT NULL
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_general_ci;
ALTER TABLE monitoring.servers ADD CONSTRAINT servers_PK PRIMARY KEY (id);
ALTER TABLE monitoring.servers MODIFY COLUMN id int(11) auto_increment NOT NULL;

CREATE TABLE monitoring.organizations (
	id int NOT NULL,
	name text NOT NULL
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_general_ci;
ALTER TABLE monitoring.organizations ADD CONSTRAINT organizations_PK PRIMARY KEY (id);
ALTER TABLE monitoring.organizations MODIFY COLUMN id int(11) auto_increment NOT NULL;
ALTER TABLE monitoring.organizations ADD CONSTRAINT organizations_UN UNIQUE KEY (name);
