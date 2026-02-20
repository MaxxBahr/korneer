CREATE TABLE if not exists `coffee` (
    `id` int NOT NULL,
    `name` varchar(100) NOT NULL,
    `rating` int NOT NULL,
    `url` varchar(100) NULL,
    `grind_size` int NULL,
    `grind_time` float NULL,
    `extraction_time` float NULL,
    `taste` varchar(100) NULL,
    PRIMARY KEY (`id`)
);
-- sqlite3 coffee.db < schema.sql  