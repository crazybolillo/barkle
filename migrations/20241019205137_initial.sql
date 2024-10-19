CREATE TABLE portfolio (
   id INTEGER PRIMARY KEY,
   name TEXT UNIQUE,
   description TEXT
);

CREATE TABLE asset (
   id INTEGER PRIMARY KEY,
   portfolio_id INTEGER NOT NULL,
   name TEXT,
   UNIQUE (portfolio_id, name),
   FOREIGN KEY (portfolio_id) REFERENCES portfolio(id)
);

CREATE TABLE valuation (
   id INTEGER PRIMARY KEY,
   asset_id INTEGER NOT NULL,
   at DATETIME  DEFAULT CURRENT_TIMESTAMP NOT NULL,
   value REAL,
   FOREIGN KEY (asset_id) REFERENCES asset(id)
);
