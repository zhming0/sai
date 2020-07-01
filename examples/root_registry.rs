use sai::{component_registry, Component};

use crate::gotham_server::GothamServer;
use crate::foo_controller::FooController;
use crate::tide_server::TideServer;
use crate::db::Db;

component_registry!(RootRegistry, [
    GothamServer,
    Db,
    FooController,
    TideServer
]);
