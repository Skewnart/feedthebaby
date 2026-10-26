use topcoat::{
    Result,
    router::{Slot, layout, module_router, page},
    view::{View, view},
    asset::{AssetBundle, RouterBuilderAssetExt},
    tailwind
};
use topcoat::context::Cx;
use topcoat::font::fontsource::fontsource_font;
use topcoat::icon::{icon, iconify};
use topcoat::icon::iconify::iconify_icon;
use topcoat::router::RouterBuilderDiscoverExt;
use topcoat::runtime::{shard, signal, Event, Signal};
use topcoat::view::{attributes, component};
use crate::components::badge::{badge, BadgeVariant};
use crate::components::button::{button, ButtonSize, ButtonVariant};
use crate::components::card::{card, card_header, card_title};
use crate::components::table::{table, table_body, table_cell, table_footer, table_head, table_header, table_row};
use crate::models::aliment::Aliment;

mod components;
mod models;

iconify::include!("lucide");

const ALIMENTS: [Aliment; 3] = [
    Aliment::new("Pomme", 12, "12/11/2026"),
    Aliment::new("PDT", 20, "01/12/2026"),
    Aliment::new("Butternut", 5, "30/10/2026")
];

#[tokio::main]
async fn main() {
    let router = module_router!()
        .discover()
        .assets(AssetBundle::load().unwrap())
        .build();

    topcoat::start(router).await.unwrap();
}

#[layout]
async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head>
                topcoat::dev::script()
                topcoat::font::link(font: fontsource_font!(MONTSERRAT, host: Asset))
                // The stylesheet build.rs generated, served as an asset.
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
            </head>

            <body
                class="flex min-h-screen items-center justify-center bg-slate-100 font-sans"
            >
                (slot)
            </body>
        </html>
    })
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    // let dark = signal(cx, || false);

    Ok(view! {
        <main
            id="overview"
            class="mx-auto w-full max-w-6xl scroll-mt-20 px-6 py-12"
        >
            <div
                class="flex flex-col-reverse items-start justify-between gap-6 sm:flex-row"
            >
                <header class="max-w-2xl">
                    <h1 class="text-4xl font-bold tracking-tight">
                        "Qu'est-ce qui nous reste ?"
                    </h1>
                    <p class="mt-3 text-muted-foreground">
                        "Dans le "
                        <code class="text-foreground">"congélateur"</code>
                        ", principalement"
                    </p>
                </header>
            </div>

        <div
            id="components"
            class="mt-14 scroll-mt-20"
        >
            <div class="mb-4 scroll-mt-20 break-inside-avoid">
                aliments_list()
            </div>
        </div>
        </main>
    })
}

/// Deployment statuses and their badge styles.
const STATUSES: [(&str, BadgeVariant); 4] = [
    ("Live", BadgeVariant::Primary),
    ("Building", BadgeVariant::Secondary),
    ("Queued", BadgeVariant::Outline),
    ("Failed", BadgeVariant::Destructive),
];

/// The badge variant the deployment status `status` shows in.
fn status_variant(status: &str) -> BadgeVariant {
    // TODO : Il faut changer la gestion du badge pour le temps qu'il reste à une date

    STATUSES
        .iter()
        .find(|(known, _)| *known == status)
        .map_or(BadgeVariant::default(), |(_, variant)| *variant)
}

/// A table of deployments with pagination underneath.
#[component]
async fn aliments_list(cx: &Cx) -> Result<impl View> {
    let rows = &ALIMENTS;
    let test_signal = signal(cx, || false);

    Ok(view! {
        card(
            card_header(
                card_title("Format 10g (les ptits ronds)")
                // card_description("Deployment rows with badges and pagination.")
            )
            // The card pads its sections rather than itself, so the table can
            // span its full width.
            table(
                // table_caption("Deployments")
                table_header(
                    table_row(
                        table_head("Aliment")
                        table_head("Combien ?")
                        table_head("Date d'expiration")
                    )
                )
                table_body(
                    for aliment in rows.iter().copied() {
                        table_row(
                            table_cell(
                                attrs: attributes! { class="font-mono" },
                                (aliment.name)
                            )
                            table_cell(
                                <div class="flex items-center gap-4">
                                button(
                                    size: ButtonSize::Icon,
                                    variant: ButtonVariant::Outline,
                                    attrs: attributes! {
                                        @click=$(|_e: Event| update_quantity(aliment, -1i8))
                                    }
                                    icon(data: lucide::MINUS, label: "Enlever 1")
                                )
                                (aliment.quantity)
                                button(
                                    size: ButtonSize::Icon,
                                    variant: ButtonVariant::Outline,
                                    icon(data: lucide::PLUS, label: "Ajouter 1")
                                )
                                </div>
                            )
                            table_cell(badge(variant: status_variant(aliment.date), (aliment.date)))
                        )
                    }
                )
                table_footer(
                    table_row(
                        table_cell("Total")
                        table_cell(attrs: attributes! { colspan="2" },
                            (format!("{} aliments", rows.len()))
                        )
                    )
                )
            )
        )
    })
}

fn update_quantity(aliment: &Aliment, delta: i8) {

}