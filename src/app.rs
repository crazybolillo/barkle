use crate::{PortfolioArgs, PortfolioCommands};
use sqlx::{Pool, Sqlite, SqlitePool};
use tabled::{Table, Tabled};

const QUERY_PORTFOLIO_SHOW_ALL: &str = "
SELECT
    p.name, COUNT(a.id) AS asset_count, ROUND(SUM(v.value), 2) AS value
FROM
    portfolio p
LEFT JOIN
    asset a
ON
    a.portfolio_id = p.id
LEFT JOIN
    valuation v
ON
    v.id IN (SELECT id FROM (SELECT id, MAX(at) FROM valuation v2 WHERE v2.asset_id = a.id GROUP BY v2.asset_id))
GROUP BY
    p.name"
;

#[derive(Debug, sqlx::FromRow, Tabled)]
struct PortfolioAllRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Assets")]
    asset_count: i32,
    #[tabled(rename = "Valuation")]
    value: f64,
}

pub struct App {
    pool: Pool<Sqlite>,
}

impl App {
    pub async fn new(file: &str) -> Self {
        let conn = SqlitePool::connect(file).await.unwrap();
        App { pool: conn }
    }

    pub async fn init(&mut self) {
        // TODO: Run SQLX migrations
    }

    pub async fn portfolio(&self, args: PortfolioArgs) {
        match args.cmd {
            PortfolioCommands::Add { name } => {
                sqlx::query("INSERT INTO portfolio (name) VALUES ($1)")
                    .bind(name)
                    .execute(&self.pool)
                    .await
                    .unwrap();
            }
            PortfolioCommands::Delete { .. } => {}
            PortfolioCommands::Show { name } => {
                if name == "all" {
                    let mut rows = sqlx::query_as::<_, PortfolioAllRow>(QUERY_PORTFOLIO_SHOW_ALL)
                        .fetch_all(&self.pool)
                        .await
                        .unwrap();

                    let mut total_value = 0.0;
                    let mut total_assets = 0;
                    for row in rows.iter() {
                        total_value += row.value;
                        total_assets += row.asset_count;
                    }
                    rows.push(PortfolioAllRow{
                        name: String::new(),
                        asset_count: total_assets,
                        value: total_value,
                    });

                    println!("{}", Table::new(rows))
                }
            }
        }
    }
}
