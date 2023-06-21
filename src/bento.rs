use std::collections::HashMap;

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serenity::builder::{CreateMessage, EditMessage};
use serenity::http::{CacheHttp, Http};
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::prelude::InteractionResponseType;
use serenity::model::{prelude::Message, user::User};
use serenity::prelude::Context;
use serenity::{futures::lock::Mutex, prelude::TypeMapKey};

use crate::database::shop_database::Shop;
use crate::roulette::{RouletteClient, RouletteClientData, RouletteCreateRequest, RouletteItem};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BentoState {
    VOTE,
    ROULETTE,
    END,
}

pub struct BentoInstance {
    pub start_user: User,
    pub message: Message,
    pub vote: HashMap<u64, (User, String)>,
    pub roulette_id: Option<String>,
    pub roulette_secret: Option<String>,
    pub state: BentoState,
    pub shops: Vec<Shop>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum BentoEventAction {
    VOTE_SHOP,
    END_VOTE,
    START_ROULETTE,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BentoEvent {
    pub id: u64,
    pub action: BentoEventAction,
    pub arg: String,
}

impl BentoInstance {
    async fn create_roulette_ui(&mut self, roulette_client: &RouletteClient) -> String {
        let mut items = vec![];

        for vote in self.vote.clone() {
            items.push(RouletteItem { label: vote.1 .1 });
        }

        let request = RouletteCreateRequest {
            title: String::from("ARCH弁当"),
            items,
        };

        let response = roulette_client.create_roulette(request).await;

        self.roulette_id = Some(response.id.clone());
        self.roulette_secret = Some(response.secret);

        response.id
    }

    pub async fn interaction(
        &mut self,
        ctx: Context,
        event: BentoEvent,
        interaction: MessageComponentInteraction,
    ) {
        match event.action {
            BentoEventAction::VOTE_SHOP => {
                if let Some(vote) = self.vote.get_mut(&interaction.user.id.0) {
                    vote.1 = event.arg.clone();
                } else {
                    self.vote.insert(
                        interaction.user.id.0,
                        (interaction.user.clone(), event.arg.clone()),
                    );
                }

                interaction
                    .create_interaction_response(&ctx.http, |f| {
                        f.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|d| {
                                d.content(format!("`{}` へ投票しました。", event.arg))
                                    .ephemeral(true)
                            })
                    })
                    .await
                    .unwrap();

                self.edit_message(&ctx.http).await;
            }
            BentoEventAction::END_VOTE => {
                if self.start_user.id.0 == interaction.user.id.0 {
                    self.state = BentoState::ROULETTE;

                    interaction
                        .create_interaction_response(&ctx.http, |f| {
                            f.kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|d| {
                                    d.content("投票を締め切りました。ルーレットを作成します。")
                                        .ephemeral(true)
                                })
                        })
                        .await
                        .unwrap();

                    let roulette_client = {
                        let data_read = ctx.data.read().await;
                        data_read
                            .get::<RouletteClientData>()
                            .expect("Cannot get RouletteClientData")
                            .clone()
                    };
                    let roulette_client = roulette_client;

                    let roulette_id = self.create_roulette_ui(&roulette_client).await;

                    self.message
                        .channel_id
                        .send_message(&ctx.http, |f| {
                            f.content(format!(
                                "## ルーレットが作成されました。下のリンクから参加できます。\n{}/roulette/{}",
                                roulette_client.base_url, roulette_id
                            ))
                        })
                        .await
                        .unwrap();

                    self.edit_message(&ctx.http).await;
                } else {
                    interaction
                        .create_interaction_response(&ctx.http, |f| {
                            f.kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|d| {
                                    d.content("投票の作成者のみ締め切り可能です。")
                                        .ephemeral(true)
                                })
                        })
                        .await
                        .unwrap();
                }
            }
            BentoEventAction::START_ROULETTE => {
                if self.start_user.id.0 == interaction.user.id.0 {
                    self.state = BentoState::END;

                    interaction
                        .create_interaction_response(&ctx.http, |f| {
                            f.kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|d| {
                                    d.content("ルーレットを開始しました。").ephemeral(true)
                                })
                        })
                        .await
                        .unwrap();

                    let roulette_client = {
                        let data_read = ctx.data.read().await;
                        data_read
                            .get::<RouletteClientData>()
                            .expect("Cannot get RouletteClientData")
                            .clone()
                    };
                    let roulette_client = roulette_client;

                    roulette_client
                        .start_roulette(
                            self.roulette_id.clone().unwrap(),
                            self.roulette_secret.clone().unwrap(),
                        )
                        .await;

                    self.edit_message(&ctx.http).await;
                } else {
                    interaction
                        .create_interaction_response(&ctx.http, |f| {
                            f.kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|d| {
                                    d.content("投票の作成者のみ可能です。").ephemeral(true)
                                })
                        })
                        .await
                        .unwrap();
                }
            }
        }
    }

    pub async fn edit_message(&mut self, cache_http: &impl CacheHttp) {
        let id = self.message.clone().id.0;
        self.message
            .edit(cache_http, |f| {
                f.embed(|e| {
                    let mut base = e
                        .footer(|a| {
                            a.text(self.start_user.name.clone()).icon_url(
                                self.start_user
                                    .avatar_url()
                                    .unwrap_or(self.start_user.default_avatar_url()),
                            )
                        })
                        .title("ARCH弁当");

                    let mut votes: HashMap<String, String> = HashMap::new();

                    for vote in self.vote.clone() {
                        let user = vote.1 .0;
                        if let Some(t) = votes.get_mut(&vote.1 .1) {
                            t.push_str(&format!(", {}", user.name));
                        } else {
                            votes.insert(vote.1 .1.clone(), user.name);
                        }
                    }

                    for vote in votes {
                        base = base.field(vote.0, vote.1, true);
                    }

                    base
                })
                .components(|c| {
                    match self.state {
                        BentoState::VOTE => {
                            c.create_action_row(|a| {
                                let mut a = a;
                                for shop in self.shops.clone() {
                                    let event = BentoEvent {
                                        id,
                                        action: BentoEventAction::VOTE_SHOP,
                                        arg: shop.name.clone(),
                                    };
                                    a = a.create_button(|cb| {
                                        cb.style(ButtonStyle::Primary)
                                            .label(shop.name.clone())
                                            .custom_id(serde_json::to_string(&event).unwrap())
                                    });
                                }

                                a
                            })
                            .create_action_row(|a| {
                                a.create_button(|b| {
                                    let event = BentoEvent {
                                        id,
                                        action: BentoEventAction::END_VOTE,
                                        arg: String::default(),
                                    };

                                    b.style(ButtonStyle::Secondary)
                                        .label("投票を締め切る")
                                        .custom_id(serde_json::to_string(&event).unwrap())
                                })
                            });
                        }
                        BentoState::ROULETTE => {
                            c.create_action_row(|a| {
                                a.create_button(|b| {
                                    let event = BentoEvent {
                                        id,
                                        action: BentoEventAction::START_ROULETTE,
                                        arg: String::default(),
                                    };

                                    b.style(ButtonStyle::Success)
                                        .label("ルーレットを回す")
                                        .custom_id(serde_json::to_string(&event).unwrap())
                                })
                            });
                        }
                        BentoState::END => {}
                    }

                    c
                })
            })
            .await
            .unwrap();
    }
}

pub struct BentoInstanceData;

impl TypeMapKey for BentoInstanceData {
    type Value = Arc<Mutex<HashMap<u64, BentoInstance>>>;
}
