use api::fetch_orders;
use sycamore::{prelude::*, component, suspense::Suspense};
use sycamore_router::{Route, Router, HistoryIntegration};

use crate::api::OrderView;

mod api;

#[derive(Route, Clone, Debug)]
enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/order-tracker/<order_id>")]
    Tracker { order_id: String },
    #[not_found]
    NotFound
}

#[component]
async fn OrdersView<G: Html>(ctx: ScopeRef<'_>) -> View<G> {
    let res = fetch_orders().await.unwrap_or_default();
    let orders = ctx.create_signal(res);

    view! { ctx,
        div {
            h3 { "All Orders" }
            table {
                tr {
                    th { "Order Id:" }
                    th { "Order Status:" }
                }
                Indexed {
                    iterable: orders,
                    view: |ctx, OrderView{ id, order_status, .. }| {
                        view! { ctx,
                                tr {
                                    th { (id) }
                                    th { (order_status.to_string()) }
                                }
                        }
                    }
                }
            }
        }
    }
}

fn switch<'a, G: Html>(ctx: ScopeRef<'a>, route: &'a ReadSignal<AppRoutes>) -> View<G> {
    let view = ctx.create_memo(on([route], move || match route.get().as_ref().clone() {
        AppRoutes::Index => view! { ctx,
            Suspense {
                fallback: view! { ctx, "Loading..." },
                OrdersView {}
            }
        },

        AppRoutes::Tracker { order_id } => view! { ctx, (format!("Pizza Tracker {}", order_id)) },

        AppRoutes::NotFound => view! { ctx, "404" }
    }));

    view! { ctx,
        div  {
            ((*view.get()).clone())
        }
    }
}

#[component]
fn App<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        Router {
            integration: HistoryIntegration::new(),
            view: switch
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|ctx| view! { ctx,
        App {}
    });
}
