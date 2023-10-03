// Copyright 2023 Jaime Alvarez Fernandez

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use nighty_night::{check_db_initialisation, create_app, set_environment};

#[tokio::main]
async fn main() {
    set_environment();
    check_db_initialisation().await;
    create_app().await
}
