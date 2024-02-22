use makepad_widgets::*;
use std::iter;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;

    Hero = <View> {
        width: Fill, height: Fit,
        flow: Overlay,
        align: {y: 1., x: 1.}
        banner = <Image> {
            width: Fill, height: 200.0
            source: (IMG_BANNER),
        }
        content = <View> {
            width: Fit, height: Fit
            align: {y: 0.5}
            username = <H2> {
                width: Fit, height: Fit
                text: "Test 減活乗治外進"
            }
            avatar = <Image> {
                source: (IMG_DEFAULT_AVATAR),
                width: 50., height: 50.
            }
        }
    }

    TextPost = <View> {
        width: Fill, height: Fit,
        flow: Right,
        padding: <MSPACE_2> {},
        spacing: (SPACE_2),

        avatar = <Image> {
            source: (IMG_DEFAULT_AVATAR),
            width: 36., height: 36.
        }

        content = <View> {
            width: Fill, height: Fit
            flow: Down,
            spacing: (SPACE_2),

            username = <Pbold> { text:"Josh" }
            text = <Pbold> { text:"Lorem ipsum dolor sit amet, consectetur" }
        }
    }

    ImagePost = <View> {
        width: Fill, height: Fit,
        flow: Right,
        padding: <MSPACE_2> {},
        spacing: (SPACE_2),

        avatar = <Image> {
            source: (IMG_DEFAULT_AVATAR),
            width: 36., height: 36.
        }

        content = <View> {
            width: Fill, height: Fit,
            flow: Down,
            spacing: (SPACE_2),

            username = <Pbold> { text:"Josh" }
            text = <Pbold> { text:"Lorem ipsum dolor sit amet, consectetur" }

            images = <View> {
                width: Fill, height: 110.,
                flow: Right,
                spacing: (SPACE_2),

                image_1 = <Image> {
                    width: 90., height: 110.
                    source: (IMG_POST1),
                }

                image_2 = <Image> {
                    width: 180., height: 110.,
                    source: (IMG_POST2),
                }
            }
        }
    }

    MomentList = {{MomentList}} {
        width: Fill, height: Fill
        flow: Down

        list = <PortalList> {
            width: Fill, height: Fill
            flow: Down,
            spacing: (SPACE_0)

            image_post = <ImagePost> {}
            text_post = <TextPost> {}
            hero = <Hero> {}
        }
    }
}

#[derive(Live, Widget)]
pub struct MomentList {
    #[deref]
    view: View,
    
    #[rust]
    moment_entries: Vec<MomentEntry>,
}

impl LiveHook for MomentList {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        let entries: Vec<MomentEntry> = vec![
            MomentEntry {
                username: "John Doe".to_string(),
                text: "無嶋可済野誰実玉全示餌強".to_string(),
            },
            MomentEntry {
                username: "Andrew Lin".to_string(),
                text: "俳権竹減活乗治外進梨詰鉄掲動覇予載".to_string(),
            },
            MomentEntry {
                username: "Chris Huxley".to_string(),
                text: "犯福併読併棋一御質慰".to_string(),
            },
            MomentEntry {
                username: "Adam Adler".to_string(),
                text: "体議速人幅触無持編聞組込".to_string(),
            },
            MomentEntry {
                username: "Eric Ford".to_string(),
                text: "体議速人幅触無持編聞組込 減活乗治外進".to_string(),
            }
        ];

        let repeated = iter::repeat(entries.clone()).take(10).flatten().collect();
        self.moment_entries = repeated;
    }
}

impl Widget for MomentList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let moment_entries_count = self.moment_entries.len();

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step(){
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, moment_entries_count + 1);
                while let Some(item_id) = list.next_visible_item(cx) {
                    let template = match item_id {
                        0 => id!(hero),
                        x if x % 2 == 0 => id!(text_post),
                        _ => id!(image_post),
                    };
    
                    let item = list.item(cx, item_id, template[0]).unwrap();
    
                    if item_id >= 1 && item_id < moment_entries_count + 1 {
                        let post = &self.moment_entries[item_id as usize - 1]; // offset by 1 to account for the hero
                        item.label(id!(content.username))
                            .set_text(&post.username);
                        item.label(id!(content.text)).set_text(&post.text);
                    }

                    item.draw_all(cx, &mut Scope::empty());
                }
            }
        }

        DrawStep::done()
    }
}

#[derive(Clone)]
struct MomentEntry {
    username: String,
    text: String,
}
