// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use datafusion::prelude::SessionContext;
use datafusion_table_provider_flight::sql::FlightSqlDriver;
use datafusion_table_provider_flight::FlightTableFactory;
use std::sync::Arc;

/// Prerequisites:
/// ```
/// $ brew install roapi
/// $ roapi -t taxi=https://d37ci6vzurychx.cloudfront.net/trip-data/yellow_tripdata_2024-01.parquet
/// ```
#[tokio::main]
async fn main() -> datafusion::common::Result<()> {
    let ctx = SessionContext::new();
    ctx.state_ref().write().table_factories_mut().insert(
        "FLIGHT_SQL".into(),
        Arc::new(FlightTableFactory::new(
            Arc::new(FlightSqlDriver::default()),
        )),
    );
    let _ = ctx
        .sql(
            r#"
            CREATE EXTERNAL TABLE trip_data STORED AS FLIGHT_SQL
            LOCATION 'http://localhost:32010'
            OPTIONS (
                'flight.sql.query' 'SELECT * FROM taxi'
            )
        "#,
        )
        .await?;

    let df = ctx
        .sql(
            r#"
            SELECT "VendorID", COUNT(*), SUM(passenger_count), SUM(total_amount)
            FROM trip_data
            GROUP BY "VendorID"
            ORDER BY COUNT(*) DESC
        "#,
        )
        .await?;
    df.clone().explain(true, false)?.show().await?;
    df.show().await
}
